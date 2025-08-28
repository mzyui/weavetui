//! This module handles the parsing of arguments for the `#[component]` attribute macro.
//! It defines the structure for a child entry, which consists of a key (name) and a type.

use syn::{LitStr, Token, Type, parse::Parse};

/// Represents a single child component entry parsed from the `children` attribute.
///
/// This struct holds the key (name) as a `LitStr` and the type of the child component
/// as a `Type` token.
pub struct ChildEntry {
    pub key: LitStr,
    pub _arrow: Token![=>],
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
