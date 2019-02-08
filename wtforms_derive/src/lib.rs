#![recursion_limit = "128"]

extern crate proc_macro;

mod validators;

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::quote;
use syn::{AttrStyle, Fields, Meta};

use crate::validators::parse_validator;

#[proc_macro_derive(Form, attributes(field, validators))]
pub fn hello(item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemStruct);
    let name = &input.ident;
    let mod_name = Ident::new(&format!("__wtforms_{}", name), Span::call_site());
    let validator_name = Ident::new(&format!("__Form_{}_Validator", name), Span::call_site());

    let fields = match &input.fields {
        Fields::Named(named_fields) => &named_fields.named,
        _ => panic!("cannot use tuple or unit struct here"),
    };

    let mut as_html = TokenStream2::new();
    let mut validator_types = TokenStream2::new();
    let mut validators = TokenStream2::new();
    for field in fields {
        let mut field_validator_types = TokenStream2::new();
        let mut field_validators = TokenStream2::new();
        let field_type = &field.ty;
        if let Some(field_name) = &field.ident {
            let field_name_str = field_name.to_string();
            as_html.extend(quote! {
                let _ = <#field_type as ::wtforms::html::AsHtml>::as_html(&mut contents, #field_name_str, Some(()));
            });
            for attr in &field.attrs {
                if let AttrStyle::Outer = attr.style {
                    if let Meta::List(list) = attr.parse_meta().expect("fuck you") {
                        for item in list.nested {
                            let (ty, val) = parse_validator(&item);
                            field_validator_types.extend(quote!(#ty,));
                            field_validators.extend(quote!(#val,));
                        }
                    }
                }
            }
        }
        validator_types.extend(quote!(Hlist![#field_validator_types],));
        validators.extend(quote!(hlist![#field_validators],));
    }
    // panic!("{}\n{}", validator_types, validators);

    let tokens = quote! {
        mod #mod_name {
            use ::wtforms::_frunk::{Hlist, hlist};
            static #validator_name: Hlist![#validator_types] = hlist![#validators];
        }

        impl ::wtforms::Form for #name {
            type Error = ();

            fn as_html() -> String {
                let mut contents = String::new();
                #as_html
                contents
            }

            fn validate(self) -> Result<Self, Self::Error> {
                Ok(self)
            }
        }
    };
    // panic!("{}", tokens);
    tokens.into()
}
