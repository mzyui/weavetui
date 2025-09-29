//! Procedural macro for the `weavetui` TUI framework.

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

    let mut has_default_derive_initial = false;
    let mut has_debug_derive_initial = false;

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
        .expect("Failed to parse attribute: expected `children(...)` or `default`");
    }

    let mut found_ctx_field = false;
    if let Fields::Named(FieldsNamed { named, .. }) = &mut ast.fields {
        for field in named.iter() {
            if field.ident.as_ref().is_some_and(|ident| ident == "_ctx") {
                found_ctx_field = true;
                break;
            }
        }
        if !found_ctx_field {
            let new_ctx_field: syn::Field =
                parse_quote! { pub _ctx: weavetui_core::ComponentContext };
            named.push(new_ctx_field);
        }
    } else if let Fields::Unnamed(FieldsUnnamed {
        unnamed: _unnamed, ..
    }) = &mut ast.fields
    {
        panic!("#[component] does not support unnamed fields when adding `_ctx` automatically.");
    } else {
        // Unit struct, add named ctx field
        let new_ctx_field: syn::Field = parse_quote! { pub _ctx: weavetui_core::ComponentContext };
        let mut named_fields = syn::punctuated::Punctuated::new();
        named_fields.push(new_ctx_field);
        ast.fields = Fields::Named(FieldsNamed {
            brace_token: syn::token::Brace::default(),
            named: named_fields,
        });
    }

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

    if !has_debug_derive_initial {
        ast.attrs.push(parse_quote! { #[derive(Debug)] });
    }

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
                if field_name == &Ident::new("_ctx", name.span()) {
                    field_initializers.push(quote! { #field_name: weavetui_core::ComponentContext { children: children_map, ..Default::default() } });
                } else {
                    field_initializers.push(quote! { #field_name: Default::default() });
                }
            }
        } else {
            field_initializers.push(quote! { _ctx: weavetui_core::ComponentContext { children: children_map, ..Default::default() } });
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
        let mut field_initializers = Vec::new();
        if let Fields::Named(FieldsNamed { named, .. }) = &ast.fields {
            for field in named.iter() {
                let field_name = field.ident.as_ref().unwrap();
                if field_name == &Ident::new("_ctx", name.span()) {
                    field_initializers
                        .push(quote! { #field_name: weavetui_core::ComponentContext::default() });
                } else {
                    field_initializers.push(quote! { #field_name: Default::default() });
                }
            }
        } else {
            field_initializers.push(quote! { _ctx: weavetui_core::ComponentContext::default() });
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
                    if let Some(area) = self._ctx.area {
                        f.render_widget(
                            ratatui::widgets::Block::bordered()
                                .border_type(ratatui::widgets::BorderType::Rounded)
                                .title_top(ratatui::text::Line::from(format!(" {}: {} x {} ", weavetui_core::ComponentAccessor::name(self), area.height, area.width)))
                                .title_alignment(ratatui::layout::Alignment::Center),
                            area
                        );
                    }

                }
            }
        }
    } else {
        quote! {}
    };

    let expanded = quote! {
        #ast

        #component_impl

        impl weavetui_core::ComponentAccessor for #name {
            fn name(&self) -> String {
                stringify!(#name).to_string()
            }

            fn area(&self) -> Option<ratatui::layout::Rect> {
                self._ctx.area
            }

            fn set_area(&mut self, area: ratatui::layout::Rect) {
                self._ctx.area = Some(area);
            }

            fn is_active(&self) -> bool {
                self._ctx.active
            }

            fn set_active(&mut self, active: bool) {
                self._ctx.active = active;
                (self as &mut dyn weavetui_core::Component).on_active_changed(active);
            }

            fn register_action_handler(&mut self, tx: tokio::sync::mpsc::UnboundedSender<weavetui_core::event::Action>) {
                self._ctx.action_tx = Some(tx);
            }

            fn send(&self, action: &str) {
                if let Some(tx) = &self._ctx.action_tx {
                    let _ = tx.send(weavetui_core::event::Action::AppAction(action.to_string()));
                }
            }

            fn send_action(&self, action: weavetui_core::event::Action) {
                if let Some(tx) = &self._ctx.action_tx {
                    let _ = tx.send(action);
                }
            }

            

            fn get_children(&mut self) -> &mut weavetui_core::Children {
                &mut self._ctx.children
            }

            fn get_theme_manager(&self) -> &weavetui_core::theme::ThemeManager {
                &self._ctx.theme_manager
            }

            fn set_theme_manager(&mut self, theme_manager: weavetui_core::theme::ThemeManager) {
                self._ctx.theme_manager = theme_manager.clone();
            }
        }


        #default_impl
    };
    expanded.into()
}
