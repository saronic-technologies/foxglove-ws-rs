use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput};

pub trait Foxglovify {
    fn to_jsonschema() -> String;
    fn to_foxglove_schema() -> String;
    fn to_foxglove(&self) -> String;
}

#[proc_macro_derive(Foxglove)]
pub fn foxglove(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let test_mod = format_ident!("test_{}", name);
    let test_fn = format_ident!("check_schema_refs_{}", name);
    let expanded = quote! {
        impl Foxglovify for #name {
            fn to_jsonschema() -> String {
                serde_json::to_string(&schemars::schema_for!(#name)).unwrap()
            }
            fn to_foxglove_schema() -> String {
                String::from(stringify!(#name))
            }
            fn to_foxglove(&self) -> String {
                serde_json::to_string(&self).unwrap()
            }
        }

        // Set up a test case to ensure schema does not contain any JSON references.
        // See: https://docs.foxglove.dev/docs/connecting-to-data/frameworks/custom#json
        #[cfg(test)]
        #[allow(non_snake_case)]
        mod #test_mod {
            use super::#name;

            #[test]
            #[allow(non_snake_case)]
            fn #test_fn() {
                let schema = schemars::schema_for!(#name);
                let schema_json_str = serde_json::to_string(&schema).unwrap();
                eprintln!("{}", serde_json::to_string_pretty(&schema).unwrap());
                assert!(!schema_json_str.contains("$ref"));
            }
        }
    };
    TokenStream::from(expanded)
}
