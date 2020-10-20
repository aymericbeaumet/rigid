use quote::TokenStreamExt;

enum Step {
    SkipWhitespaces,
    Quote(syn::export::TokenStream2),
}

impl quote::ToTokens for Step {
    fn to_tokens(&self, tokens: &mut syn::export::TokenStream2) {
        tokens.append_all(match self {
            Self::SkipWhitespaces => quote::quote! {
                idx += ::rigid::runtime::skip_whitespaces(&bytes[idx..]);
            },
            Self::Quote(quote) => quote.clone(),
        });
    }
}

struct Steps(Vec<Step>);

impl Steps {
    const fn new() -> Self {
        Self(Vec::new())
    }

    fn push(&mut self, step: Step) {
        self.0.push(step)
    }
}

impl IntoIterator for Steps {
    type Item = Step;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[proc_macro_derive(FromJSON)]
pub fn derive_from_json(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let typename = &input.ident;
    let steps = get_steps(&input, typename).into_iter();

    proc_macro::TokenStream::from(quote::quote! {
        impl #typename {
            fn from_json(s: &str) -> Result<#typename, ::rigid::Error> {
                let bytes = s.as_bytes();
                let mut idx = 0;

                #(#steps)*

                if idx == s.len() {
                    Ok(ret)
                } else {
                    Err(())
                }
            }
        }
    })
}

fn get_steps(input: &syn::DeriveInput, typename: &syn::Ident) -> Steps {
    let data = match &input.data {
        syn::Data::Struct(s) => s,
        _ => unimplemented!("Only structs are supported at the moment."),
    };
    let fields: Vec<_> = data.fields.iter().collect();

    let mut steps = Steps::new();

    // Make sure we consume all leading whitespaces
    steps.push(Step::SkipWhitespaces);

    // Deserialize into a tuple struct (only when it has a single field)
    if fields.len() == 1 && fields[0].ident.is_none() {
        let eat_fn = get_eat_fn(&fields[0].ty);

        steps.push(Step::Quote(quote::quote! {
            let(delta, out) = ::rigid::runtime::#eat_fn(&bytes[idx..])?;
            idx += delta;

            let ret = #typename(out);
        }));
    }
    // Deserialize into a struct
    else {
        let mut ret_fields = vec![];

        steps.push(Step::Quote(quote::quote! {
            idx += ::rigid::runtime::eat_char(&bytes[idx..], b'{')?;
        }));
        steps.push(Step::SkipWhitespaces);

        for (i, field) in fields.iter().enumerate() {
            match &field.ident {
                Some(ident) => {
                    let eat_fn = get_eat_fn(&field.ty);
                    let ident_string = ident.to_string();

                    steps.push(Step::SkipWhitespaces);
                    steps.push(Step::Quote(quote::quote! {
                        idx += ::rigid::runtime::eat_object_key(&bytes[idx..], #ident_string.as_bytes())?;
                    }));
                    steps.push(Step::SkipWhitespaces);
                    steps.push(Step::Quote(quote::quote! {
                        idx += ::rigid::runtime::eat_char(&bytes[idx..], b':')?;
                    }));
                    steps.push(Step::SkipWhitespaces);
                    steps.push(Step::Quote(quote::quote! {
                        let (delta, #ident) = ::rigid::runtime::#eat_fn(&bytes[idx..])?;
                        idx += delta;
                    }));
                    steps.push(Step::SkipWhitespaces);

                    ret_fields.push(quote::quote! {
                        #ident: #ident
                    });

                    if i < fields.len() - 1 {
                        steps.push(Step::Quote(quote::quote! {
                                idx += ::rigid::runtime::eat_char(&bytes[idx..], b',')?;
                        }));
                        steps.push(Step::SkipWhitespaces);
                    }
                }
                _ => unimplemented!("Tuple struct with multiple fields are not supported."),
            }
        }

        steps.push(Step::Quote(quote::quote! {
            idx += ::rigid::runtime::eat_char(&bytes[idx..], b'}')?;
        }));

        steps.push(Step::Quote(quote::quote! {
            let ret = #typename {
                #(#ret_fields),*
            };
        }));
    };

    // Make sure we consume all trailing whitespaces
    steps.push(Step::SkipWhitespaces);

    steps
}

fn get_eat_fn(ty: &syn::Type) -> syn::Ident {
    quote::format_ident!(
        "eat_{}",
        quote::quote! {#ty}.to_string().to_ascii_lowercase(),
    )
}
