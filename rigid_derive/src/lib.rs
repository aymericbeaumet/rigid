use proc_macro::TokenStream;

#[proc_macro_derive(FromJSON)]
pub fn derive_from_json(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let typename = input.ident;

    let data = match input.data {
        syn::Data::Struct(s) => s,
        _ => unimplemented!("Only structs are supported at the moment."),
    };

    let mut steps = vec![];
    let mut ret = vec![];

    steps.push(quote::quote! {
        idx += ::rigid::runtime::eat_whitespaces(&bytes[idx..])?;
        idx += ::rigid::runtime::eat_char(&bytes[idx..], b'{')?;
        idx += ::rigid::runtime::eat_whitespaces(&bytes[idx..])?;
    });

    let fields: Vec<_> = data.fields.iter().collect();
    for (i, field) in fields.iter().enumerate() {
        if let Some(ident) = &field.ident {
            let ident_string = ident.to_string();

            ret.push(quote::quote! {
                #ident: #ident
            });

            steps.push(quote::quote! {
                let (delta, #ident) = ::rigid::runtime::eat_object_key_value(
                    &bytes[idx..],
                    #ident_string.as_bytes(),
                )?;
                idx += delta;
            });

            if i < fields.len() - 1 {
                steps.push(quote::quote! {
                    idx += ::rigid::runtime::eat_char(&bytes[idx..], b',')?;
                    idx += ::rigid::runtime::eat_whitespaces(&bytes[idx..])?;
                });
            }
        }
    }

    steps.push(quote::quote! {
        idx += ::rigid::runtime::eat_whitespaces(&bytes[idx..])?;
        idx += ::rigid::runtime::eat_char(&bytes[idx..], b'}')?;
        idx += ::rigid::runtime::eat_whitespaces(&bytes[idx..])?;
    });

    let from_json_impl = quote::quote! {
        impl #typename {
            fn from_json(s: &str) -> Result<#typename, ::rigid::runtime::Error> {
                let bytes = s.as_bytes();
                let mut idx = 0;

                #(#steps)*

                if idx == s.len() {
                    Ok(#typename {
                        #(#ret),*
                    })
                } else {
                    Err(format!(r#"from_json_str found trailing characters that cannot be parsed: "{}""#, &s[idx..]).into())
                }
            }
        }
    };

    TokenStream::from(from_json_impl)
}
