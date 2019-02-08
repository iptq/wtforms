use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Lit, Meta, NestedMeta};

pub fn parse_validator(meta: &NestedMeta) -> (TokenStream2, TokenStream2) {
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
