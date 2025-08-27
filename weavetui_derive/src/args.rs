use syn::{LitStr, Token, Type, parse::Parse};

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
