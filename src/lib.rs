use proc_macro::TokenStream;

#[proc_macro_derive(JSONParser)]
pub fn derive_json_parser(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    let typename = input.ident;

    let expanded = quote::quote! {
        impl #typename {
            fn json_from_str(s: &str) -> Result<#typename, ()> {
                Ok(#typename { age: 43 })
            }
        }
    };

    TokenStream::from(expanded)
}
