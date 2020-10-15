use proc_macro::TokenStream;

#[proc_macro_derive(JSONParser)]
pub fn derive_json_parser(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let typename = input.ident;

    let data = match input.data {
        syn::Data::Struct(s) => s,
        _ => {
            return TokenStream::default();
        }
    };

    let mut steps = vec![];
    let mut ret = vec![];

    steps.push(quote::quote! {
        idx += ::rigid_runtime::eat_whitespaces(&bytes[idx..])?;
        idx += ::rigid_runtime::eat_char(&bytes[idx..], b'{')?;
        idx += ::rigid_runtime::eat_whitespaces(&bytes[idx..])?;
    });

    let fields: Vec<_> = data.fields.iter().collect();
    for (i, field) in fields.iter().enumerate() {
        if let Some(ident) = &field.ident {
            let ident_string = ident.to_string();

            ret.push(quote::quote! {
                #ident: #ident
            });

            steps.push(quote::quote! {
                let (delta, #ident) = ::rigid_runtime::eat_object_key_value(
                    &bytes[idx..],
                    #ident_string.as_bytes(),
                )?;
                idx += delta;
            });

            if i < fields.len() - 1 {
                steps.push(quote::quote! {
                    idx += ::rigid_runtime::eat_char(&bytes[idx..], b',')?;
                    idx += ::rigid_runtime::eat_whitespaces(&bytes[idx..])?;
                });
            }
        }
    }

    steps.push(quote::quote! {
        idx += ::rigid_runtime::eat_whitespaces(&bytes[idx..])?;
        idx += ::rigid_runtime::eat_char(&bytes[idx..], b'}')?;
        idx += ::rigid_runtime::eat_whitespaces(&bytes[idx..])?;
    });

    let from_json_str_impl = quote::quote! {
        impl #typename {
            fn from_json_str(s: &str) -> Result<#typename, String> {
                let bytes = s.as_bytes();
                let mut idx = 0;

                #(#steps)*

                if idx == s.len() {
                    Ok(#typename {
                        #(#ret),*
                    })
                } else {
                    Err(format!(r#"from_json_str found trailing characters that cannot be parsed: "{}""#, &s[idx..]))
                }
            }
        }
    };

    TokenStream::from(from_json_str_impl)
}
