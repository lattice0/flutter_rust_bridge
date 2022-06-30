use crate::ir::*;

use convert_case::{Case, Casing};
use lazy_static::lazy_static;
use quote::ToTokens;
use regex::Regex;

#[derive(Debug, Clone)]
pub struct IrTypeOpaque {
    pub inner_rust: String,
    pub inner_dart: String,
}

fn char_not_alphanumeric(c: char) -> bool {
    !c.is_alphanumeric()
}

fn rust_type_to_dart_type(rust: &str) -> String {
    lazy_static! {
        static ref OPAQUE_FILTER: Regex =
            Regex::new(r"(\bdyn|'static|\bDartSafe|\+ (Send|Sync|UnwindSafe|RefUnwindSafe))\b")
                .unwrap();
    }
    OPAQUE_FILTER
        .replace_all(rust, "")
        .replace(char_not_alphanumeric, "_")
        .to_case(Case::Pascal)
}

impl From<&syn::Type> for IrTypeOpaque {
    fn from(rust_ty: &syn::Type) -> Self {
        let inner_dart = match rust_ty {
            syn::Type::Tuple(tup) if tup.elems.is_empty() => "void".to_owned(),
            _ => rust_type_to_dart_type(&rust_ty.into_token_stream().to_string()),
        };

        Self {
            inner_rust: rust_ty.to_token_stream().to_string(),
            inner_dart,
        }
    }
}

impl From<String> for IrTypeOpaque {
    fn from(inner_rust: String) -> Self {
        let inner_dart = rust_type_to_dart_type(&inner_rust);
        Self {
            inner_rust,
            inner_dart,
        }
    }
}

impl IrTypeOpaque {
    pub fn new_unit() -> Self {
        Self {
            inner_rust: "()".to_owned(),
            inner_dart: "void".to_owned(),
        }
    }
}

impl IrTypeTrait for IrTypeOpaque {
    fn safe_ident(&self) -> String {
        self.inner_dart.clone()
    }
    fn dart_api_type(&self) -> String {
        self.inner_dart.clone()
    }
    fn dart_wire_type(&self) -> String {
        format!("ffi.Pointer<{}>", self.rust_wire_type())
    }
    fn rust_wire_type(&self) -> String {
        format!("wire_{}", self.safe_ident())
    }
    fn rust_api_type(&self) -> String {
        format!("Opaque<{}>", self.inner_rust)
    }
    fn rust_wire_is_pointer(&self) -> bool {
        true
    }
    fn visit_children_types<F: FnMut(&super::IrType) -> bool>(&self, _: &mut F, _: &super::IrFile) {
        // Do nothing.
    }
}
