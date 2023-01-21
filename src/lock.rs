#![cfg_attr(debug_assertions, allow(dead_code, unused_variables,))]
use std::ops::{Deref, DerefMut};

#[cfg(all(feature = "sync", not(feature = "parking_lot")))]
pub struct Lock<T>(std::sync::Mutex<T>);

#[cfg(all(feature = "sync", not(feature = "parking_lot")))]
impl<T> Lock<T> {
    pub const fn new(value: T) -> Self {
        Self(std::sync::Mutex::new(value))
    }
    pub fn borrow(&self) -> impl Deref<Target = T> + '_ {
        self.0.lock().unwrap()
    }
    pub fn borrow_mut(&self) -> impl DerefMut<Target = T> + '_ {
        self.0.lock().unwrap()
    }
}

#[cfg(all(feature = "sync", feature = "parking_lot"))]
pub struct Lock<T>(parking_lot::Mutex<T>);

#[cfg(all(feature = "sync", feature = "parking_lot"))]
impl<T> Lock<T> {
    pub const fn new(value: T) -> Self {
        Self(parking_lot::Mutex::new(value))
    }
    pub fn borrow(&self) -> impl Deref<Target = T> + '_ {
        self.0.lock()
    }
    pub fn borrow_mut(&self) -> impl DerefMut<Target = T> + '_ {
        self.0.lock()
    }
}

#[cfg(not(feature = "sync"))]
pub struct Lock<T>(std::cell::RefCell<T>);

#[cfg(not(feature = "sync"))]
impl<T> Lock<T> {
    pub const fn new(value: T) -> Self {
        Self(std::cell::RefCell::new(value))
    }
    pub fn borrow(&self) -> impl Deref<Target = T> + '_ {
        self.0.borrow()
    }
    pub fn borrow_mut(&self) -> impl DerefMut<Target = T> + '_ {
        self.0.borrow_mut()
    }
}
