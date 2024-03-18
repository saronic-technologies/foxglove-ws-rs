use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Foxglove)]
pub fn foxglove(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let expanded = quote! {
        impl Foxglove for #name {
            fn to_jsonschema() -> String {
                serde_json::to_string(&schema_for!(#name)).unwrap()
            }
            fn to_foxglove_schema() -> String {
                String::from(stringify!(#name))
            }
            fn to_foxglove(&self) -> String {
                serde_json::to_string(&self).unwrap()
            }
        }
    };
    TokenStream::from(expanded)
}
