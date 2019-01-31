#![recursion_limit = "128"]

extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::quote;
use syn::{AttrStyle, Fields, Lit, Meta, NestedMeta};

fn parse_validator(meta: &NestedMeta) -> (TokenStream2, TokenStream2) {
    use Meta::*;
    use NestedMeta::{Literal as L, Meta as M};
    match meta {
        M(Word(name)) if name == "required" => (
            quote!(::wtforms::validators::Required),
            quote!(::wtforms::validators::Required),
        ),
        M(List(list)) if list.ident == "length" => {
            let mut min = 0u32;
            let max: u32;
            match list.nested.len() {
                1 => match &list.nested[0] {
                    L(Lit::Int(val)) => max = val.value() as u32,
                    _ => unreachable!(),
                },
                2 => match (&list.nested[0], &list.nested[1]) {
                    (L(Lit::Int(val0)), L(Lit::Int(val1))) => {
                        min = val0.value() as u32;
                        max = val1.value() as u32;
                    }
                    _ => unreachable!(),
                },
                _ => panic!("length should only contain 1 or 2 elements"),
            }
            (
                quote!(::wtforms::validators::Length),
                quote!(::wtforms::validators::Length(#min, #max)),
            )
        }
        _ => (
            quote!(::wtforms::validators::Required),
            quote!(::wtforms::validators::Required),
        ),
        _ => panic!("invalid meta: {} {:?}", quote!(#meta), quote!(#meta)),
    }
}

#[proc_macro_derive(Form, attributes(field, validators))]
pub fn hello(item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemStruct);
    let name = &input.ident;
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
                <#field_type as ::wtforms::html::AsHtml>::as_html(&mut contents, #field_name_str, Some(()));
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
        mod foo {
            use frunk::{Hlist, hlist};
            static #validator_name: Hlist![#validator_types] = hlist![#validators];
        }

        impl ::wtforms::Form for #name {
            type Error = ();

            fn as_html() -> String {
                let mut contents = String::new();
                #as_html
                contents
            }

            fn validate(&self) -> Result<(), Self::Error> {
                Err(())
            }
        }
    };
    // panic!("{}", tokens);
    tokens.into()
}
