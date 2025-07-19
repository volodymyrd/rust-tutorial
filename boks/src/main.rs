#![feature(dropck_eyepatch)]
use std::ops::{Deref, DerefMut};

struct Boks<T> {
    p: *mut T,
}

unsafe impl<#[may_dangle] T> Drop for Boks<T> {
    fn drop(&mut self) {
        unsafe {
            let _ = Box::from_raw(self.p);
        }
    }
}

impl<T> Boks<T> {
    fn new(t: T) -> Self {
        Boks {
            p: Box::into_raw(Box::new(t)),
        }
    }
}

impl<T> Deref for Boks<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.p }
    }
}

impl<T> DerefMut for Boks<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.p }
    }
}

fn main() {
    let x = "42";
    let b = Boks::new(x);
    println!("{}", *b);

    let mut y = 42;
    let b = Boks::new(&mut y);
    println!("{}", y);
}
