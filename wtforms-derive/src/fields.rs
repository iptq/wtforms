use syn;

#[derive(Debug)]
pub struct FieldOpts {
    /// The list of attributes for the HTML element.
    attrs: Vec<(String, String)>,
}

impl FieldOpts {
    pub fn new() -> Self {
        FieldOpts { attrs: Vec::new() }
    }
    pub fn push_attribute(&mut self, attr: &syn::NestedMeta) {
        match attr {
            &syn::NestedMeta::Meta(syn::Meta::NameValue(syn::MetaNameValue {
                ident: ref key_wrap,
                lit: syn::Lit::Str(ref value_wrap),
                ..
            })) => {
                let key = key_wrap.to_string();
                let value = value_wrap.value();
                self.attrs.push((key, value));
            }
            other => panic!("unsupported syntax: {}", quote!(#other).to_string()),
        }
    }
    pub fn push_field(&mut self, field: &syn::Field) {
        for attr in field
            .attrs
            .iter()
            .filter_map(|attr| {
                // make sure it's in the form #[field(key = "value")]
                let path = &attr.path;
                match quote!(#path).to_string() == "field" {
                    true => attr.interpret_meta(),
                    false => None,
                }
            })
            .flat_map(|m| match m {
                // get rid of the field(..) wrapper
                syn::Meta::List(l) => l.nested,
                tokens => panic!(
                    "expecting list field(..), found {}",
                    quote!(#tokens).to_string()
                ),
            }) {
            self.push_attribute(&attr);
        }
    }
}
