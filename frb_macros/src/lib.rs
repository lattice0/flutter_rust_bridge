use proc_macro::*;

fn remove_marker_attr(input: TokenStream, ident: &str) -> TokenStream {
    use TokenTree as tt;
    input
        .into_iter()
        .scan(None, |state, x| match (state, x) {
            (pound @ None, tt::Punct(p)) if p.as_char() == '#' => {
                *pound = Some(tt::Punct(p));
                Some(vec![])
            }
            (p @ Some(_), tt::Group(g)) => match g.delimiter() {
                Delimiter::Bracket => match g.stream().into_iter().next() {
                    Some(tt::Ident(i)) if i.to_string() == ident => {
                        let _ = p.take();
                        Some(vec![])
                    }
                    Some(_) => Some(vec![p.take().unwrap(), tt::Group(g)]),
                    _ => Some(vec![tt::Group(g)]),
                },
                _ => Some(vec![p.take().unwrap(), tt::Group(g)]),
            },
            (None, tt::Group(g)) => Some(vec![tt::Group(Group::new(
                g.delimiter(),
                remove_marker_attr(g.stream(), ident),
            ))]),
            (_, x) => Some(vec![x]),
        })
        .flatten()
        .collect()
}

/// Attribute to guide code generation.
/// ### Common attributes
/// - `attr = "@.."` attaches additional attributes onto an item.
///     Can be declared multiple times.
/// - `json` generates json_serializable boilerplate for an enum or struct.
/// - `deprecated` denotes deprecation of an item.
/// ### Attributes on structs
/// - `no_final` denotes that all fields in this struct are non-final.
/// ### Attributes on fields
/// - `no_final` denotes that this field is non-final.
///     Also marks the class's constructor as non-const.
///     Has no effect if already declared at the struct level.
/// - `final` denotes that this field is non-reassignable, which is
///     the default. Takes precedence over declarations of `no_final`.
/// ### Attributes on consts
/// - `custom_section` denotes that the contents of a string literal are to be
///     copied into the generated file. Other than the order of declaration,
///     no guarantees are made concerning the placement of the code blocks.
#[proc_macro_attribute]
pub fn frb(_: TokenStream, item: TokenStream) -> TokenStream {
    remove_marker_attr(item, "frb")
}
