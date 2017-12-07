//! Cell allowing the inner value to be consumed without a mutable reference.
//!
//! In order to maintain safety, it is not possible to get a reference to the
//! inner value.

use std::fmt;
use std::cell::UnsafeCell;

/// Cell allowing the inner value to be consumed without a mutable reference.
pub struct Take<T> {
    val: UnsafeCell<Option<T>>,
}

impl<T> Take<T> {
    /// Create and return a new `Take` value containing the given inner value.
    pub fn new(val: T) -> Take<T> {
        Take { val: UnsafeCell::new(Some(val)) }
    }

    /// Consume and return the inner value.
    ///
    /// # Panics
    ///
    /// If the inner value has already been consumed, the call will panic.
    pub fn take(&self) -> T {
        unsafe { (*self.val.get()).take() }.expect("value already consumed")
    }

    // This function cannot be public as no public API functions should be able
    // to return a reference to the inner value. It is only safe to grab this
    // reference if `take` is not called while the reference is held.
    fn inner(&self) -> &Option<T> {
        unsafe { &*self.val.get() }
    }
}

impl<T: fmt::Debug> fmt::Debug for Take<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Take")
            .field("val", self.inner())
            .finish()
    }
}
