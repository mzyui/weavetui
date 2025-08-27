use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Fields, FieldsNamed, FieldsUnnamed, Ident, ItemStruct, parse::ParseStream, parse_macro_input,
    parse_quote, punctuated::Punctuated,
};
mod args;

#[proc_macro_attribute]
pub fn component(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(item as ItemStruct);

    let name = &ast.ident;

    let mut children_entries: Option<Punctuated<args::ChildEntry, syn::token::Comma>> = None;
    let mut default_component_impl = false;

    // --- Moved declarations to top ---
    let mut has_default_derive_initial = false;
    let mut has_debug_derive_initial = false;
    // --- End moved declarations ---

    // Parse the attribute (e.g., #[children("home" => Home, "button": Button), no_component_impl])
    if !attr.is_empty() {
        let input_tokens = proc_macro2::TokenStream::from(attr);
        syn::parse::Parser::parse2(
            |input: ParseStream| {
                while !input.is_empty() {
                    let lookahead = input.lookahead1();
                    if lookahead.peek(syn::Ident) && input.peek2(syn::token::Paren) {
                        let children_kw: Ident = input.parse()?;
                        if children_kw == "children" {
                            let content;
                            syn::parenthesized!(content in input);
                            children_entries = Some(Punctuated::parse_terminated(&content)?);
                        } else {
                            return Err(lookahead.error());
                        }
                    } else if lookahead.peek(syn::Ident) {
                        let ident: Ident = input.parse()?;
                        if ident == "default" {
                            // New branch for default
                            default_component_impl = true;
                        } else {
                            return Err(lookahead.error());
                        }
                    }
                    if !input.is_empty() && input.peek(syn::token::Comma) {
                        input.parse::<syn::token::Comma>()?;
                    }
                }
                Ok(())
            },
            input_tokens,
        )
        .expect(
            "Failed to parse attribute: expected `children(\"key\" => Type, ...)` or `default`",
        );
    }

    // Determine the type of the children field - ALWAYS Vec<Box<dyn Component>>
    let actual_children_type =
        quote! { std::collections::BTreeMap<String, Box<dyn weavetui_core::Component>> };

    // Add children field if not already present
    let children_field_name = Ident::new("children", name.span());
    let mut found_children_field = false;
    if let Fields::Named(FieldsNamed { named, .. }) = &mut ast.fields {
        for field in named.iter() {
            if field
                .ident
                .as_ref()
                .is_some_and(|ident| ident == "children")
            {
                found_children_field = true;
                break;
            }
        }
        if !found_children_field {
            let new_children_field: syn::Field =
                parse_quote! { pub #children_field_name: #actual_children_type };
            named.push(new_children_field);
        }
        // Add _active field if not present
        if !named
            .iter()
            .any(|f| f.ident.as_ref().is_some_and(|i| i == "_active"))
        {
            named.push(parse_quote! { pub _active: bool });
        }
        // Add _action_tx field if not present
        if !named
            .iter()
            .any(|f| f.ident.as_ref().is_some_and(|i| i == "_action_tx"))
        {
            named.push(
                parse_quote! { pub _action_tx: Option<tokio::sync::mpsc::UnboundedSender<String>> },
            );
        }
    } else if let Fields::Unnamed(FieldsUnnamed {
        unnamed: _unnamed, ..
    }) = &mut ast.fields
    {
        // For tuple structs, we can't easily add named fields. This case is complex.
        // For simplicity, we'll assume named fields for now or panic.
        panic!("#[component] does not support unnamed fields when adding children automatically.");
    } else {
        // Unit struct, add named children field
        let new_children_field: syn::Field = parse_quote! { pub children: #actual_children_type };
        let mut named_fields = syn::punctuated::Punctuated::new();
        named_fields.push(new_children_field);
        named_fields.push(parse_quote! { pub _active: bool });
        named_fields.push(
            parse_quote! { pub _action_tx: Option<tokio::sync::mpsc::UnboundedSender<String>> },
        );
        ast.fields = Fields::Named(FieldsNamed {
            brace_token: syn::token::Brace::default(),
            named: named_fields,
        });
    }

    // --- Moved this block here to set has_default_derive_initial before use ---
    for attr in &ast.attrs {
        if attr.path().is_ident("derive") {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("Default") {
                    has_default_derive_initial = true;
                }
                if meta.path.is_ident("Debug") {
                    has_debug_derive_initial = true;
                }
                Ok(())
            })
            .expect("Failed to parse derive attribute");
        }
    }
    // --- End moved block ---

    // Remove the automatic derive(Default) and derive(Debug)
    ast.attrs.retain(|attr| {
        if attr.path().is_ident("derive") {
            let mut keep = true;
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("Default") || meta.path.is_ident("Debug") {
                    keep = false;
                }
                Ok(())
            })
            .expect("Failed to parse derive attribute");
            keep
        } else {
            true
        }
    });

    // Add derive(Debug) back
    ast.attrs.push(parse_quote! { #[derive(Debug)] });

    let default_impl = if let Some(entries) = children_entries {
        let children_inits = entries.iter().map(|entry| {
            let key = &entry.key;
            let ty = &entry.value;
            quote! {
                children_map.insert(#key.to_string(), Box::new(#ty::default()) as Box<dyn weavetui_core::Component>);
            }
        });

        let mut field_initializers = Vec::new();
        if let Fields::Named(FieldsNamed { named, .. }) = &ast.fields {
            for field in named.iter() {
                let field_name = field.ident.as_ref().unwrap();
                if field_name == &children_field_name {
                    // Use the actual children field name
                    field_initializers.push(quote! { #field_name: children_map });
                } else {
                    field_initializers.push(quote! { #field_name: Default::default() });
                }
            }
        } else {
            // Handle unit structs or unnamed fields if necessary, though the macro panics for unnamed fields.
            // For unit structs, we've already added named fields, so this branch might not be hit.
            // If it is, it means the original struct was a unit struct and we added fields.
            // In that case, we need to initialize the added fields.
            field_initializers.push(quote! { children: children_map });
            field_initializers.push(quote! { _active: Default::default() });
            field_initializers.push(quote! { _action_tx: Default::default() });
        }

        quote! {
            impl Default for #name {
                fn default() -> Self {
                    let mut children_map = std::collections::BTreeMap::new();
                    #(#children_inits)*
                    Self {
                        #(#field_initializers),*
                    }
                }
            }
        }
    } else {
        // If no children are specified, provide a basic default impl
        // This will ensure that `children`, `_active`, `_action_tx` are initialized
        // and other fields are initialized via `Default::default()`
        let mut field_initializers = Vec::new();
        if let Fields::Named(FieldsNamed { named, .. }) = &ast.fields {
            for field in named.iter() {
                let field_name = field.ident.as_ref().unwrap();
                if field_name == &children_field_name {
                    field_initializers
                        .push(quote! { #field_name: std::collections::BTreeMap::new() });
                } else {
                    field_initializers.push(quote! { #field_name: Default::default() });
                }
            }
        } else {
            field_initializers.push(quote! { children: std::collections::BTreeMap::new() });
            field_initializers.push(quote! { _active: Default::default() });
            field_initializers.push(quote! { _action_tx: Default::default() });
        }

        quote! {
            impl Default for #name {
                fn default() -> Self {
                    Self {
                        #(#field_initializers),*
                    }
                }
            }
        }
    };

    let component_impl = if default_component_impl {
        quote! {
            impl weavetui_core::Component for #name {
                fn draw(&mut self, f: &mut ratatui::Frame<'_>, area: ratatui::layout::Rect) {
                    f.render_widget(
                        ratatui::widgets::Block::bordered()
                            .title_top(ratatui::text::Line::from(format!(" {}: {} x {} ", self.name(), area.height, area.width)))
                            .title_alignment(ratatui::layout::Alignment::Center),
                        area.inner(ratatui::layout::Margin { horizontal: 1, vertical: 1 })
                    );
                }
            }
        }
    } else {
        quote! {}
    };

    let expanded = quote! {
        #ast // The modified struct definition

        #component_impl

        impl weavetui_core::ComponentAccessor for #name {
            fn name(&self) -> String {
                stringify!(#name).to_string()
            }

            fn is_active(&self) -> bool {
                self._active
            }

            fn set_active(&mut self, active: bool) {
                self._active = active;
                (self as &mut dyn weavetui_core::Component).on_active_changed(active);

                for child in self.children.values_mut() {
                    child.set_active(active);
                }
            }

            fn register_action_handler(&mut self, tx: tokio::sync::mpsc::UnboundedSender<String>) {
                self._action_tx = Some(tx);
            }

            fn send(&self, action: &str) {
                if let Some(tx) = &self._action_tx {
                    let _ = tx.send(action.to_string());
                }
            }

            fn send_action(&self, action: weavetui_core::event::Action) {
                if let Some(tx) = &self._action_tx {
                    let _ = tx.send(action.to_string());
                }
            }

            fn as_active(mut self) -> Self {
                self.set_active(true);
                self
            }

            fn get_children(&mut self) -> &mut weavetui_core::Children {
                &mut self.children
            }
        }


        #default_impl
    };
    expanded.into()
}

