pub use cfg_if::cfg_if;
pub use flutter_rust_bridge_macros::{frb, wasm32};
pub use handler::{FfiCallMode, Handler, WrapInfo};
pub use rust2dart::StreamSink;
cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        pub use wasm_bindgen;
        pub use wasm_bindgen::prelude::*;
        mod ffi;
        pub use ffi::*;
    } else {
        pub use allo_isolate::ZeroCopyBuffer;
    }
}

pub mod handler;
pub mod rust2dart;
pub mod support;

/// Use this struct in return type of your function, in order to tell the code generator
/// the function should return synchronously. Otherwise, it is by default asynchronously.
pub struct SyncReturn<T>(pub T);
