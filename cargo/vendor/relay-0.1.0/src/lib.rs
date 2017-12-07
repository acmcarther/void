//! # relay
//!
//! A light-weight channel using `Future`. A relay channel does not implement
//! `Send`, and so is not meant for synchronizing between threads. Instead,
//! its used to send message between tasks that live in the same thread.
//!
//! It is similar to the `oneshot` channel in the `futures` crate, but since
//! it is not meant for sending across threads, it performs about twice as
//! fast.
//!
//! ## Example
//!
//! ```rust
//! # extern crate futures;
//! # extern crate relay;
//! # use futures::Future;
//! # fn main() {
//! let (tx, rx) = relay::channel();
//! tx.complete("foo");
//! assert_eq!(rx.wait().unwrap(), "foo");
//! # }
//! ```
#![deny(warnings)]
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
extern crate futures;

use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

use futures::{Future, Poll, Async};
use futures::task::{self, Task};

/// Create a new channel to send a message.
pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let inner = Rc::new(RefCell::new(Inner {
        value: None,
        complete: false,
        tx_task: None,
        rx_task: None,
    }));
    let tx = Sender {
        inner: inner.clone(),
    };
    let rx = Receiver {
        inner: inner,
    };
    (tx, rx)
}

/// The Sender portion of a channel.
pub struct Sender<T> {
    inner: Rc<RefCell<Inner<T>>>,
}

impl<T> Sender<T> {
    /// Sends the message to the `Receiver`.
    pub fn complete(self, val: T) {
        let mut borrow = self.inner.borrow_mut();
        borrow.value = Some(val);
    }

    /// Returns true if the `Receiver` has been dropped.
    pub fn is_canceled(&self) -> bool {
        self.inner.borrow().complete
    }

    /// Creates a `Future` that waits until someone is waiting on the `Receiver`.
    pub fn waiting(self) -> Waiting<T> {
        Waiting {
            tx: Some(self),
        }
    }
}

impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        let rx_task = {
            let mut borrow = self.inner.borrow_mut();
            borrow.complete = true;
            borrow.tx_task.take();
            borrow.rx_task.take()
        };
        if let Some(task) = rx_task {
            task.unpark();
        }
    }
}

impl<T> fmt::Debug for Sender<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad("Sender")
    }
}

/// The receiver end of the channel.
///
/// The Receiver is a `Future` that resolves to the sent message.
pub struct Receiver<T> {
    inner: Rc<RefCell<Inner<T>>>,
}

impl<T> Receiver<T> {
    /// Returns true if the `Sender` was dropped without sending a message.
    pub fn is_canceled(&self) -> bool {
        let borrow = self.inner.borrow();
        borrow.complete && borrow.value.is_none()
    }
}

impl<T> Future for Receiver<T> {
    type Item = T;
    type Error = Canceled;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        let mut borrow = self.inner.borrow_mut();
        if let Some(val) = borrow.value.take() {
            Ok(Async::Ready(val))
        } else if borrow.complete {
            Err(Canceled)
        } else {
            borrow.rx_task = Some(task::park());
            if let Some(task) = borrow.tx_task.take() {
                task.unpark();
            }
            Ok(Async::NotReady)
        }
    }
}

impl<T> Drop for Receiver<T> {
    fn drop(&mut self) {
        let tx_task = {
            let mut borrow = self.inner.borrow_mut();
            borrow.complete = true;
            borrow.rx_task.take();
            borrow.tx_task.take()
        };
        if let Some(task) = tx_task {
            task.unpark();
        }
    }
}

impl<T> fmt::Debug for Receiver<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad("Receiver")
    }
}

/// A `Future` waiting for interest to be registered on the `Receiver`.
pub struct Waiting<T> {
    tx: Option<Sender<T>>,
}

impl<T> Future for Waiting<T> {
    type Item = Sender<T>;
    type Error = Canceled;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        if self.tx.as_ref().unwrap().is_canceled() {
            Err(Canceled)
        } else if self.tx.as_ref().unwrap().inner.borrow().rx_task.is_some() {
            Ok(Async::Ready(self.tx.take().unwrap()))
        } else {
            self.tx.as_ref().unwrap().inner.borrow_mut().tx_task = Some(task::park());
            Ok(Async::NotReady)
        }
    }
}

impl<T> fmt::Debug for Waiting<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad("Waiting")
    }
}

/// Represents that the `Sender` dropped before sending a message.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Canceled;

struct Inner<T> {
    value: Option<T>,
    complete: bool,
    tx_task: Option<Task>,
    rx_task: Option<Task>,
}

#[cfg(test)]
mod tests {
    use futures::Future;
    use super::channel;

    #[test]
    fn test_smoke() {
        let (tx, rx) = channel();
        tx.complete(33);
        assert_eq!(rx.wait().unwrap(), 33);
    }

    #[test]
    fn test_canceled() {
        let (_, rx) = channel::<()>();
        assert_eq!(rx.wait().unwrap_err(), super::Canceled);
    }

    #[test]
    fn test_is_canceled() {
        let (tx, _) = channel::<()>();
        assert!(tx.is_canceled());

        let (_, rx) = channel::<()>();
        assert!(rx.is_canceled());

        let (tx, rx) = channel::<()>();
        assert!(!tx.is_canceled());
        assert!(!rx.is_canceled());

        tx.complete(());
        assert!(!rx.is_canceled());
    }

    #[test]
    fn test_tx_complete_rx_unparked() {
        let (tx, rx) = channel();

        let res = rx.join(::futures::lazy(move || {
            tx.complete(55);
            Ok(11)
        }));
        assert_eq!(res.wait().unwrap(), (55, 11));
    }

    #[test]
    fn test_tx_dropped_rx_unparked() {
        let (tx, rx) = channel::<i32>();

        let res = rx.join(::futures::lazy(move || {
            let _tx = tx;
            Ok(11)
        }));
        assert_eq!(res.wait().unwrap_err(), super::Canceled);
    }

    #[test]
    fn test_waiting_unparked() {
        let (tx, rx) = channel::<i32>();

        let res = tx.waiting().join(::futures::lazy(move || {
            let mut rx = rx;
            let _ = rx.poll(); // unpark
            Ok(rx)
        })).and_then(|(tx, rx)| {
            tx.complete(5);
            rx
        });
        assert_eq!(res.wait().unwrap(), 5);
    }

    #[test]
    fn test_waiting_canceled() {
        let (tx, rx) = channel::<i32>();

        let res = tx.waiting().join(::futures::lazy(move || {
            let _rx = rx;
            Ok(())
        }));
        assert_eq!(res.wait().unwrap_err(), super::Canceled);
    }
}
