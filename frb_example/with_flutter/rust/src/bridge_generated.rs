#![allow(
    non_camel_case_types,
    unused,
    clippy::redundant_closure,
    clippy::useless_conversion,
    clippy::unit_arg,
    non_snake_case
)]
// AUTO GENERATED FILE, DO NOT EDIT.
// Generated by `flutter_rust_bridge`.

use crate::api::*;
use flutter_rust_bridge::*;

// Section: imports

// Section: wire functions

#[no_mangle]
pub extern "C" fn wire_draw_mandelbrot(
    port_: i64,
    image_size: *mut wire_Size,
    zoom_point: *mut wire_Point,
    scale: f64,
    num_threads: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "draw_mandelbrot",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_image_size = image_size.wire2api();
            let api_zoom_point = zoom_point.wire2api();
            let api_scale = scale.wire2api();
            let api_num_threads = num_threads.wire2api();
            move |task_callback| {
                draw_mandelbrot(api_image_size, api_zoom_point, api_scale, api_num_threads)
            }
        },
    )
}

#[no_mangle]
pub extern "C" fn wire_passing_complex_structs(port_: i64, root: *mut wire_TreeNode) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "passing_complex_structs",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_root = root.wire2api();
            move |task_callback| Ok(passing_complex_structs(api_root))
        },
    )
}

#[no_mangle]
pub extern "C" fn wire_off_topic_memory_test_input_array(port_: i64, input: *mut wire_uint_8_list) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "off_topic_memory_test_input_array",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_input = input.wire2api();
            move |task_callback| Ok(off_topic_memory_test_input_array(api_input))
        },
    )
}

#[no_mangle]
pub extern "C" fn wire_off_topic_memory_test_output_zero_copy_buffer(port_: i64, len: i32) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "off_topic_memory_test_output_zero_copy_buffer",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_len = len.wire2api();
            move |task_callback| Ok(off_topic_memory_test_output_zero_copy_buffer(api_len))
        },
    )
}

#[no_mangle]
pub extern "C" fn wire_off_topic_memory_test_output_vec_u8(port_: i64, len: i32) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "off_topic_memory_test_output_vec_u8",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_len = len.wire2api();
            move |task_callback| Ok(off_topic_memory_test_output_vec_u8(api_len))
        },
    )
}

#[no_mangle]
pub extern "C" fn wire_off_topic_memory_test_input_vec_of_object(
    port_: i64,
    input: *mut wire_list_size,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "off_topic_memory_test_input_vec_of_object",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_input = input.wire2api();
            move |task_callback| Ok(off_topic_memory_test_input_vec_of_object(api_input))
        },
    )
}

#[no_mangle]
pub extern "C" fn wire_off_topic_memory_test_output_vec_of_object(port_: i64, len: i32) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "off_topic_memory_test_output_vec_of_object",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_len = len.wire2api();
            move |task_callback| Ok(off_topic_memory_test_output_vec_of_object(api_len))
        },
    )
}

#[no_mangle]
pub extern "C" fn wire_off_topic_memory_test_input_complex_struct(
    port_: i64,
    input: *mut wire_TreeNode,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "off_topic_memory_test_input_complex_struct",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_input = input.wire2api();
            move |task_callback| Ok(off_topic_memory_test_input_complex_struct(api_input))
        },
    )
}

#[no_mangle]
pub extern "C" fn wire_off_topic_memory_test_output_complex_struct(port_: i64, len: i32) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "off_topic_memory_test_output_complex_struct",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_len = len.wire2api();
            move |task_callback| Ok(off_topic_memory_test_output_complex_struct(api_len))
        },
    )
}

#[no_mangle]
pub extern "C" fn wire_off_topic_deliberately_return_error(port_: i64) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "off_topic_deliberately_return_error",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || move |task_callback| off_topic_deliberately_return_error(),
    )
}

#[no_mangle]
pub extern "C" fn wire_off_topic_deliberately_panic(port_: i64) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "off_topic_deliberately_panic",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || move |task_callback| Ok(off_topic_deliberately_panic()),
    )
}

// Section: wire structs

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_size {
    ptr: *mut wire_Size,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_tree_node {
    ptr: *mut wire_TreeNode,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_Point {
    x: f64,
    y: f64,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_Size {
    width: i32,
    height: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_TreeNode {
    name: *mut wire_uint_8_list,
    children: *mut wire_list_tree_node,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_uint_8_list {
    ptr: *mut u8,
    len: i32,
}

// Section: wrapper structs

// Section: static checks

// Section: allocate functions

#[no_mangle]
pub extern "C" fn new_box_autoadd_point() -> *mut wire_Point {
    support::new_leak_box_ptr(wire_Point::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_size() -> *mut wire_Size {
    support::new_leak_box_ptr(wire_Size::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_tree_node() -> *mut wire_TreeNode {
    support::new_leak_box_ptr(wire_TreeNode::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_list_size(len: i32) -> *mut wire_list_size {
    let wrap = wire_list_size {
        ptr: support::new_leak_vec_ptr(<wire_Size>::new_with_null_ptr(), len),
        len,
    };
    support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_list_tree_node(len: i32) -> *mut wire_list_tree_node {
    let wrap = wire_list_tree_node {
        ptr: support::new_leak_vec_ptr(<wire_TreeNode>::new_with_null_ptr(), len),
        len,
    };
    support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_uint_8_list(len: i32) -> *mut wire_uint_8_list {
    let ans = wire_uint_8_list {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(ans)
}

// Section: impl Wire2Api

pub trait Wire2Api<T> {
    fn wire2api(self) -> T;
}

impl<T, S> Wire2Api<Option<T>> for *mut S
where
    *mut S: Wire2Api<T>,
{
    #[inline]
    fn wire2api(self) -> Option<T> {
        if self.is_null() {
            None
        } else {
            Some(self.wire2api())
        }
    }
}

impl<T: Wire2Api<U>, U> Wire2Api<Option<U>> for Option<T> {
    #[inline]
    fn wire2api(self) -> Option<U> {
        self.map(Wire2Api::wire2api)
    }
}

impl Wire2Api<String> for *mut wire_uint_8_list {
    fn wire2api(self) -> String {
        let vec: Vec<u8> = self.wire2api();
        String::from_utf8_lossy(&vec).into_owned()
    }
}

impl Wire2Api<Point> for *mut wire_Point {
    fn wire2api(self) -> Point {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        (*wrap).wire2api().into()
    }
}

impl Wire2Api<Size> for *mut wire_Size {
    fn wire2api(self) -> Size {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        (*wrap).wire2api().into()
    }
}

impl Wire2Api<TreeNode> for *mut wire_TreeNode {
    fn wire2api(self) -> TreeNode {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        (*wrap).wire2api().into()
    }
}

impl Wire2Api<f64> for f64 {
    fn wire2api(self) -> f64 {
        self
    }
}

impl Wire2Api<i32> for i32 {
    fn wire2api(self) -> i32 {
        self
    }
}

impl Wire2Api<Vec<Size>> for *mut wire_list_size {
    fn wire2api(self) -> Vec<Size> {
        let vec = unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}

impl Wire2Api<Vec<TreeNode>> for *mut wire_list_tree_node {
    fn wire2api(self) -> Vec<TreeNode> {
        let vec = unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}

impl Wire2Api<Point> for wire_Point {
    fn wire2api(self) -> Point {
        Point {
            x: self.x.wire2api(),
            y: self.y.wire2api(),
        }
    }
}

impl Wire2Api<Size> for wire_Size {
    fn wire2api(self) -> Size {
        Size {
            width: self.width.wire2api(),
            height: self.height.wire2api(),
        }
    }
}

impl Wire2Api<TreeNode> for wire_TreeNode {
    fn wire2api(self) -> TreeNode {
        TreeNode {
            name: self.name.wire2api(),
            children: self.children.wire2api(),
        }
    }
}

impl Wire2Api<u8> for u8 {
    fn wire2api(self) -> u8 {
        self
    }
}

impl Wire2Api<Vec<u8>> for *mut wire_uint_8_list {
    fn wire2api(self) -> Vec<u8> {
        unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}

// Section: impl NewWithNullPtr

pub trait NewWithNullPtr {
    fn new_with_null_ptr() -> Self;
}

impl<T> NewWithNullPtr for *mut T {
    fn new_with_null_ptr() -> Self {
        std::ptr::null_mut()
    }
}

impl NewWithNullPtr for wire_Point {
    fn new_with_null_ptr() -> Self {
        return Self {
            x: Default::default(),
            y: Default::default(),
        };
    }
}

impl NewWithNullPtr for wire_Size {
    fn new_with_null_ptr() -> Self {
        return Self {
            width: Default::default(),
            height: Default::default(),
        };
    }
}

impl NewWithNullPtr for wire_TreeNode {
    fn new_with_null_ptr() -> Self {
        return Self {
            name: core::ptr::null_mut(),
            children: core::ptr::null_mut(),
        };
    }
}

// Section: impl IntoDart

impl support::IntoDart for Size {
    fn into_dart(self) -> support::DartCObject {
        vec![self.width.into_dart(), self.height.into_dart()].into_dart()
    }
}
impl support::IntoDartExceptPrimitive for Size {}

impl support::IntoDart for TreeNode {
    fn into_dart(self) -> support::DartCObject {
        vec![self.name.into_dart(), self.children.into_dart()].into_dart()
    }
}
impl support::IntoDartExceptPrimitive for TreeNode {}

// Section: executor

support::lazy_static! {
    pub static ref FLUTTER_RUST_BRIDGE_HANDLER: support::DefaultHandler = Default::default();
}

// Section: sync execution mode utility

#[no_mangle]
pub extern "C" fn free_WireSyncReturnStruct(val: support::WireSyncReturnStruct) {
    unsafe {
        let _ = support::vec_from_leak_ptr(val.ptr, val.len);
    }
}
