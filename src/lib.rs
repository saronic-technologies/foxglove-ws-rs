pub use foxglove_derive::ToFoxglove;

pub trait Foxglove {
    fn to_jsonschema() -> String;
    fn to_foxglove_schema() -> String;
    fn to_foxglove(&self) -> String;
}
