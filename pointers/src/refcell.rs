use crate::cell::Cell;
use std::cell::UnsafeCell;
use std::ops::{Deref, DerefMut};

pub struct RefCell<T> {
    value: UnsafeCell<T>,
    reference: Cell<isize>,
}

impl<T> RefCell<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
            reference: Cell::new(0),
        }
    }

    pub fn borrow(&self) -> Option<Ref<'_, T>> {
        let reference = self.reference.get();
        if reference >= 0 {
            self.reference.set(reference + 1);
            Some(Ref { refcell: self })
        } else {
            None
        }
    }

    pub fn borrow_mut(&self) -> Option<RefMut<'_, T>> {
        let reference = self.reference.get();
        if reference == 0 {
            self.reference.set(reference - 1);
            Some(RefMut { refcell: self })
        } else {
            None
        }
    }
}

pub struct Ref<'refcell, T> {
    refcell: &'refcell RefCell<T>,
}

impl<T> Drop for Ref<'_, T> {
    fn drop(&mut self) {
        let reference = self.refcell.reference.get();
        self.refcell.reference.set(reference - 1);
    }
}

impl<T> Deref for Ref<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // SAFETY: no exclusive references have been given out since reference < 0.
        unsafe { &*self.refcell.value.get() }
    }
}

pub struct RefMut<'refcell, T> {
    refcell: &'refcell RefCell<T>,
}

impl<T> Drop for RefMut<'_, T> {
    fn drop(&mut self) {
        self.refcell.reference.set(0);
    }
}

impl<T> Deref for RefMut<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // SAFETY: no exclusive references have been given out since reference < 0.
        unsafe { &*self.refcell.value.get() }
    }
}

impl<T> DerefMut for RefMut<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: no other references have been given out since reference < 0 or reference > 0.
        unsafe { &mut *self.refcell.value.get() }
    }
}
