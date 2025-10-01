//! Procedural macro for the `weavetui` TUI framework.

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Fields, FieldsNamed, FieldsUnnamed, Ident, ItemStruct, parse::ParseStream, parse_macro_input,
    parse_quote, punctuated::Punctuated, Type, Expr,
};
mod args;

#[proc_macro_attribute]
pub fn component(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(item as ItemStruct);

    let name = &ast.ident;
    let input = proc_macro2::TokenStream::from(attr);

    let mut children_entries: Option<Punctuated<args::ChildEntry, syn::token::Comma>> = None;
    let mut default_component_impl = false;

    // Redux-specific attributes
    let mut state_type: Option<Type> = None;
    let mut action_type: Option<Type> = None;
    let mut reducer_fn: Option<Expr> = None;
    let mut is_redux_component = false;

    let mut has_default_derive_initial = false;
    let mut has_debug_derive_initial = false;

    if !input.is_empty() {
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
                    } else if lookahead.peek(syn::Ident) && input.peek2(syn::token::Eq) {
                        // Redux attributes
                        let ident: Ident = input.parse()?;

                        if ident == "state" {
                            input.parse::<syn::token::Eq>()?;
                            state_type = Some(input.parse()?);
                            is_redux_component = true;
                        } else if ident == "action" {
                            input.parse::<syn::token::Eq>()?;
                            action_type = Some(input.parse()?);
                            is_redux_component = true;
                        } else if ident == "reducer" {
                            input.parse::<syn::token::Eq>()?;
                            reducer_fn = Some(input.parse()?);
                            is_redux_component = true;
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
            input,
        )
        .expect("Failed to parse attribute");
    }

    // Validate Redux attributes if this is a Redux component
    if is_redux_component {
        if state_type.is_none() {
            panic!("Redux component requires 'state = StateType' attribute");
        }
        if action_type.is_none() {
            panic!("Redux component requires 'action = ActionType' attribute");
        }
        if reducer_fn.is_none() {
            panic!("Redux component requires 'reducer = reducer_function' attribute");
        }
    }

    // Add necessary fields to the struct
    let mut found_ctx_field = false;
    let mut found_store_field = false;
    let mut found_state_field = false;

    if let Fields::Named(FieldsNamed { named, .. }) = &mut ast.fields {
        for field in named.iter() {
            if let Some(ident) = &field.ident {
                if ident == "_ctx" {
                    found_ctx_field = true;
                } else if ident == "store_connection" {
                    found_store_field = true;
                } else if ident == "current_state" {
                    found_state_field = true;
                }
            }
        }

        if !found_ctx_field {
            let new_ctx_field: syn::Field =
                parse_quote! { pub _ctx: weavetui_core::ComponentContext };
            named.push(new_ctx_field);
        }

        // Add Redux fields if this is a Redux component
        if is_redux_component {
            let state_ty = state_type.as_ref().unwrap();
            let action_ty = action_type.as_ref().unwrap();

            if !found_store_field {
                let new_store_field: syn::Field = parse_quote! {
                    store_connection: Option<weavetui_core::redux::StoreConnection<#state_ty, #action_ty>>
                };
                named.push(new_store_field);
            }

            if !found_state_field {
                let new_state_field: syn::Field = parse_quote! {
                    current_state: #state_ty
                };
                named.push(new_state_field);
            }
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

        // Add Redux fields for unit struct if needed
        if is_redux_component {
            let state_ty = state_type.as_ref().unwrap();
            let action_ty = action_type.as_ref().unwrap();

            let store_field: syn::Field = parse_quote! {
                store_connection: Option<weavetui_core::redux::StoreConnection<#state_ty, #action_ty>>
            };
            named_fields.push(store_field);

            let state_field: syn::Field = parse_quote! {
                current_state: #state_ty
            };
            named_fields.push(state_field);
        }

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

    // Generate Redux methods if this is a Redux component
    let redux_methods = if is_redux_component {
        let state_ty = state_type.as_ref().unwrap();
        let action_ty = action_type.as_ref().unwrap();
        let reducer = reducer_fn.as_ref().unwrap();

        quote! {
            impl #name {
                /// Create a new Redux component with store
                pub fn new(store: weavetui_core::redux::Store<#state_ty, #action_ty>) -> Self {
                    let store_connection = weavetui_core::redux::StoreConnection::new(store);
                    let current_state = store_connection.store().get_state();

                    Self {
                        store_connection: Some(store_connection),
                        current_state,
                        _ctx: Default::default(),
                        ..Default::default()
                    }
                }

                /// Create a new Redux component with initial state (creates store internally)
                pub fn with_initial_state(initial_state: #state_ty) -> Self {
                    let store = weavetui_core::redux::Store::new(initial_state, #reducer);
                    Self::new(store)
                }

                /// Dispatch an action to the Redux store
                pub fn dispatch(&self, action: #action_ty) {
                    if let Some(ref connection) = self.store_connection {
                        connection.store().dispatch(&action);
                    }
                }

                /// Get reference to the Redux store
                pub fn store(&self) -> Option<&weavetui_core::redux::Store<#state_ty, #action_ty>> {
                    self.store_connection.as_ref().map(|conn| conn.store())
                }

                /// Update component state from Redux store
                pub fn update_from_store(&mut self) {
                    if let Some(ref mut connection) = self.store_connection {
                        if let Some(new_state) = connection.try_recv_state() {
                            self.current_state = new_state;
                        }
                    }
                }

                /// Get current state
                pub fn state(&self) -> &#state_ty {
                    &self.current_state
                }
            }
        }
    } else {
        quote! {}
    };

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
                } else if is_redux_component && (field_name == "store_connection" || field_name == "current_state") {
                    // Skip Redux fields in default impl - they'll be handled by new() method
                    if field_name == "store_connection" {
                        field_initializers.push(quote! { #field_name: None });
                    } else {
                        field_initializers.push(quote! { #field_name: Default::default() });
                    }
                } else {
                    field_initializers.push(quote! { #field_name: Default::default() });
                }
            }
        } else {
            field_initializers.push(quote! { _ctx: weavetui_core::ComponentContext { children: children_map, ..Default::default() } });
            if is_redux_component {
                field_initializers.push(quote! { store_connection: None });
                field_initializers.push(quote! { current_state: Default::default() });
            }
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
                } else if is_redux_component && (field_name == "store_connection" || field_name == "current_state") {
                    // Skip Redux fields in default impl
                    if field_name == "store_connection" {
                        field_initializers.push(quote! { #field_name: None });
                    } else {
                        field_initializers.push(quote! { #field_name: Default::default() });
                    }
                } else {
                    field_initializers.push(quote! { #field_name: Default::default() });
                }
            }
        } else {
            field_initializers.push(quote! { _ctx: weavetui_core::ComponentContext::default() });
            if is_redux_component {
                field_initializers.push(quote! { store_connection: None });
                field_initializers.push(quote! { current_state: Default::default() });
            }
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

        #redux_methods

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
