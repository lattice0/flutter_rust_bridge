use std::iter::FromIterator;

use wasm_bindgen::prelude::*;

pub type DartCObject = wasm_bindgen::JsValue;

pub trait IntoDart {
    fn into_dart(self) -> DartCObject;
}

macro_rules! forward_impl {
    ($( $ty:ty ),*) => ($(
        impl IntoDart for $ty {
            #[inline] fn into_dart(self) -> DartCObject { self.into() }
        }
    )*);
}

forward_impl! {
    i8, u8, i16, u16, i32, u32, i64, u64, isize, usize,
    &'static str, String, bool
}

impl IntoDart for Vec<DartCObject> {
    fn into_dart(self) -> DartCObject {
        js_sys::Array::from_iter(self).into()
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
        wasm_bindgen::JsValue::undefined()
    }
}

#[wasm_bindgen]
extern "C" {
    fn dart_post_js_object(port: i64, msg: DartCObject) -> bool;
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub struct Isolate {
    port: i64,
}

#[wasm_bindgen]
impl Isolate {
    #[wasm_bindgen(constructor)]
    pub fn new(port: i64) -> Self {
        Isolate { port }
    }
}

impl Isolate {
    pub fn post(&self, mes: impl IntoDart) -> bool {
        dart_post_js_object(self.port, mes.into_dart())
    }
}
