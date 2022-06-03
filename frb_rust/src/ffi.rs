use std::iter::FromIterator;

use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;

/// See [`allo_isolate::ffi::DartCObject`]
pub type DartCObject = wasm_bindgen::JsValue;

/// See [`allo_isolate::IntoDart`]
pub trait IntoDart {
    fn into_dart(self) -> DartCObject;
}

/// See [`allo_isolate::IntoDartExceptPrimitive`]
pub trait IntoDartExceptPrimitive {}
impl IntoDartExceptPrimitive for DartCObject {}

macro_rules! forward_impl {
    ($( $ty:ty ),*) => ($(
        impl IntoDart for $ty {
            #[inline] fn into_dart(self) -> DartCObject { self.into() }
        }
    )*);
}

forward_impl! {
    i8, u8, i16, u16, i32, u32, i64, u64, isize, usize, f32, f64,
    &'static str, String, bool, DartCObject
}

impl IntoDart for Vec<u8> {
    fn into_dart(self) -> DartCObject {
        js_sys::Uint8Array::from(&self[..]).into()
    }
}

impl<T: IntoDart + IntoDartExceptPrimitive> IntoDart for Vec<T> {
    fn into_dart(self) -> DartCObject {
        js_sys::Array::from_iter(self.into_iter().map(T::into_dart)).into()
    }
}

impl<T: IntoDart> IntoDart for Option<T> {
    fn into_dart(self) -> DartCObject {
        self.map(IntoDart::into_dart)
            .unwrap_or_else(wasm_bindgen::JsValue::null)
    }
}

impl IntoDart for () {
    fn into_dart(self) -> DartCObject {
        wasm_bindgen::JsValue::null()
    }
}

#[wasm_bindgen]
extern "C" {
    fn dart_post_js_object(port: i64, msg: DartCObject) -> bool;
}

#[derive(Clone, Copy)]
pub struct Isolate {
    port: i64,
}

impl Isolate {
    pub fn new(port: i64) -> Self {
        Isolate { port }
    }
    pub fn post(&self, mes: impl IntoDart) -> bool {
        dart_post_js_object(self.port, mes.into_dart())
    }
}

pub struct ZeroCopyBuffer<T>(pub T);

impl IntoDart for ZeroCopyBuffer<Vec<u8>> {
    fn into_dart(self) -> DartCObject {
        let mut buf = self.0.into_boxed_slice();
        let len = buf.len();
        let ptr = buf.as_mut_ptr();
        core::mem::forget(buf);
        unsafe { Uint8Array::view_mut_raw(ptr, len).into() }
    }
}
