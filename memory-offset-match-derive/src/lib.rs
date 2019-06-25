extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{
    parse_macro_input, Data::Struct, DataStruct, DeriveInput, Fields, Ident, Lit, Meta, NestedMeta,
};

#[derive(Debug, Clone)]
enum MatchError {
    NotFound
}

type Result<T> = std::result::Result<T, MatchError>;

#[proc_macro_derive(RegexMagic, attributes(byte_pattern, bytes_from))]
pub fn regex_magic_derive(item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as DeriveInput);
    println!("{}", item.ident);

    let item_ident = &item.ident;
    let (impl_generics, ty_generics, where_clause) = item.generics.split_for_impl();

    let regex = item.attrs.iter().flat_map(syn::Attribute::parse_meta)
        .filter_map(|x| match x {
            Meta::List(y) => Some(y),
            _ => None,
        })
        .filter(|x| x.ident == "byte_pattern")
        .flat_map(|x| x.nested.into_iter())
        .filter_map(|x| match x {
            NestedMeta::Literal(Lit::Str(y)) => Some(y.value()),
            _ => None,
        })
        .next()
        .unwrap();

    println!("regex = {:#?}", regex);

    println!("{:#?}", item);

    let regex = memory_offset_match_core::valid_byte_pattern_regex(&regex).unwrap();

    let result = quote! {
        impl #impl_generics #item_ident #ty_generics #where_clause {
            // pub fn regex() -> &memory_offset_match::Regex {
            //     memory_offset_match::lazy_static! {
            //         static ref RE: memory_offset_match::Regex = memory_offset_match::Regex::new(#regex)
            //             .expect("Failed to compile regex");
            //     }
                
            //     *RE
            // }
            // pub fn from_image_base(image_base: usize) -> Option<#item_ident> {

            // }
            // pub fn from_slice(data: &[u8]) -> Result<#item_ident> {
            //     memory_offset_match::lazy_static! {
            //         static ref RE: memory_offset_match::Regex = memory_offset_match::Regex::new(#regex)
            //             .expect("Failed to compile regex");
            //     }

            //     let captures = RE.captures_iter().next().ok_or(MatchError::NotFound)?;
            //     captures.

            // }
        }
    };

    let injector = Ident::new(
        &format!("IMPL_FROMSTUFF_FOR_{}", item.ident.to_string()),
        Span::call_site(),
    );

    let out = quote! {
        // const #injector: () = {
        //     extern crate memory_offset_match;
            #result
        // };
    };

    println!("{}", out.to_string());

    out.into()
}

