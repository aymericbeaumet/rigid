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
        idx += ::rigid::runtime::skip_whitespaces(&bytes[idx..])?;
        idx += ::rigid::runtime::eat_char(&bytes[idx..], b'{')?;
        idx += ::rigid::runtime::skip_whitespaces(&bytes[idx..])?;
    });

    let fields: Vec<_> = data.fields.iter().collect();
    for (i, field) in fields.iter().enumerate() {
        if let Some(ident) = &field.ident {
            let ident_string = ident.to_string();

            let field_type = &field.ty;
            let parsing_method = quote::format_ident!(
                "eat_object_key_value_{}",
                quote::quote! {#field_type}.to_string().to_ascii_lowercase(),
            );

            ret.push(quote::quote! {
                #ident: #ident
            });

            steps.push(quote::quote! {
                let (delta, #ident) = ::rigid::runtime::#parsing_method(
                    &bytes[idx..],
                    #ident_string.as_bytes(),
                )?;
                idx += delta;
            });

            if i < fields.len() - 1 {
                steps.push(quote::quote! {
                    idx += ::rigid::runtime::skip_whitespaces(&bytes[idx..])?;
                    idx += ::rigid::runtime::eat_char(&bytes[idx..], b',')?;
                    idx += ::rigid::runtime::skip_whitespaces(&bytes[idx..])?;
                });
            }
        }
    }

    steps.push(quote::quote! {
        idx += ::rigid::runtime::skip_whitespaces(&bytes[idx..])?;
        idx += ::rigid::runtime::eat_char(&bytes[idx..], b'}')?;
        idx += ::rigid::runtime::skip_whitespaces(&bytes[idx..])?;
    });

    let from_json_impl = quote::quote! {
        impl #typename {
            fn from_json(s: &str) -> Result<#typename, ::rigid::Error> {
                let bytes = s.as_bytes();
                let mut idx = 0;

                #(#steps)*

                if idx == s.len() {
                    Ok(#typename {
                        #(#ret),*
                    })
                } else {
                    Err(::rigid::Error::new(
                        "from_json_str found trailing characters that cannot be parsed",
                        &bytes[idx..],
                    ))
                }
            }
        }
    };

    TokenStream::from(from_json_impl)
}
