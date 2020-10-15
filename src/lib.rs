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
                idx += eat_char(&bytes[idx..], b'{')?;
                idx += eat_whitespaces(&bytes[idx..])?;
    });

    steps.push(quote::quote! {
                let (delta, age) = eat_object_key_value(&bytes[idx..], "age".as_bytes())?;
                idx += delta;
    });

    steps.push(quote::quote! {
                idx += eat_whitespaces(&bytes[idx..])?;
                idx += eat_char(&bytes[idx..], b'}')?;
    });

    let from_json_str_impl = quote::quote! {
        impl #typename {
            fn from_json_str(s: &str) -> Result<#typename, ()> {
                let bytes = s.as_bytes();
                let mut idx = 0;

                idx += eat_whitespaces(&bytes[idx..])?;

                #(#steps)*

                idx += eat_whitespaces(&bytes[idx..])?;

                if idx == s.len() {
                    Ok(#typename { age: age })
                } else {
                    Err(())
                }
            }
        }

        fn eat_whitespaces(bytes: &[u8]) -> Result<usize, ()> {
            let mut idx = 0;
            while idx < bytes.len() && bytes[idx] == b' ' {
                idx += 1;
            }
            Ok(idx)
        }

        fn eat_char(bytes: &[u8], c: u8) -> Result<usize, ()> {
            if bytes.len() >= 1 && bytes[0] == c {
                Ok(1)
            } else {
                Err(())
            }
        }

        fn eat_object_key_value(bytes: &[u8], k: &[u8]) -> Result<(usize, u8), ()> {
            let mut idx = 0;
            idx += eat_object_key(&bytes[idx..], k)?;
            idx += eat_whitespaces(&bytes[idx..])?;
            idx += eat_char(&bytes[idx..], b':')?;
            idx += eat_whitespaces(&bytes[idx..])?;

            let (delta, value) = eat_number(&bytes[idx..])?;
            idx += delta;

            Ok((idx, value))
        }

        fn eat_object_key(bytes: &[u8], k: &[u8]) -> Result<usize, ()> {
            let mut idx = 0;
            idx += eat_char(&bytes[idx..], b'"')?;
            idx += eat_slice(&bytes[idx..], k)?;
            idx += eat_char(&bytes[idx..], b'"')?;
            Ok(idx)
        }

        fn eat_number(bytes: &[u8]) -> Result<(usize, u8), ()> {
            let mut idx = 0;
            let mut out: u8 = 0;
            while idx < bytes.len() && bytes[idx] >= b'0' && bytes[idx] <= b'9' {
                out = out * 10 + (bytes[idx] - b'0');
                idx += 1;
            }
            if idx > 0 {
                Ok((idx, out))
            } else {
                Err(())
            }
        }

        fn eat_slice(bytes: &[u8], s: &[u8]) -> Result<usize, ()> {
            if bytes.starts_with(s) {
                Ok(s.len())
            } else {
                Err(())
            }
        }
    };

    TokenStream::from(from_json_str_impl)
}
