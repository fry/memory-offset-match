extern crate proc_macro2;

use proc_macro2::{TokenStream, Span};
use quote::{ToTokens, quote};
use syn::{
    parse_macro_input, Data::Struct, DataStruct, DeriveInput, Fields, Ident, Lit, Meta, NestedMeta,
};

use darling::FromDeriveInput;
use darling::ast;


#[derive(Debug, FromDeriveInput)]
#[darling(attributes(byte_match), supports(struct_any))]
pub struct BytePatternTraitOpts {
    ident: syn::Ident,
    generics: syn::Generics,
    pattern: String,
    data: ast::Data<(), BytePatternFieldOpts>
}

#[derive(Debug, Clone, Copy, FromMeta)]
#[darling(default)]
pub enum Endianness {
    #[darling(rename = "be")]
    BE, 
    #[darling(rename = "le")]
    LE
}

impl Default for Endianness {
    fn default() -> Self {
        Endianness::LE
    }
}

#[derive(Debug, FromField)]
#[darling(attributes(bytes_from))]
pub struct BytePatternFieldOpts {
    ident: Option<syn::Ident>,
    ty: syn::Type,
    #[darling(default)]
    endianness: Endianness
}

#[derive(Debug, Clone)]
enum MatchError {
    NotFound
}

type Result<T> = std::result::Result<T, MatchError>;

impl ToTokens for BytePatternTraitOpts {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        println!("{}", self.ident);

        let ident = &self.ident;
        let (impl_generics, ty_generics, where_clause) = self.generics.split_for_impl();

        println!("pattern = {:#?}", self.pattern);

        // if let Struct(x) = item.data {
        //     if let Fields::Named(x) = x.fields {
        //         println!("{:#?}", x.named.iter().collect::<Vec<_>>());
        //     }
        // }
        // println!("{:#?}", item);


        let regex = crate::valid_byte_pattern_regex(&self.pattern).unwrap();

        let fields = self.data.as_ref().take_struct().expect("should never be an enum").fields;
        println!("{:#?}", fields);

        let result = quote! {
            impl #impl_generics #ident #ty_generics #where_clause {
                // pub fn regex() -> &memory_offset_match::Regex {
                //     memory_offset_match::lazy_static! {
                //         static ref RE: memory_offset_match::Regex = memory_offset_match::Regex::new(#regex)
                //             .expect("Failed to compile regex");
                //     }
                    
                //     *RE
                // }
                // pub fn from_image_base(image_base: usize) -> Option<#item_ident> {

                // }
                // pub fn from_slice(data: &[u8]) -> Result<Self> {
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
            &format!("IMPL_FROMSTUFF_FOR_{}", ident.to_string()),
            Span::call_site(),
        );

        tokens.extend(quote! {
            // const #injector: () = {
            //     extern crate memory_offset_match;
                #result
            // };
        });
    }
}

// #[proc_macro_derive(BytePattern, attributes(pattern, bytes_from))]
// pub fn regex_magic_derive(item: TokenStream) -> TokenStream {
  
// }
