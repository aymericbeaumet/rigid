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

    steps.push(quote::quote! {
        idx += eat_whitespaces(&bytes[idx..])?;
        idx += eat_char(&bytes[idx..], b'{')?;
        idx += eat_whitespaces(&bytes[idx..])?;
    });

    let fields: Vec<_> = data.fields.iter().collect();
    for (i, field) in fields.iter().enumerate() {
        if let Some(ident) = &field.ident {
            let ident_string = ident.to_string();
            steps.push(quote::quote! {
                let (delta, #ident) = eat_object_key_value(
                    &bytes[idx..],
                    #ident_string.as_bytes(),
                )?;
                idx += delta;
            });

            if i < fields.len() - 1 {
                steps.push(quote::quote! {
                    idx += eat_char(&bytes[idx..], b',')?;
                    idx += eat_whitespaces(&bytes[idx..])?;
                });
            }
        }
    }

    steps.push(quote::quote! {
        idx += eat_whitespaces(&bytes[idx..])?;
        idx += eat_char(&bytes[idx..], b'}')?;
        idx += eat_whitespaces(&bytes[idx..])?;
    });

    let from_json_str_impl = quote::quote! {
        impl #typename {
            fn from_json_str(s: &str) -> Result<#typename, String> {
                let bytes = s.as_bytes();
                let mut idx = 0;

                #(#steps)*

                if idx == s.len() {
                    Ok(#typename { age: age, height: height })
                } else {
                    Err(format!(r#"from_json_str found trailing characters that cannot be parsed: "{}""#, &s[idx..]))
                }
            }
        }

        fn eat_whitespaces(bytes: &[u8]) -> Result<usize, String> {
            let mut idx = 0;
            while idx < bytes.len() && bytes[idx] == b' ' {
                idx += 1;
            }
            Ok(idx)
        }

        fn eat_char(bytes: &[u8], c: u8) -> Result<usize, String> {
            if bytes.len() >= 1 && bytes[0] == c {
                Ok(1)
            } else {
                Err(format!("eat_char expected {} ({}) but found {} ({})", c, c as char, bytes[0], bytes[0] as char))
            }
        }

        fn eat_object_key_value(bytes: &[u8], k: &[u8]) -> Result<(usize, u8), String> {
            let mut idx = 0;
            idx += eat_object_key(&bytes[idx..], k)?;
            idx += eat_whitespaces(&bytes[idx..])?;
            idx += eat_char(&bytes[idx..], b':')?;
            idx += eat_whitespaces(&bytes[idx..])?;

            let (delta, value) = eat_number(&bytes[idx..])?;
            idx += delta;

            Ok((idx, value))
        }

        fn eat_object_key(bytes: &[u8], k: &[u8]) -> Result<usize, String> {
            let mut idx = 0;
            idx += eat_char(&bytes[idx..], b'"')?;
            idx += eat_slice(&bytes[idx..], k)?;
            idx += eat_char(&bytes[idx..], b'"')?;
            Ok(idx)
        }

        fn eat_number(bytes: &[u8]) -> Result<(usize, u8), String> {
            let mut idx = 0;
            let mut out: u8 = 0;
            while idx < bytes.len() && bytes[idx] >= b'0' && bytes[idx] <= b'9' {
                out = out * 10 + (bytes[idx] - b'0');
                idx += 1;
            }
            if idx > 0 {
                Ok((idx, out))
            } else {
                Err(String::from("eat_number couldn't find any digit"))
            }
        }

        fn eat_slice(bytes: &[u8], s: &[u8]) -> Result<usize, String> {
            if bytes.starts_with(s) {
                Ok(s.len())
            } else {
                Err(format!("eat_slice could not match {:?} as a prefix of {:?}", s, bytes))
            }
        }
    };

    TokenStream::from(from_json_str_impl)
}
