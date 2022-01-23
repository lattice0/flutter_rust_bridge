#![allow(non_camel_case_types, unused, clippy::redundant_closure, clippy::useless_conversion, non_snake_case)]
        // AUTO GENERATED FILE, DO NOT EDIT.
// Generated by `flutter_rust_bridge`.

        use crate::api::*;
        use flutter_rust_bridge::*;

        // Section: wire functions

        #[wasm_bindgen] pub fn wire_simple_adder(port_: i64,a: i32,b: i32)   { FLUTTER_RUST_BRIDGE_HANDLER.wrap(WrapInfo{ debug_name: "simple_adder", port: Some(port_), mode: FfiCallMode::Normal }, move || {
                let api_a = a.wire2api();let api_b = b.wire2api();
                    move |task_callback| simple_adder(api_a, api_b)
                    
            })
                 }

#[wasm_bindgen] pub fn wire_primitive_types(port_: i64,my_i32: i32,my_i64: i64,my_f64: f64,my_bool: bool)   { FLUTTER_RUST_BRIDGE_HANDLER.wrap(WrapInfo{ debug_name: "primitive_types", port: Some(port_), mode: FfiCallMode::Normal }, move || {
                let api_my_i32 = my_i32.wire2api();let api_my_i64 = my_i64.wire2api();let api_my_f64 = my_f64.wire2api();let api_my_bool = my_bool.wire2api();
                    move |task_callback| primitive_types(api_my_i32, api_my_i64, api_my_f64, api_my_bool)
                    
            })
                 }

#[wasm_bindgen] pub fn wire_primitive_u32(port_: i64,my_u32: u32)   { FLUTTER_RUST_BRIDGE_HANDLER.wrap(WrapInfo{ debug_name: "primitive_u32", port: Some(port_), mode: FfiCallMode::Normal }, move || {
                let api_my_u32 = my_u32.wire2api();
                    move |task_callback| primitive_u32(api_my_u32)
                    
            })
                 }

#[wasm_bindgen] pub fn wire_handle_string(port_: i64,s: String)   { FLUTTER_RUST_BRIDGE_HANDLER.wrap(WrapInfo{ debug_name: "handle_string", port: Some(port_), mode: FfiCallMode::Normal }, move || {
                let api_s = s.wire2api();
                    move |task_callback| handle_string(api_s)
                    
            })
                 }

#[wasm_bindgen] pub fn wire_handle_return_unit(port_: i64)   { FLUTTER_RUST_BRIDGE_HANDLER.wrap(WrapInfo{ debug_name: "handle_return_unit", port: Some(port_), mode: FfiCallMode::Normal }, move || {
                
                    move |task_callback| handle_return_unit()
                    
            })
                 }

#[wasm_bindgen] pub fn wire_handle_vec_u8(port_: i64,v: Box<[u8]>)   { FLUTTER_RUST_BRIDGE_HANDLER.wrap(WrapInfo{ debug_name: "handle_vec_u8", port: Some(port_), mode: FfiCallMode::Normal }, move || {
                let api_v = v.wire2api();
                    move |task_callback| handle_vec_u8(api_v)
                    
            })
                 }

#[wasm_bindgen] pub fn wire_handle_vec_of_primitive(port_: i64,n: i32)   { FLUTTER_RUST_BRIDGE_HANDLER.wrap(WrapInfo{ debug_name: "handle_vec_of_primitive", port: Some(port_), mode: FfiCallMode::Normal }, move || {
                let api_n = n.wire2api();
                    move |task_callback| handle_vec_of_primitive(api_n)
                    
            })
                 }

#[wasm_bindgen] pub fn wire_handle_zero_copy_vec_of_primitive(port_: i64,n: i32)   { FLUTTER_RUST_BRIDGE_HANDLER.wrap(WrapInfo{ debug_name: "handle_zero_copy_vec_of_primitive", port: Some(port_), mode: FfiCallMode::Normal }, move || {
                let api_n = n.wire2api();
                    move |task_callback| handle_zero_copy_vec_of_primitive(api_n)
                    
            })
                 }

#[wasm_bindgen] pub fn wire_handle_struct(port_: i64,arg: JsValue,boxed: JsValue)   { FLUTTER_RUST_BRIDGE_HANDLER.wrap(WrapInfo{ debug_name: "handle_struct", port: Some(port_), mode: FfiCallMode::Normal }, move || {
                let api_arg = arg.wire2api();let api_boxed = boxed.wire2api();
                    move |task_callback| handle_struct(api_arg, api_boxed)
                    
            })
                 }

#[wasm_bindgen] pub fn wire_handle_newtype(port_: i64,arg: JsValue)   { FLUTTER_RUST_BRIDGE_HANDLER.wrap(WrapInfo{ debug_name: "handle_newtype", port: Some(port_), mode: FfiCallMode::Normal }, move || {
                let api_arg = arg.wire2api();
                    move |task_callback| handle_newtype(api_arg)
                    
            })
                 }

#[wasm_bindgen] pub fn wire_handle_list_of_struct(port_: i64,l: Box<[JsValue]>)   { FLUTTER_RUST_BRIDGE_HANDLER.wrap(WrapInfo{ debug_name: "handle_list_of_struct", port: Some(port_), mode: FfiCallMode::Normal }, move || {
                let api_l = l.wire2api();
                    move |task_callback| handle_list_of_struct(api_l)
                    
            })
                 }

#[wasm_bindgen] pub fn wire_handle_string_list(port_: i64,names: Box<[JsString]>)   { FLUTTER_RUST_BRIDGE_HANDLER.wrap(WrapInfo{ debug_name: "handle_string_list", port: Some(port_), mode: FfiCallMode::Normal }, move || {
                let api_names = names.wire2api();
                    move |task_callback| handle_string_list(api_names)
                    
            })
                 }

#[wasm_bindgen] pub fn wire_handle_complex_struct(port_: i64,s: JsValue)   { FLUTTER_RUST_BRIDGE_HANDLER.wrap(WrapInfo{ debug_name: "handle_complex_struct", port: Some(port_), mode: FfiCallMode::Normal }, move || {
                let api_s = s.wire2api();
                    move |task_callback| handle_complex_struct(api_s)
                    
            })
                 }

#[wasm_bindgen] pub fn wire_handle_sync_return(mode: String) -> support::WireSyncReturnStruct { FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(WrapInfo{ debug_name: "handle_sync_return", port: None, mode: FfiCallMode::Sync }, move || {
                let api_mode = mode.wire2api();
                    handle_sync_return(api_mode)
            })
                 }

#[wasm_bindgen] pub fn wire_handle_stream(port_: i64,arg: String)   { FLUTTER_RUST_BRIDGE_HANDLER.wrap(WrapInfo{ debug_name: "handle_stream", port: Some(port_), mode: FfiCallMode::Stream }, move || {
                let api_arg = arg.wire2api();
                    move |task_callback| handle_stream(task_callback.stream_sink(), api_arg)
                    
            })
                 }

#[wasm_bindgen] pub fn wire_return_err(port_: i64)   { FLUTTER_RUST_BRIDGE_HANDLER.wrap(WrapInfo{ debug_name: "return_err", port: Some(port_), mode: FfiCallMode::Normal }, move || {
                
                    move |task_callback| return_err()
                    
            })
                 }

#[wasm_bindgen] pub fn wire_return_panic(port_: i64)   { FLUTTER_RUST_BRIDGE_HANDLER.wrap(WrapInfo{ debug_name: "return_panic", port: Some(port_), mode: FfiCallMode::Normal }, move || {
                
                    move |task_callback| return_panic()
                    
            })
                 }

#[wasm_bindgen] pub fn wire_handle_optional_return(port_: i64,left: f64,right: f64)   { FLUTTER_RUST_BRIDGE_HANDLER.wrap(WrapInfo{ debug_name: "handle_optional_return", port: Some(port_), mode: FfiCallMode::Normal }, move || {
                let api_left = left.wire2api();let api_right = right.wire2api();
                    move |task_callback| handle_optional_return(api_left, api_right)
                    
            })
                 }

#[wasm_bindgen] pub fn wire_handle_optional_struct(port_: i64,document: Option<String>)   { FLUTTER_RUST_BRIDGE_HANDLER.wrap(WrapInfo{ debug_name: "handle_optional_struct", port: Some(port_), mode: FfiCallMode::Normal }, move || {
                let api_document = document.wire2api();
                    move |task_callback| handle_optional_struct(api_document)
                    
            })
                 }

#[wasm_bindgen] pub fn wire_handle_optional_increment(port_: i64,opt: JsValue)   { FLUTTER_RUST_BRIDGE_HANDLER.wrap(WrapInfo{ debug_name: "handle_optional_increment", port: Some(port_), mode: FfiCallMode::Normal }, move || {
                let api_opt = opt.wire2api();
                    move |task_callback| handle_optional_increment(api_opt)
                    
            })
                 }

#[wasm_bindgen] pub fn wire_handle_increment_boxed_optional(port_: i64,opt: Option<f64>)   { FLUTTER_RUST_BRIDGE_HANDLER.wrap(WrapInfo{ debug_name: "handle_increment_boxed_optional", port: Some(port_), mode: FfiCallMode::Normal }, move || {
                let api_opt = opt.wire2api();
                    move |task_callback| handle_increment_boxed_optional(api_opt)
                    
            })
                 }

#[wasm_bindgen] pub fn wire_handle_option_box_arguments(port_: i64,i8box: Option<i8>,u8box: Option<u8>,i32box: Option<i32>,i64box: Option<i64>,f64box: Option<f64>,boolbox: Option<bool>,structbox: JsValue)   { FLUTTER_RUST_BRIDGE_HANDLER.wrap(WrapInfo{ debug_name: "handle_option_box_arguments", port: Some(port_), mode: FfiCallMode::Normal }, move || {
                let api_i8box = i8box.wire2api();let api_u8box = u8box.wire2api();let api_i32box = i32box.wire2api();let api_i64box = i64box.wire2api();let api_f64box = f64box.wire2api();let api_boolbox = boolbox.wire2api();let api_structbox = structbox.wire2api();
                    move |task_callback| handle_option_box_arguments(api_i8box, api_u8box, api_i32box, api_i64box, api_f64box, api_boolbox, api_structbox)
                    
            })
                 }

#[wasm_bindgen] pub fn wire_handle_return_enum(port_: i64,input: String)   { FLUTTER_RUST_BRIDGE_HANDLER.wrap(WrapInfo{ debug_name: "handle_return_enum", port: Some(port_), mode: FfiCallMode::Normal }, move || {
                let api_input = input.wire2api();
                    move |task_callback| handle_return_enum(api_input)
                    
            })
                 }

#[wasm_bindgen] pub fn wire_handle_enum_parameter(port_: i64,weekday: int)   { FLUTTER_RUST_BRIDGE_HANDLER.wrap(WrapInfo{ debug_name: "handle_enum_parameter", port: Some(port_), mode: FfiCallMode::Normal }, move || {
                let api_weekday = weekday.wire2api();
                    move |task_callback| handle_enum_parameter(api_weekday)
                    
            })
                 }

#[wasm_bindgen] pub fn wire_handle_u64_vec(port_: i64,vec: Option<Box<[u64]>>)   { FLUTTER_RUST_BRIDGE_HANDLER.wrap(WrapInfo{ debug_name: "handle_u64_vec", port: Some(port_), mode: FfiCallMode::Normal }, move || {
                let api_vec = vec.wire2api();
                    move |task_callback| handle_u64_vec(api_vec)
                    
            })
                 }

#[wasm_bindgen] pub fn wire_handle_customized_struct(port_: i64,val: JsValue)   { FLUTTER_RUST_BRIDGE_HANDLER.wrap(WrapInfo{ debug_name: "handle_customized_struct", port: Some(port_), mode: FfiCallMode::Normal }, move || {
                let api_val = val.wire2api();
                    move |task_callback| handle_customized_struct(api_val)
                    
            })
                 }

#[wasm_bindgen] pub fn wire_handle_enum_struct(port_: i64,val: wire_KitchenSink)   { FLUTTER_RUST_BRIDGE_HANDLER.wrap(WrapInfo{ debug_name: "handle_enum_struct", port: Some(port_), mode: FfiCallMode::Normal }, move || {
                let api_val = val.wire2api();
                    move |task_callback| handle_enum_struct(api_val)
                    
            })
                 }

        // Section: wire structs

        





#[wasm_bindgen]
            extern "C" {
                pub type wire_Attribute;
                #[wasm_bindgen(method, getter)] pub fn key(this: &wire_Attribute) -> String;
#[wasm_bindgen(method, getter)] pub fn value(this: &wire_Attribute) -> String;
            }
            











































#[wasm_bindgen]
            extern "C" {
                pub type wire_Customized;
                #[wasm_bindgen(method, getter)] pub fn final_field(this: &wire_Customized) -> String;
#[wasm_bindgen(method, getter)] pub fn non_final_field(this: &wire_Customized) -> Option<String>;
            }
            

#[wasm_bindgen]
            extern "C" {
                pub type wire_ExoticOptionals;
                #[wasm_bindgen(method, getter)] pub fn int32(this: &wire_ExoticOptionals) -> Option<i32>;
#[wasm_bindgen(method, getter)] pub fn int64(this: &wire_ExoticOptionals) -> Option<i64>;
#[wasm_bindgen(method, getter)] pub fn float64(this: &wire_ExoticOptionals) -> Option<f64>;
#[wasm_bindgen(method, getter)] pub fn boolean(this: &wire_ExoticOptionals) -> Option<bool>;
#[wasm_bindgen(method, getter)] pub fn zerocopy(this: &wire_ExoticOptionals) -> Option<Box<[u8]>>;
#[wasm_bindgen(method, getter)] pub fn int8list(this: &wire_ExoticOptionals) -> Option<Box<[i8]>>;
#[wasm_bindgen(method, getter)] pub fn uint8list(this: &wire_ExoticOptionals) -> Option<Box<[u8]>>;
#[wasm_bindgen(method, getter)] pub fn int32list(this: &wire_ExoticOptionals) -> Option<Box<[i32]>>;
#[wasm_bindgen(method, getter)] pub fn int64list(this: &wire_ExoticOptionals) -> Option<Box<[i64]>>;
#[wasm_bindgen(method, getter)] pub fn float32list(this: &wire_ExoticOptionals) -> Option<Box<[f32]>>;
#[wasm_bindgen(method, getter)] pub fn float64list(this: &wire_ExoticOptionals) -> Option<Box<[f64]>>;
#[wasm_bindgen(method, getter)] pub fn attributes(this: &wire_ExoticOptionals) -> Option<Box<[JsValue]>>;
#[wasm_bindgen(method, getter)] pub fn attributes_nullable(this: &wire_ExoticOptionals) -> Box<[JsValue]>;
#[wasm_bindgen(method, getter)] pub fn nullable_attributes(this: &wire_ExoticOptionals) -> Option<Box<[JsValue]>>;
#[wasm_bindgen(method, getter)] pub fn newtypeint(this: &wire_ExoticOptionals) -> JsValue;
#[wasm_bindgen(method, getter)] pub fn string_list(this: &wire_ExoticOptionals) -> Option<Box<[JsString]>>;
            }
            





















#[wasm_bindgen]
            extern "C" {
                pub type wire_KitchenSink;
                #[wasm_bindgen(method, getter)] pub fn kind(this: &wire_KitchenSink) -> JsValue;
#[wasm_bindgen(method, getter)] pub fn tag(this: &wire_KitchenSink) -> i32;
            }
            #[wasm_bindgen]
                    extern "C" {
                        pub type KitchenSink_Empty;
                        
                    }
#[wasm_bindgen]
                    extern "C" {
                        pub type KitchenSink_Primitives;
                        #[wasm_bindgen(method, getter)] pub fn int32(this: &KitchenSink_Primitives) -> i32;
#[wasm_bindgen(method, getter)] pub fn float64(this: &KitchenSink_Primitives) -> f64;
#[wasm_bindgen(method, getter)] pub fn boolean(this: &KitchenSink_Primitives) -> bool;
                    }
#[wasm_bindgen]
                    extern "C" {
                        pub type KitchenSink_Nested;
                        #[wasm_bindgen(method, getter)] pub fn field0(this: &KitchenSink_Nested) -> wire_KitchenSink;
                    }
#[wasm_bindgen]
                    extern "C" {
                        pub type KitchenSink_Optional;
                        #[wasm_bindgen(method, getter)] pub fn field0(this: &KitchenSink_Optional) -> Option<i32>;
#[wasm_bindgen(method, getter)] pub fn field1(this: &KitchenSink_Optional) -> Option<i32>;
                    }
#[wasm_bindgen]
                    extern "C" {
                        pub type KitchenSink_Buffer;
                        #[wasm_bindgen(method, getter)] pub fn field0(this: &KitchenSink_Buffer) -> Box<[u8]>;
                    }
#[wasm_bindgen]
                    extern "C" {
                        pub type KitchenSink_Enums;
                        #[wasm_bindgen(method, getter)] pub fn field0(this: &KitchenSink_Enums) -> int;
                    }









#[wasm_bindgen]
            extern "C" {
                pub type wire_MySize;
                #[wasm_bindgen(method, getter)] pub fn width(this: &wire_MySize) -> i32;
#[wasm_bindgen(method, getter)] pub fn height(this: &wire_MySize) -> i32;
            }
            

#[wasm_bindgen]
            extern "C" {
                pub type wire_MyTreeNode;
                #[wasm_bindgen(method, getter)] pub fn value_i32(this: &wire_MyTreeNode) -> i32;
#[wasm_bindgen(method, getter)] pub fn value_vec_u8(this: &wire_MyTreeNode) -> Box<[u8]>;
#[wasm_bindgen(method, getter)] pub fn value_boolean(this: &wire_MyTreeNode) -> bool;
#[wasm_bindgen(method, getter)] pub fn children(this: &wire_MyTreeNode) -> Box<[JsValue]>;
            }
            

#[wasm_bindgen]
            extern "C" {
                pub type wire_NewTypeInt;
                #[wasm_bindgen(method, getter)] pub fn field0(this: &wire_NewTypeInt) -> i64;
            }
            

































































        // Section: wire enums

        



        // Section: allocate functions

        

























































































































































        // Section: impl Wire2Api

        pub trait Wire2Api<T> {
            fn wire2api(self) -> T;
        }

        impl<T, S> Wire2Api<Option<T>> for *mut S
        where
            *mut S: Wire2Api<T>
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

        impl Wire2Api<String> for String {
                fn wire2api(self) -> String {
                    self
                }
            }

impl Wire2Api<Vec<String>> for Box<[JsString]> {
                fn wire2api(self) -> Vec<String> {
                    self.into_iter().map(|e| e
                    .as_string().expect("not a string, or invalid utf-8")).collect()
                }
            }

impl Wire2Api<ZeroCopyBuffer<Vec<u8>>> for Box<[u8]> {
                fn wire2api(self) -> ZeroCopyBuffer<Vec<u8>> {
                    ZeroCopyBuffer(self.to_vec())
                }
            }

impl Wire2Api<Attribute> for &JsValue {
                fn wire2api(self) -> Attribute {
                    let raw = self.unchecked_ref::<wire_Attribute>(); Attribute{key:  raw.key().wire2api(),value:  raw.value().wire2api()}
                }
            }

impl Wire2Api<bool> for bool {
                fn wire2api(self) -> bool {
                    self
                }
            }























impl Wire2Api<Box<bool>> for bool {
                fn wire2api(self) -> Box<bool> {
                    Box::new(self)
                }
            }

impl Wire2Api<Box<ExoticOptionals>> for &JsValue {
                fn wire2api(self) -> Box<ExoticOptionals> {
                    Box::new(self.wire2api())
                }
            }

impl Wire2Api<Box<f64>> for f64 {
                fn wire2api(self) -> Box<f64> {
                    Box::new(self)
                }
            }

impl Wire2Api<Box<i32>> for i32 {
                fn wire2api(self) -> Box<i32> {
                    Box::new(self)
                }
            }

impl Wire2Api<Box<i64>> for i64 {
                fn wire2api(self) -> Box<i64> {
                    Box::new(self)
                }
            }

impl Wire2Api<Box<i8>> for i8 {
                fn wire2api(self) -> Box<i8> {
                    Box::new(self)
                }
            }

impl Wire2Api<Box<KitchenSink>> for &wire_KitchenSink {
                fn wire2api(self) -> Box<KitchenSink> {
                    Box::new(self.wire2api())
                }
            }

impl Wire2Api<Box<MySize>> for &JsValue {
                fn wire2api(self) -> Box<MySize> {
                    Box::new(self.wire2api())
                }
            }

impl Wire2Api<Box<u8>> for u8 {
                fn wire2api(self) -> Box<u8> {
                    Box::new(self)
                }
            }

impl Wire2Api<Customized> for &JsValue {
                fn wire2api(self) -> Customized {
                    let raw = self.unchecked_ref::<wire_Customized>(); Customized{final_field:  raw.final_field().wire2api(),non_final_field:  raw.non_final_field().wire2api()}
                }
            }

impl Wire2Api<ExoticOptionals> for &JsValue {
                fn wire2api(self) -> ExoticOptionals {
                    let raw = self.unchecked_ref::<wire_ExoticOptionals>(); ExoticOptionals{int32:  raw.int32().wire2api(),int64:  raw.int64().wire2api(),float64:  raw.float64().wire2api(),boolean:  raw.boolean().wire2api(),zerocopy:  raw.zerocopy().wire2api(),int8list:  raw.int8list().wire2api(),uint8list:  raw.uint8list().wire2api(),int32list:  raw.int32list().wire2api(),int64list:  raw.int64list().wire2api(),float32list:  raw.float32list().wire2api(),float64list:  raw.float64list().wire2api(),attributes:  raw.attributes().wire2api(),attributes_nullable:  raw.attributes_nullable().wire2api(),nullable_attributes:  raw.nullable_attributes().wire2api(),newtypeint:  raw.newtypeint().wire2api(),string_list:  raw.string_list().wire2api()}
                }
            }

impl Wire2Api<f32> for f32 {
                fn wire2api(self) -> f32 {
                    self
                }
            }

impl Wire2Api<f64> for f64 {
                fn wire2api(self) -> f64 {
                    self
                }
            }

impl Wire2Api<Vec<f32>> for Box<[f32]> {
                fn wire2api(self) -> Vec<f32> {
                    self.to_vec()
                }
            }

impl Wire2Api<Vec<f64>> for Box<[f64]> {
                fn wire2api(self) -> Vec<f64> {
                    self.to_vec()
                }
            }

impl Wire2Api<i32> for i32 {
                fn wire2api(self) -> i32 {
                    self
                }
            }

impl Wire2Api<i64> for i64 {
                fn wire2api(self) -> i64 {
                    self
                }
            }

impl Wire2Api<i8> for i8 {
                fn wire2api(self) -> i8 {
                    self
                }
            }

impl Wire2Api<Vec<i32>> for Box<[i32]> {
                fn wire2api(self) -> Vec<i32> {
                    self.to_vec()
                }
            }

impl Wire2Api<Vec<i64>> for Box<[i64]> {
                fn wire2api(self) -> Vec<i64> {
                    self.to_vec()
                }
            }

impl Wire2Api<Vec<i8>> for Box<[i8]> {
                fn wire2api(self) -> Vec<i8> {
                    self.to_vec()
                }
            }

impl Wire2Api<KitchenSink> for &wire_KitchenSink {
                fn wire2api(self) -> KitchenSink {
                    match self.tag() { 0 => KitchenSink::Empty,
1 => {
                                    let kind = self.kind().unchecked_into::<KitchenSink_Primitives>();
                                    KitchenSink::Primitives{int32:  kind.int32().wire2api(),float64:  kind.float64().wire2api(),boolean:  kind.boolean().wire2api()}
                                }
2 => {
                                    let kind = self.kind().unchecked_into::<KitchenSink_Nested>();
                                    KitchenSink::Nested( kind.field0().wire2api())
                                }
3 => {
                                    let kind = self.kind().unchecked_into::<KitchenSink_Optional>();
                                    KitchenSink::Optional( kind.field0().wire2api(), kind.field1().wire2api())
                                }
4 => {
                                    let kind = self.kind().unchecked_into::<KitchenSink_Buffer>();
                                    KitchenSink::Buffer( kind.field0().wire2api())
                                }
5 => {
                                    let kind = self.kind().unchecked_into::<KitchenSink_Enums>();
                                    KitchenSink::Enums( kind.field0().wire2api())
                                } _ => unreachable!() }
                }
            }

impl Wire2Api<Vec<Attribute>> for Box<[JsValue]> {
                fn wire2api(self) -> Vec<Attribute> {
                    self.iter().map(Wire2Api::wire2api).collect()
                }
            }

impl Wire2Api<Vec<MySize>> for Box<[JsValue]> {
                fn wire2api(self) -> Vec<MySize> {
                    self.iter().map(Wire2Api::wire2api).collect()
                }
            }

impl Wire2Api<Vec<MyTreeNode>> for Box<[JsValue]> {
                fn wire2api(self) -> Vec<MyTreeNode> {
                    self.iter().map(Wire2Api::wire2api).collect()
                }
            }

impl Wire2Api<Vec<Option<Attribute>>> for Box<[JsValue]> {
                fn wire2api(self) -> Vec<Option<Attribute>> {
                    self.iter().map(Wire2Api::wire2api).collect()
                }
            }

impl Wire2Api<MySize> for &JsValue {
                fn wire2api(self) -> MySize {
                    let raw = self.unchecked_ref::<wire_MySize>(); MySize{width:  raw.width().wire2api(),height:  raw.height().wire2api()}
                }
            }

impl Wire2Api<MyTreeNode> for &JsValue {
                fn wire2api(self) -> MyTreeNode {
                    let raw = self.unchecked_ref::<wire_MyTreeNode>(); MyTreeNode{value_i32:  raw.value_i32().wire2api(),value_vec_u8:  raw.value_vec_u8().wire2api(),value_boolean:  raw.value_boolean().wire2api(),children:  raw.children().wire2api()}
                }
            }

impl Wire2Api<NewTypeInt> for &JsValue {
                fn wire2api(self) -> NewTypeInt {
                    let raw = self.unchecked_ref::<wire_NewTypeInt>(); NewTypeInt( raw.field0().wire2api())
                }
            }







impl Wire2Api<Option<Attribute>> for &JsValue {
                fn wire2api(self) -> Option<Attribute> {
                    Some(self.wire2api())
                }
            }



impl Wire2Api<Option<ExoticOptionals>> for &JsValue {
                fn wire2api(self) -> Option<ExoticOptionals> {
                    Some(self.wire2api())
                }
            }







impl Wire2Api<Option<NewTypeInt>> for &JsValue {
                fn wire2api(self) -> Option<NewTypeInt> {
                    Some(self.wire2api())
                }
            }



impl Wire2Api<Option<Box<ExoticOptionals>>> for &JsValue {
                fn wire2api(self) -> Option<Box<ExoticOptionals>> {
                    Some(self.wire2api())
                }
            }





























impl Wire2Api<u32> for u32 {
                fn wire2api(self) -> u32 {
                    self
                }
            }

impl Wire2Api<u64> for u64 {
                fn wire2api(self) -> u64 {
                    self
                }
            }

impl Wire2Api<u8> for u8 {
                fn wire2api(self) -> u8 {
                    self
                }
            }

impl Wire2Api<Vec<u64>> for Box<[u64]> {
                fn wire2api(self) -> Vec<u64> {
                    self.to_vec()
                }
            }

impl Wire2Api<Vec<u8>> for Box<[u8]> {
                fn wire2api(self) -> Vec<u8> {
                    self.to_vec()
                }
            }

impl Wire2Api<Weekdays> for int {
                fn wire2api(self) -> Weekdays {
                    match self { 0 => Weekdays::Monday,
1 => Weekdays::Tuesday,
2 => Weekdays::Wednesday,
3 => Weekdays::Thursday,
4 => Weekdays::Friday,
5 => Weekdays::Saturday,
6 => Weekdays::Sunday, _ => unreachable!() }
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

        

























































































































































        // Section: impl IntoDart
        

























impl support::IntoDart for Attribute {
                fn into_dart(self) -> support::DartCObject {
                    vec![
                        self.key.into_dart(),
self.value.into_dart()
                    ].into_dart()
                }
            }
            impl support::IntoDartExceptPrimitive for Attribute {}
            





















impl support::IntoDart for Element {
                fn into_dart(self) -> support::DartCObject {
                    vec![
                        self.tag.into_dart(),
self.text.into_dart(),
self.attributes.into_dart(),
self.children.into_dart()
                    ].into_dart()
                }
            }
            impl support::IntoDartExceptPrimitive for Element {}
            

impl support::IntoDart for ExoticOptionals {
                fn into_dart(self) -> support::DartCObject {
                    vec![
                        self.int32.into_dart(),
self.int64.into_dart(),
self.float64.into_dart(),
self.boolean.into_dart(),
self.zerocopy.into_dart(),
self.int8list.into_dart(),
self.uint8list.into_dart(),
self.int32list.into_dart(),
self.int64list.into_dart(),
self.float32list.into_dart(),
self.float64list.into_dart(),
self.attributes.into_dart(),
self.attributes_nullable.into_dart(),
self.nullable_attributes.into_dart(),
self.newtypeint.into_dart(),
self.string_list.into_dart()
                    ].into_dart()
                }
            }
            impl support::IntoDartExceptPrimitive for ExoticOptionals {}
            

























impl support::IntoDart for KitchenSink {
                fn into_dart(self) -> support::DartCObject {
                    match self {
                        Self::Empty => vec![0.into_dart()],
Self::Primitives{int32,float64,boolean} => vec![1.into_dart(),int32.into_dart(),float64.into_dart(),boolean.into_dart()],
Self::Nested(field0) => vec![2.into_dart(),field0.into_dart()],
Self::Optional(field0,field1) => vec![3.into_dart(),field0.into_dart(),field1.into_dart()],
Self::Buffer(field0) => vec![4.into_dart(),field0.into_dart()],
Self::Enums(field0) => vec![5.into_dart(),field0.into_dart()],
                    }.into_dart()
                }
            }











impl support::IntoDart for MySize {
                fn into_dart(self) -> support::DartCObject {
                    vec![
                        self.width.into_dart(),
self.height.into_dart()
                    ].into_dart()
                }
            }
            impl support::IntoDartExceptPrimitive for MySize {}
            

impl support::IntoDart for MyTreeNode {
                fn into_dart(self) -> support::DartCObject {
                    vec![
                        self.value_i32.into_dart(),
self.value_vec_u8.into_dart(),
self.value_boolean.into_dart(),
self.children.into_dart()
                    ].into_dart()
                }
            }
            impl support::IntoDartExceptPrimitive for MyTreeNode {}
            

impl support::IntoDart for NewTypeInt {
                fn into_dart(self) -> support::DartCObject {
                    vec![
                        self.0.into_dart()
                    ].into_dart()
                }
            }
            impl support::IntoDartExceptPrimitive for NewTypeInt {}
            





























































impl support::IntoDart for VecOfPrimitivePack {
                fn into_dart(self) -> support::DartCObject {
                    vec![
                        self.int8list.into_dart(),
self.uint8list.into_dart(),
self.int16list.into_dart(),
self.uint16list.into_dart(),
self.uint32list.into_dart(),
self.int32list.into_dart(),
self.uint64list.into_dart(),
self.int64list.into_dart(),
self.float32list.into_dart(),
self.float64list.into_dart()
                    ].into_dart()
                }
            }
            impl support::IntoDartExceptPrimitive for VecOfPrimitivePack {}
            

impl support::IntoDart for Weekdays {
                fn into_dart(self) -> support::DartCObject {
                    match self {
                        Self::Monday => 0,
Self::Tuesday => 1,
Self::Wednesday => 2,
Self::Thursday => 3,
Self::Friday => 4,
Self::Saturday => 5,
Self::Sunday => 6,
                    }.into_dart()
                }
            }

impl support::IntoDart for ZeroCopyVecOfPrimitivePack {
                fn into_dart(self) -> support::DartCObject {
                    vec![
                        self.int8list.into_dart(),
self.uint8list.into_dart(),
self.int16list.into_dart(),
self.uint16list.into_dart(),
self.uint32list.into_dart(),
self.int32list.into_dart(),
self.uint64list.into_dart(),
self.int64list.into_dart(),
self.float32list.into_dart(),
self.float64list.into_dart()
                    ].into_dart()
                }
            }
            impl support::IntoDartExceptPrimitive for ZeroCopyVecOfPrimitivePack {}
            

        // Section: executor
        support::lazy_static! {
                pub static ref FLUTTER_RUST_BRIDGE_HANDLER: support::DefaultHandler = Default::default();
            }
            

        // Section: sync execution mode utility
        
                #[no_mangle]
                pub extern "C" fn free_WireSyncReturnStruct(val: support::WireSyncReturnStruct)  {
                    unsafe { let _ = support::vec_from_leak_ptr(val.ptr, val.len); }
                }
            

        