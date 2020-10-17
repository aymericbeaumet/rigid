use proc_macro::TokenStream;

#[proc_macro_derive(FromJSON)]
pub fn derive_from_json(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let typename = &input.ident;
    let steps = get_steps(&input, &typename);

    TokenStream::from(quote::quote! {
        impl #typename {
            fn from_json(s: &str) -> Result<#typename, ::rigid::Error> {
                let bytes = s.as_bytes();
                let mut idx = 0;

                idx += ::rigid::runtime::skip_whitespaces(&bytes[idx..])?;

                #(#steps)*

                idx += ::rigid::runtime::skip_whitespaces(&bytes[idx..])?;

                if idx == s.len() {
                    Ok(ret)
                } else {
                    Err(::rigid::Error::new(
                        "from_json_str found trailing characters that cannot be parsed",
                        &bytes[idx..],
                    ))
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

    // Parse into tuple struct with one field

    if fields.len() == 1 && fields[0].ident.is_none() {
        steps.push(quote::quote! {
            let(delta, out) = ::rigid::runtime::eat_string(&bytes[idx..])?;
            idx += delta;

            let ret = #typename(out);
        });

        return steps;
    }

    // Parse into struct

    {
        let mut ret_fields = vec![];

        steps.push(quote::quote! {
            idx += ::rigid::runtime::eat_char(&bytes[idx..], b'{')?;
            idx += ::rigid::runtime::skip_whitespaces(&bytes[idx..])?;
        });

        for (i, field) in fields.iter().enumerate() {
            match &field.ident {
                Some(ident) => {
                    ret_fields.push(quote::quote! {
                        #ident: #ident
                    });

                    let ident_string = ident.to_string();

                    let field_type = &field.ty;
                    let parsing_method = quote::format_ident!(
                        "eat_object_key_value_{}",
                        quote::quote! {#field_type}.to_string().to_ascii_lowercase(),
                    );

                    steps.push(quote::quote! {
                        let (delta, #ident) = ::rigid::runtime::#parsing_method(
                            &bytes[idx..],
                            #ident_string.as_bytes(),
                        )?;
                        idx += delta;

                        idx += ::rigid::runtime::skip_whitespaces(&bytes[idx..])?;
                    });

                    if i < fields.len() - 1 {
                        steps.push(quote::quote! {
                            idx += ::rigid::runtime::eat_char(&bytes[idx..], b',')?;
                            idx += ::rigid::runtime::skip_whitespaces(&bytes[idx..])?;
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

        return steps;
    }
}
