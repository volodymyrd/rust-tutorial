use crate::cell::Cell;
use std::fmt;
use std::marker::PhantomData;
use std::ops::Deref;
use std::ptr::NonNull;

struct RcInner<T> {
    ref_count: Cell<usize>,
    value: T,
}

pub struct Rc<T> {
    ptr: NonNull<RcInner<T>>,
    phantom: PhantomData<RcInner<T>>,
}

impl<T> Rc<T> {
    pub fn new(value: T) -> Self {
        Self {
            ptr: Box::leak(Box::new(RcInner {
                ref_count: Cell::new(1),
                value,
            }))
            .into(),
            phantom: PhantomData,
        }
    }
}

impl<T> Clone for Rc<T> {
    fn clone(&self) -> Self {
        let inner = unsafe { self.ptr.as_ref() };
        inner.ref_count.set(inner.ref_count.get() + 1);
        Rc {
            ptr: self.ptr,
            phantom: PhantomData,
        }
    }
}

impl<T> Deref for Rc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // SAFETY: self.ptr is a Box that is only deallocated when the last Rc goes away.
        &unsafe { self.ptr.as_ref() }.value
    }
}

impl<T> Drop for Rc<T> {
    fn drop(&mut self) {
        let inner = unsafe { self.ptr.as_ref() };
        let rc = inner.ref_count.get();
        if rc == 1 {
            let _ = unsafe { Box::from_raw(self.ptr.as_ptr()) };
        } else {
            inner.ref_count.set(rc - 1);
        }
    }
}

// Add Debug impl for easier printing in tests
impl<T: fmt::Debug> fmt::Debug for Rc<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Rc")
            .field("value", &**self) // Dereference to show the inner value
            .field(
                "strong_count",
                &unsafe { self.ptr.as_ref() }.ref_count.get(),
            )
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_rc() {
        let five = Rc::new(5);

        assert_eq!(*five, 5);
        assert_eq!(unsafe { five.ptr.as_ref() }.ref_count.get(), 1);

        let five_clone = five.clone();
        assert_eq!(*five_clone, 5);
        assert_eq!(unsafe { five.ptr.as_ref() }.ref_count.get(), 2);
        assert_eq!(unsafe { five_clone.ptr.as_ref() }.ref_count.get(), 2);

        drop(five);
        assert_eq!(unsafe { five_clone.ptr.as_ref() }.ref_count.get(), 1);

        drop(five_clone);
        // Memory should be deallocated here.
        // We can't safely check the ref count after the last drop,
        // but the test should pass without memory errors.
    }
}
