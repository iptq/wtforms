use syn;

#[derive(Debug)]
pub(crate) struct FieldOpts {
    name: Option<String>,
    ty: Option<String>,
    id: Option<String>,

    /// The list of attributes for the HTML element.
    extras: Vec<(String, Option<String>)>,
}

impl FieldOpts {
    pub fn new() -> Self {
        FieldOpts {
            name: None,
            ty: None,
            id: None,
            extras: Vec::new(),
        }
    }
    fn set_field(&mut self, (key, value): (String, Option<String>)) {
        match &key[..] {
            "name" => self.name = value,
            "ty" => self.ty = value,
            "id" => self.id = value,
            _ => self.extras.push((key, value)),
        }
    }
    fn push_attribute(&mut self, attr: &syn::NestedMeta) {
        match attr {
            &syn::NestedMeta::Meta(syn::Meta::Word(ref ident)) => {
                let key = quote!(#ident).to_string();
                self.set_field((key, None));
            }
            &syn::NestedMeta::Meta(syn::Meta::NameValue(syn::MetaNameValue {
                ident: ref key_wrap,
                lit: syn::Lit::Str(ref value_wrap),
                ..
            })) => {
                let key = key_wrap.to_string();
                let value = value_wrap.value();
                self.set_field((key, Some(value)));
            }
            other => panic!("unsupported syntax: {}", quote!(#other).to_string()),
        }
    }
    pub fn tokens(&self) -> TokenStream {
    }
    pub fn from(field: &syn::Field) -> FieldOpts {
        let mut opts = FieldOpts::new();
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
            opts.push_attribute(&attr);
        }
        opts
    }
}
