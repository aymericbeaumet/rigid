use proc_macro::TokenStream;

#[proc_macro_derive(FromJSON)]
pub fn derive_from_json(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let typename = &input.ident;
    let steps = get_steps(&input, typename);

    TokenStream::from(quote::quote! {
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

fn get_steps(input: &syn::DeriveInput, typename: &syn::Ident) -> Vec<syn::export::TokenStream2> {
    let data = match &input.data {
        syn::Data::Struct(s) => s,
        _ => unimplemented!("Only structs are supported at the moment."),
    };
    let fields: Vec<_> = data.fields.iter().collect();

    let mut steps = vec![];

    // Make sure we consume all leading whitespaces
    steps.push(quote::quote! {
        idx += ::rigid::runtime::skip_whitespaces(&bytes[idx..]);
    });

    // Deserialize into a tuple struct (only when it has a single field)
    if fields.len() == 1 && fields[0].ident.is_none() {
        let eat_fn = get_eat_fn(&fields[0].ty);

        steps.push(quote::quote! {
            let(delta, out) = ::rigid::runtime::#eat_fn(&bytes[idx..])?;
            idx += delta;

            let ret = #typename(out);
        });
    }
    // Deserialize into a struct
    else {
        let mut ret_fields = vec![];

        steps.push(quote::quote! {
            idx += ::rigid::runtime::eat_char(&bytes[idx..], b'{')?;
            idx += ::rigid::runtime::skip_whitespaces(&bytes[idx..]);
        });

        for (i, field) in fields.iter().enumerate() {
            match &field.ident {
                Some(ident) => {
                    let eat_fn = get_eat_fn(&field.ty);
                    let ident_string = ident.to_string();

                    steps.push(quote::quote! {
                        idx += ::rigid::runtime::skip_whitespaces(&bytes[idx..]);
                        idx += ::rigid::runtime::eat_object_key(&bytes[idx..], #ident_string.as_bytes())?;
                        idx += ::rigid::runtime::skip_whitespaces(&bytes[idx..]);
                        idx += ::rigid::runtime::eat_char(&bytes[idx..], b':')?;
                        idx += ::rigid::runtime::skip_whitespaces(&bytes[idx..]);

                        let (delta, #ident) = ::rigid::runtime::#eat_fn(&bytes[idx..])?;
                        idx += delta;

                        idx += ::rigid::runtime::skip_whitespaces(&bytes[idx..]);
                    });

                    ret_fields.push(quote::quote! {
                        #ident: #ident
                    });

                    if i < fields.len() - 1 {
                        steps.push(quote::quote! {
                            idx += ::rigid::runtime::eat_char(&bytes[idx..], b',')?;
                            idx += ::rigid::runtime::skip_whitespaces(&bytes[idx..]);
                        });
                    }
                }
                _ => unimplemented!("Tuple struct with multiple fields are not supported."),
            }
        }

        steps.push(quote::quote! {
            idx += ::rigid::runtime::eat_char(&bytes[idx..], b'}')?;
        });

        steps.push(quote::quote! {
            let ret = #typename {
                #(#ret_fields),*
            };
        });
    };

    // Make sure we consume all trailing whitespaces
    steps.push(quote::quote! {
        idx += ::rigid::runtime::skip_whitespaces(&bytes[idx..]);
    });

    steps
}

fn get_eat_fn(ty: &syn::Type) -> syn::Ident {
    quote::format_ident!(
        "eat_{}",
        quote::quote! {#ty}.to_string().to_ascii_lowercase(),
    )
}
