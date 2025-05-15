use std::cell::UnsafeCell;

pub struct Cell<T> {
    value: UnsafeCell<T>,
}
// already implemented by UnsafeCell
// impl<T> !Sync for Cell<T> {}

impl<T> Cell<T> {
    pub fn new(value: T) -> Self {
        Cell {
            value: UnsafeCell::new(value),
        }
    }

    pub fn get(&self) -> T
    where
        T: Copy,
    {
        // SAFETY: only current thread can mutate (because of !Sync).
        unsafe { *self.value.get() }
    }

    pub fn set(&self, value: T) {
        // SAFETY: no one else is concurrently mutating self.value (because of !Sync).
        // SAFETY: new expose any reference out, so any of them invalidating.
        unsafe { *self.value.get() = value };
    }
}
