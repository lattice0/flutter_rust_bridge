//! Functions that support auto-generated Rust code.
//! These functions are *not* meant to be used by humans directly.
#![doc(hidden)]

use crate::DartSafe;
use std::mem;
use std::sync::Arc;

pub use allo_isolate::ffi::DartCObject;
pub use allo_isolate::{IntoDart, IntoDartExceptPrimitive};
pub use lazy_static::lazy_static;

pub use crate::handler::DefaultHandler;
use crate::Opaque;

// ref https://stackoverflow.com/questions/39224904/how-to-expose-a-rust-vect-to-ffi
pub fn new_leak_vec_ptr<T: Clone>(fill: T, length: i32) -> *mut T {
    into_leak_vec_ptr(vec![fill; length as usize]).0
}

pub fn into_leak_vec_ptr<T: Clone>(mut v: Vec<T>) -> (*mut T, i32) {
    v.shrink_to_fit();
    assert!(v.len() == v.capacity());
    let ptr = v.as_mut_ptr();
    let len = v.len() as i32;
    mem::forget(v);
    (ptr, len)
}

/// # Safety
/// Use it in pair with [new_leak_vec_ptr].
pub unsafe fn vec_from_leak_ptr<T>(ptr: *mut T, len: i32) -> Vec<T> {
    Vec::from_raw_parts(ptr, len as usize, len as usize)
}

// ref: doc of [Box::into_raw]
pub fn new_leak_box_ptr<T>(t: T) -> *mut T {
    let x: Box<T> = Box::new(t);
    Box::into_raw(x)
}

/// # Safety
/// Use it in pair with [new_leak_box_ptr].
pub unsafe fn box_from_leak_ptr<T>(ptr: *mut T) -> Box<T> {
    Box::from_raw(ptr)
}

/// NOTE for maintainer: Please keep this struct in sync with `DUMMY_WIRE_CODE_FOR_BINDGEN`
/// in the code generator
#[repr(C)]
pub struct WireSyncReturnStruct {
    pub ptr: *mut u8,
    pub len: i32,
    pub success: bool,
}
/// # Safety
/// This function should never be called manually.
/// Retrieving an opaque pointer from Dart is an implementation detail,
/// so this function is not guaranteed to be API-stable.
pub unsafe fn opaque_from_dart<T: DartSafe>(ptr: *const T) -> Opaque<T> {
    // The raw pointer is the same one created from Arc::into_raw,
    // owned and artificially incremented by Dart.
    Opaque {
        ptr: (!ptr.is_null()).then(|| unsafe { Arc::from_raw(ptr) }),
    }
}
