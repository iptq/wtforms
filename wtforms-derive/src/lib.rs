#[macro_use]
extern crate failure;
extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;
#[macro_use]
extern crate quote;

mod fields;

use proc_macro2::TokenStream;

use fields::FieldOpts;

#[proc_macro_derive(Form, attributes(field))]
pub fn form_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse(input).unwrap();
    let gen = impl_form(&input);
    gen.into()
}

fn impl_form(input: &syn::DeriveInput) -> TokenStream {
    if let syn::Data::Struct(syn::DataStruct { ref fields, .. }) = input.data {
        let cls = &input.ident;
        let mut stream = TokenStream::new();
        for field in fields {
            let field_id = &field.ident;
            let opt = FieldOpts::from(input, field);
            stream.extend(quote!(fields_render.push(self.#field_id.render()?);));
        }
        println!("{}", stream);
        quote! {
            impl ::wtforms::Render for #cls {
                fn render(&self) -> Result<String, Error> {
                    let mut fields_render = Vec::new();
                    #stream
                    Ok(format!("<form>{}</form>", fields_render.join("")))
                }
            }
        }
    } else {
        panic!("#[derive(Form)] should only be used with structs");
    }
}
