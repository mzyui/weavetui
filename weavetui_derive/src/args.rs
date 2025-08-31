//! This module handles the parsing of arguments for the `#[component]` attribute macro.
//! It defines the structure for a child entry, which consists of a key (name) and a type.

use syn::{parse::Parse, LitStr, Token, Type};

/// Represents a single child component entry parsed from the `children` attribute.
///
/// This struct holds the key (name) as a `LitStr` and the type of the child component
/// as a `Type`. It is used by the `#[component]` macro to define the children of a component.
///
/// ## Example
///
/// In `#[component(children = [ counter => Counter ])]`, `counter => Counter` is parsed
/// into a `ChildEntry`.
pub struct ChildEntry {
    /// The name of the child component (e.g., `"counter"`).
    pub key: LitStr,
    /// The `=>` token separating the key and the value.
    pub _arrow: Token![=>],
    /// The type of the child component (e.g., `Counter`).
    pub value: Type,
}

impl Parse for ChildEntry {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(ChildEntry {
            key: input.parse()?,
            _arrow: input.parse()?,
            value: input.parse()?,
        })
    }
}
