use std::{fs::File, io::Read, path::Path};

use anyhow::{anyhow, Result};
use syn::visit::{self, Visit};
use syn::{Item, VisRestricted, Visibility};
use walkdir::WalkDir;

#[derive(Default)]
pub struct Finder<'a> {
    needle: &'a str,
    fns: Vec<&'a syn::ItemFn>,
    structs: Vec<&'a syn::ItemStruct>,
    enums: Vec<&'a syn::ItemEnum>,
}

macro_rules! unwrap_single {
    ($( $expr:expr, $variant:path, $error:literal; )*) => ($(
        match $expr {
            [single] => return Ok($variant(single.clone())),
            [] => {},
            _ => return Err(anyhow!($error))
        }
    )*);
}

impl<'a> Finder<'a> {
    pub fn new(needle: &'a str) -> Self {
        Self {
            needle,
            ..Default::default()
        }
    }
    pub fn resolve(self) -> Result<Item> {
        unwrap_single! {
            self.fns[..], Item::Fn, "Multiple functions with identical names found.";
            self.structs[..], Item::Struct, "Multiple structs with identical names found.";
            self.enums[..], Item::Enum, "Multiple enums with identical names found.";
        }
        Err(anyhow!(
            "No matching items found, or multiple conflicting items found."
        ))
    }
}

/// Checks that the visibility is `pub` or `pub(crate)`.
fn crate_public(vis: &Visibility) -> bool {
    match vis {
        Visibility::Public(_) => true,
        Visibility::Restricted(VisRestricted {
            in_token: None,
            path,
            ..
        }) => path.is_ident("crate"),
        _ => false,
    }
}

impl<'ast> Visit<'ast> for Finder<'ast> {
    fn visit_item_fn(&mut self, i: &'ast syn::ItemFn) {
        if i.sig.ident == self.needle && crate_public(&i.vis) {
            self.fns.push(i);
        }
    }
    fn visit_item_struct(&mut self, i: &'ast syn::ItemStruct) {
        if i.ident == self.needle && crate_public(&i.vis) {
            self.structs.push(i);
        }
    }
    fn visit_item_enum(&mut self, i: &'ast syn::ItemEnum) {
        if i.ident == self.needle && crate_public(&i.vis) {
            self.enums.push(i);
        }
    }
}
