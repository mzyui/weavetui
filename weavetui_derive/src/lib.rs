//! Procedural macro for the `weavetui` TUI framework.
//!
//! This crate provides the `#[component]` attribute macro, which automatically implements
//! the `weavetui_core::Component` and `weavetui_core::ComponentAccessor` traits for structs.
//! It simplifies the creation of UI components by handling boilerplate code related to
//! component lifecycle, child management, and action dispatching.

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Fields, FieldsNamed, FieldsUnnamed, Ident, ItemStruct, parse::ParseStream, parse_macro_input,
    parse_quote, punctuated::Punctuated,
};
mod args;

/// Implements the `weavetui_core::Component` and `weavetui_core::ComponentAccessor` traits
/// for a struct, turning it into a `weavetui` component.
///
/// This macro simplifies component creation by:
/// - Automatically injecting necessary fields: `children`, `_area`, `_active`, `_action_tx`, and `_theme_manager`.
/// - Generating a `Default` implementation that initializes children.
/// - Providing default implementations for the `ComponentAccessor` trait.
///
/// # Attributes
///
/// - `#[component(children = [child_name => ChildType, ...])]`: Defines child components.
///   The macro will add a `children` field of type `BTreeMap<String, Box<dyn Component>>`
///   and initialize it with the specified children in the `Default` implementation.
///
///   - `child_name`: A string literal representing the key for the child.
///   - `ChildType`: The type of the child component, which must implement `Default`.
///
/// - `#[component(default)]`: Generates a default `draw` method implementation for the
///   `Component` trait. This is useful for placeholder components or for quickly
///   visualizing the component's area. The default `draw` method renders a bordered
///   block with the component's name and dimensions.
///
/// # Injected Fields
///
/// When you use the `#[component]` attribute, the following fields are automatically added to your struct if they are not already present:
///
/// - `pub children: BTreeMap<String, Box<dyn Component>>`: A map to hold child components, allowing for nested UI structures.
/// - `_area: Option<ratatui::layout::Rect>`: Stores the rendering area assigned to the component by its parent.
/// - `_active: bool`: A flag indicating whether the component is currently active and should respond to events.
/// - `_action_tx: Option<UnboundedSender<Action>>`: A channel sender for dispatching actions to the application's central event loop.
/// - `_theme_manager: weavetui_core::theme::ThemeManager`: Manages the theme and styles for the component and its children.
///
/// # Example
///
/// ```rust,ignore
/// use weavetui::prelude::*; // Includes necessary traits and macros
/// use ratatui::prelude::*;
///
/// #[component(default)]
/// pub struct Child;
///
/// #[component(children = [ "child" => Child ])]
/// #[derive(Default)]
/// pub struct MyComponent {
///     pub counter: i32,
/// }
///
/// impl Component for MyComponent {
///     fn draw(&mut self, f: &mut Frame, area: Rect) {
///         // Custom draw logic here
///     }
/// }
///
/// fn main() {
///     let my_component = MyComponent::default();
///     assert_eq!(my_component.children.len(), 1);
///     assert!(my_component.children.contains_key("child"));
/// }
/// ```
#[proc_macro_attribute]
pub fn component(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(item as ItemStruct);

    let name = &ast.ident;

    let mut children_entries: Option<Punctuated<args::ChildEntry, syn::token::Comma>> = None;
    let mut default_component_impl = false;

    let mut has_default_derive_initial = false;
    let mut has_debug_derive_initial = false;

    // Parse the attributes provided to the `#[component]` macro.
    // This block handles attributes like `children(...)` and `default`.
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

    let actual_children_type =
        quote! { std::collections::BTreeMap<String, Box<dyn weavetui_core::Component>> };

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

        // Add _area field if not present
        if !named
            .iter()
            .any(|f| f.ident.as_ref().is_some_and(|i| i == "_area"))
        {
            named.push(parse_quote! { _area: Option<ratatui::layout::Rect> });
        }

        // Add _active field if not present
        if !named
            .iter()
            .any(|f| f.ident.as_ref().is_some_and(|i| i == "_active"))
        {
            named.push(parse_quote! { _active: bool });
        }
        // Add _action_tx field if not present
        if !named
            .iter()
            .any(|f| f.ident.as_ref().is_some_and(|i| i == "_action_tx"))
        {
            named.push(
                parse_quote! { _action_tx: Option<tokio::sync::mpsc::UnboundedSender<weavetui_core::event::Action>> },
            );
        }
        if !named
            .iter()
            .any(|f| f.ident.as_ref().is_some_and(|i| i == "_theme_manager"))
        {
            named.push(parse_quote! { _theme_manager: weavetui_core::theme::ThemeManager });
        }
    } else if let Fields::Unnamed(FieldsUnnamed {
        unnamed: _unnamed, ..
    }) = &mut ast.fields
    {
        panic!("#[component] does not support unnamed fields when adding children automatically.");
    } else {
        // Unit struct, add named children field
        let new_children_field: syn::Field = parse_quote! { pub children: #actual_children_type };
        let mut named_fields = syn::punctuated::Punctuated::new();
        named_fields.push(new_children_field);
        named_fields.push(parse_quote! { _area: Option<ratatui::layout::Rect> });
        named_fields.push(parse_quote! { _active: bool });
        named_fields.push(
            parse_quote! { _action_tx: Option<tokio::sync::mpsc::UnboundedSender<weavetui_core::event::Action>> },
        );
        named_fields.push(parse_quote! { _theme_manager: weavetui_core::theme::ThemeManager });
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

    // Remove any existing `derive(Default)` and `derive(Debug)` attributes from the struct.
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

    // Add `derive(Debug)` back to the struct if it wasn't already present.
    if !has_debug_derive_initial {
        ast.attrs.push(parse_quote! { #[derive(Debug)] });
    }

    // Generate the `Default` implementation for the struct.
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
                    field_initializers.push(quote! { #field_name: children_map });
                } else {
                    field_initializers.push(quote! { #field_name: Default::default() });
                }
            }
        } else {
            field_initializers.push(quote! { children: children_map });
            field_initializers.push(quote! { _area: Default::default() });
            field_initializers.push(quote! { _active: Default::default() });
            field_initializers.push(quote! { _action_tx: Default::default() });
            field_initializers.push(quote! { _theme_manager: Default::default() });
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
                if field_name == &children_field_name {
                    field_initializers
                        .push(quote! { #field_name: std::collections::BTreeMap::new() });
                } else {
                    field_initializers.push(quote! { #field_name: Default::default() });
                }
            }
        } else {
            field_initializers.push(quote! { children: std::collections::BTreeMap::new() });
            field_initializers.push(quote! { _area: Default::default() });
            field_initializers.push(quote! { _active: Default::default() });
            field_initializers.push(quote! { _action_tx: Default::default() });
            field_initializers.push(quote! { _theme_manager: Default::default() });
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

    // Generate the `Component` trait implementation.
    let component_impl = if default_component_impl {
        quote! {
            impl weavetui_core::Component for #name {
                fn draw(&mut self, f: &mut ratatui::Frame<'_>, area: ratatui::layout::Rect) {
                    if let Some(area) = self.area() {
                        f.render_widget(
                            ratatui::widgets::Block::bordered()
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

    // Generate the `ComponentAccessor` trait implementation.
    let expanded = quote! {
        #ast // The modified struct definition

        #component_impl

        impl weavetui_core::ComponentAccessor for #name {
            fn name(&self) -> String {
                stringify!(#name).to_string()
            }

            fn area(&self) -> Option<ratatui::layout::Rect> {
                self._area
            }

            fn set_area(&mut self, area: ratatui::layout::Rect) {
                self._area = Some(area);
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

            fn register_action_handler(&mut self, tx: tokio::sync::mpsc::UnboundedSender<weavetui_core::event::Action>) {
                self._action_tx = Some(tx);
            }

            fn send(&self, action: &str) {
                if let Some(tx) = &self._action_tx {
                    let _ = tx.send(weavetui_core::event::Action::AppAction(action.to_string()));
                }
            }

            fn send_action(&self, action: weavetui_core::event::Action) {
                if let Some(tx) = &self._action_tx {
                    let _ = tx.send(action);
                }
            }

            fn as_active(mut self) -> Self {
                self.set_active(true);
                self
            }

            fn get_children(&mut self) -> &mut weavetui_core::Children {
                &mut self.children
            }

            fn get_theme_manager(&self) -> &weavetui_core::theme::ThemeManager {
                &self._theme_manager
            }

            fn set_theme_manager(&mut self, theme_manager: weavetui_core::theme::ThemeManager) {
                self._theme_manager = theme_manager.clone();
            }
        }


        #default_impl
    };
    expanded.into()
}
