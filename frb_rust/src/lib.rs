pub use allo_isolate::ZeroCopyBuffer;

pub use flutter_rust_bridge_macros::{frb, wasm32};
pub use handler::{FfiCallMode, Handler, WrapInfo};
pub use rust2dart::StreamSink;
#[wasm32]
pub use wasm_bindgen;
#[wasm32]
pub use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
mod ffi;
pub mod handler;
pub mod rust2dart;
pub mod support;

/// Use this struct in return type of your function, in order to tell the code generator
/// the function should return synchronously. Otherwise, it is by default asynchronously.
pub struct SyncReturn<T>(pub T);
