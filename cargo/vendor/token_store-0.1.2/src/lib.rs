//! # Token Store
//!
//! This crate provides a small abstraction of a type allowing
//! you to stroe values of arbitrary types and retrieving them
//! using tokens values that are cheap to move around and clone.
//!
//! The typical use-case is a data store shared by large portions
//! of an application requiring sequential access to parts of this
//! store, but while not caring nor being able to know what else
//! is stored in it.
//!
//! ## How to use it
//!
//! ```
//! # extern crate token_store;
//! use token_store::Store;
//!
//! # fn main(){
//! // create a store
//! let mut store = Store::new();
//!
//! // insert some things in it, you are given tokens
//! let token1 = store.insert(42);
//! // you can store any type as log as it is `Any + 'static`
//! let token2 = store.insert(String::from("I like trains"));
//! // the tokens keep the information of the store type,
//! // as such you don't need any annotation to retrieve a value:
//! store.get_mut(&token2).push_str(", and cars too!");
//! # }
//! ```
//!
//! The retrieved tokens can be cloned and shared as you like between various
//! parts of your code.
//!
//! Note however that, as it is possible to store `!Send` types in the `token_store`,
//! neither the store nor its tokens can be shared accross threads.
//!
//! ## Value scopes and genericity
//!
//! It is also possible to access simultaneously several values of the store using
//! a scoped access:
//!
//! ```
//! # extern crate token_store;
//! # use token_store::{Store, StoreProxy};
//! # fn main() {
//! let mut store = Store::new();
//! let token = store.insert(42);
//! store.with_value(&token, |proxy, value| {
//!     // Here, proxy is a `StoreProxy`, it allows you to to all you want with the
//!     // store, as long as you do not try to access again the value guarded by
//!     // the token provided to `with_value`.
//!     // Also, value is a mutable reference to the value guarded by this token.
//!
//!     // You can nest calls to `with_value` to access several values simultaneously
//!     let token2 = proxy.insert(String::new());
//!     proxy.with_value(&token2, |proxy, value2| {
//!         // Here you can access value, value2, as well as a proxy tracking that
//!         // both values are borrowed
//!     });
//! });
//! # }
//! ```
//!
//! Two implementations of the `From` trait are also provided, allowing you to convert
//! both a `&mut Store` and a `&mut StoreProxy` into a `StoreProxy`. This is to help
//! generic code like this:
//!
//! ```
//! # extern crate token_store;
//! # use token_store::{Store, StoreProxy};
//! # fn main() {}
//! fn do_stuff<'store, S: Into<StoreProxy<'store>>>(s: S) {
//!     let proxy = s.into();
//!     // we now have a store proxy, and can do our stuff with it
//!     // and the caller can call us directly with a `&mut Store` or
//!     // from within a value scope.
//! }
//! ```
#![warn(missing_docs)]

use std::any::Any;
use std::borrow::Cow;
use std::cell::Cell;
use std::marker::PhantomData;
use std::rc::Rc;

/// A token store
///
/// This struct allows you to store various values in a store
/// and access them back using the provided tokens.
pub struct Store {
    values: Vec<Option<(Box<Any>, Rc<Cell<bool>>)>>,
}

/// A token for accessing the store contents
pub struct Token<V> {
    id: usize,
    live: Rc<Cell<bool>>,
    _type: PhantomData<V>,
}

impl<V> Token<V> {
    /// Check wether this token is still valid
    ///
    /// If it is not, trying to use it to access contents
    /// will panic.
    pub fn valid(&self) -> bool {
        self.live.get()
    }
}

impl<V> Clone for Token<V> {
    fn clone(&self) -> Token<V> {
        Token {
            id: self.id,
            live: self.live.clone(),
            _type: PhantomData,
        }
    }
}

impl Store {
    /// Create a new store
    pub fn new() -> Store {
        Store { values: Vec::new() }
    }

    /// Insert a new value in this store
    ///
    /// Returns a clonable token that you can later use to access this
    /// value.
    pub fn insert<V: Any + 'static>(&mut self, value: V) -> Token<V> {
        let boxed = Box::new(value) as Box<Any>;
        let live = Rc::new(Cell::new(true));
        {
            // artificial scope to make the borrow checker happy
            let empty_slot = self.values
                .iter_mut()
                .enumerate()
                .find(|&(_, ref s)| s.is_none());
            if let Some((id, slot)) = empty_slot {
                *slot = Some((boxed, live.clone()));
                return Token {
                    id: id,
                    live: live,
                    _type: PhantomData,
                };
            }
        }
        self.values.push(Some((boxed, live.clone())));
        Token {
            id: self.values.len() - 1,
            live: live,
            _type: PhantomData,
        }
    }

    /// Access value previously inserted in this store
    ///
    /// Panics if the provided token corresponds to a value that was removed.
    pub fn get<V: Any + 'static>(&self, token: &Token<V>) -> &V {
        if !token.live.get() {
            panic!("Attempted to access a state value that was already removed!");
        }
        self.values[token.id]
            .as_ref()
            .and_then(|t| t.0.downcast_ref::<V>())
            .unwrap()
    }

    /// Mutably access value previously inserted in this store
    ///
    /// Panics if the provided token corresponds to a value that was removed.
    pub fn get_mut<V: Any + 'static>(&mut self, token: &Token<V>) -> &mut V {
        if !token.live.get() {
            panic!("Attempted to access a state value that was already removed!");
        }
        self.values[token.id]
            .as_mut()
            .and_then(|t| t.0.downcast_mut::<V>())
            .unwrap()
    }

    /// Remove a value previously inserted in this store
    ///
    /// Panics if the provided token corresponds to a value that was already
    /// removed.
    pub fn remove<V: Any + 'static>(&mut self, token: Token<V>) -> V {
        if !token.live.get() {
            panic!("Attempted to remove a state value that was already removed!");
        }
        let (boxed, live) = self.values[token.id].take().unwrap();
        live.set(false);
        *boxed.downcast().unwrap()
    }

    /// Create a sub-scope with access to a value
    ///
    /// In the closure you provide, the value represented by `token`
    /// will be available as an argument, as well as a `StoreProxy`,
    /// which allows you to manipulate the other values of the store
    /// while this one is mutably borrowed.
    ///
    /// Attempting to access again the same value from its token from
    /// within this closure is forbidden, and attempting to do so will
    /// result in a panic.
    ///
    /// The `StoreProxy` provides the same access methods as the `Store`,
    /// including `with_value`, allowing you to create nested sub-scopes
    /// accessing multiple store values at the same time.
    pub fn with_value<V: Any + 'static, T, F>(&mut self, token: &Token<V>, f: F) -> T
    where
        F: FnOnce(&mut StoreProxy, &mut V) -> T,
    {
        self.as_proxy().with_value(token, f)
    }

    /// See this `Store` as a `StoreProxy` with no ongoing borrow
    ///
    /// This can be usefull for code requiering access to a store,
    /// but wanting to be generic over being called from a value
    /// scope or not.
    ///
    /// You can also use the `From` and `Into` traits to perform
    /// this conversion.
    pub fn as_proxy<'a>(&'a mut self) -> StoreProxy<'a> {
        StoreProxy {
            store: self,
            borrowed: Cow::Owned(Vec::new()),
        }
    }
}

impl<'a> ::std::convert::From<&'a mut Store> for StoreProxy<'a> {
    fn from(store: &'a mut Store) -> StoreProxy<'a> {
        store.as_proxy()
    }
}

impl<'a, 'b> ::std::convert::From<&'a mut StoreProxy<'b>> for StoreProxy<'a>
where
    'b: 'a,
{
    fn from(proxy: &'a mut StoreProxy<'b>) -> StoreProxy<'a> {
        StoreProxy {
            store: proxy.store,
            borrowed: proxy.borrowed.clone(),
        }
    }
}

/// A Proxy representing a `Store` with ongoing borrows
///
/// This struct represents a handle to a store from which
/// some values are already mutably borrowed, and as such
/// cannot be touched.
///
/// See `Store::with_value` for detailed explanation of its
/// use.
pub struct StoreProxy<'store> {
    store: &'store mut Store,
    borrowed: Cow<'store, [usize]>,
}

impl<'store> StoreProxy<'store> {
    /// Insert a new value in the proxified store
    ///
    /// Returns a clonable token that you can later use to access this
    /// value.
    pub fn insert<V: Any + 'static>(&mut self, value: V) -> Token<V> {
        self.store.insert(value)
    }

    /// Access value previously inserted in the proxified store
    ///
    /// Panics if the provided token corresponds to a value that was removed, or
    /// if this value is already borrowed.
    pub fn get<V: Any + 'static>(&self, token: &Token<V>) -> &V {
        if self.borrowed.contains(&token.id) {
            panic!("Attempted to borrow twice the same value from the Store!");
        }
        self.store.get(token)
    }

    /// Mutably access value previously inserted in the proxified store
    ///
    /// Panics if the provided token corresponds to a value that was removed, or
    /// if this value is already borrowed.
    pub fn get_mut<V: Any + 'static>(&mut self, token: &Token<V>) -> &mut V {
        if self.borrowed.contains(&token.id) {
            panic!("Attempted to borrow twice the same value from the Store!");
        }
        self.store.get_mut(token)
    }

    /// Remove a value previously inserted in the proxified store
    ///
    /// Panics if the provided token corresponds to a value that was already
    /// removed, or if this value is already borrowed.
    pub fn remove<V: Any + 'static>(&mut self, token: Token<V>) -> V {
        if self.borrowed.contains(&token.id) {
            panic!("Attempted to remove a value from the Store while it was borrowed!");
        }
        self.store.remove(token)
    }

    /// Create a sub-scope with access to a value
    ///
    /// Panics if the provided token corresponds to a value that was removed, or
    /// if this value is already borrowed.
    ///
    /// See `Store::with_value` for full documentation.
    pub fn with_value<V: Any + 'static, T, F>(&mut self, token: &Token<V>, f: F) -> T
    where
        F: FnOnce(&mut StoreProxy, &mut V) -> T,
    {
        if self.borrowed.contains(&token.id) {
            panic!("Attempted to borrow twice the same value from the Store!");
        }
        let value_ptr = { self.store.get_mut(token) as *mut V };
        let value = unsafe { &mut *value_ptr };
        let mut deeper_proxy = StoreProxy {
            store: &mut *self.store,
            borrowed: {
                let mut my_borrowed = self.borrowed.clone().into_owned();
                my_borrowed.push(token.id);
                Cow::Owned(my_borrowed)
            },
        };
        f(&mut deeper_proxy, value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn insert_and_retrieve() {
        let mut store = Store::new();
        let token1 = store.insert(42);
        let token2 = store.insert("I like trains".to_owned());
        assert_eq!(*store.get(&token1), 42);
        assert_eq!(store.get(&token2), "I like trains");
    }

    #[test]
    fn mutate() {
        let mut store = Store::new();
        let token = store.insert(42);
        {
            let v = store.get_mut(&token);
            *v += 5;
        }
        assert_eq!(*store.get(&token), 47);
    }

    #[test]
    #[should_panic]
    fn no_access_removed() {
        let mut store = Store::new();
        let token = store.insert(42);
        let token2 = token.clone();
        store.remove(token2);
        let _v = store.get(&token);
    }

    #[test]
    #[should_panic]
    fn no_mut_access_removed() {
        let mut store = Store::new();
        let token = store.insert(42);
        let token2 = token.clone();
        store.remove(token2);
        let _v = store.get_mut(&token);
    }

    #[test]
    #[should_panic]
    fn no_double_remove() {
        let mut store = Store::new();
        let token = store.insert(42);
        let token2 = token.clone();
        store.remove(token2);
        store.remove(token);
    }


    #[test]
    fn place_reuse() {
        let mut store = Store::new();
        let token = store.insert(42);
        store.remove(token);
        let token = store.insert("I like trains");
        assert_eq!(store.values.len(), 1);
        assert_eq!(*store.get(&token), "I like trains");
    }

    #[test]
    fn with_value_manipulate() {
        let mut store = Store::new();
        let token1 = store.insert("I like trains".to_owned());
        let token2 = store.insert(42);
        let len = store.with_value(&token1, |proxy, value1| {
            *proxy.get_mut(&token2) += 10;
            let token3 = proxy.with_value(&token2, |proxy, value2| {
                *value2 *= 2;
                proxy.insert(*value2 as f32 + 0.5)
            });
            let number = proxy.remove(token2);
            value1.push_str(&format!(": {} = {}", number, proxy.get(&token3)));
            value1.len()
        });
        assert_eq!(len, 26);
        assert_eq!(store.get(&token1), "I like trains: 104 = 104.5");
    }

    #[test]
    #[should_panic]
    fn no_double_with_value() {
        let mut store = Store::new();
        let token = store.insert(42);
        store.with_value(&token, |proxy, _| {
            proxy.with_value(&token, |_, _| {});
        });
    }

    #[test]
    #[should_panic]
    fn no_alias_get_and_with_value() {
        let mut store = Store::new();
        let token = store.insert(42);
        store.with_value(&token, |proxy, _| {
            let _v = proxy.get(&token);
        });
    }

    #[test]
    #[should_panic]
    fn no_alias_get_mut_and_with_value() {
        let mut store = Store::new();
        let token = store.insert(42);
        store.with_value(&token, |proxy, _| {
            let _v = proxy.get_mut(&token);
        });
    }

    #[test]
    #[should_panic]
    fn no_alias_remove_and_with_value() {
        let mut store = Store::new();
        let token = store.insert(42);
        store.with_value(&token, |proxy, _| {
            let _v = proxy.remove(token.clone());
        });
    }

    #[test]
    fn generic_into_store_proxy() {
        fn insert_42<'a, S: Into<StoreProxy<'a>>>(s: S) -> Token<i32> {
            let mut proxy = s.into();
            proxy.insert(42)
        }

        let mut store = Store::new();
        let token1 = insert_42(&mut store);
        let token2 = store.with_value(&token1, |proxy, _| insert_42(proxy));
        assert_eq!(*store.get(&token1), 42);
        assert_eq!(*store.get(&token2), 42);
    }

    #[test]
    fn token_validity() {
        let mut store = Store::new();
        let token = store.insert(42);
        assert!(token.valid());
        store.remove(token.clone());
        assert!(!token.valid());
    }
}
