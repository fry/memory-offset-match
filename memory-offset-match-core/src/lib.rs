#[macro_use]
extern crate darling;

mod pattern_parse;
mod build_struct;

pub use pattern_parse::*;
pub use build_struct::*;


#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_str;
    use quote::quote;
    use darling::FromDeriveInput;

    #[test]
    fn thing() {
        let code = r#"#[derive(BytePattern)]
#[byte_match(pattern = "13 37 (yourPointer: ????) 66 66")]
struct YourPattern {
    #[bytes_from(endianness = "be")]
    your_pointer: usize,
}"#;

        let parsed = parse_str(code).unwrap();
        let opts = BytePatternTraitOpts::from_derive_input(&parsed).unwrap();
        let tokens = quote!(#opts);

        println!("{:?}", opts);
        println!("{}", tokens);

    }
}
