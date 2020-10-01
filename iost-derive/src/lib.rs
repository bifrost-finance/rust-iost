#![allow(clippy::unimplemented)]
extern crate proc_macro;

mod derive_num_bytes;
mod derive_read;
// mod derive_table;
mod derive_write;
// mod derive_digest;
mod derive_serialize_data;

use crate::proc_macro::TokenStream;
use proc_macro2::Span;
use syn::{DeriveInput, Lit, LitStr, Meta, Path};

/// Derive the `Digest` trait
// #[inline]
// #[proc_macro_derive(Digest, attributes(iost_root_path))]
// pub fn derive_digest(input: TokenStream) -> TokenStream {
//     crate::derive_digest::expand(input)
// }

/// Derive the `SerializeData` trait
#[inline]
#[proc_macro_derive(SerializeData, attributes(iost_root_path))]
pub fn derive_serialize_data(input: TokenStream) -> TokenStream {
    crate::derive_serialize_data::expand(input)
}

/// Derive the `Write` trait
#[inline]
#[proc_macro_derive(Write, attributes(iost_root_path))]
pub fn derive_write(input: TokenStream) -> TokenStream {
    crate::derive_write::expand(input)
}

/// Derive the `Read` trait
#[inline]
#[proc_macro_derive(Read, attributes(iost_root_path))]
pub fn derive_read(input: TokenStream) -> TokenStream {
    crate::derive_read::expand(input)
}

/// Derive the `NumberBytes` trait
#[inline]
#[proc_macro_derive(NumberBytes, attributes(iost_root_path))]
pub fn derive_num_bytes(input: TokenStream) -> TokenStream {
    crate::derive_num_bytes::expand(input)
}

/// TODO docs
// #[inline]
// #[proc_macro_derive(
//     Table,
//     attributes(table_name, primary, secondary, singleton)
// )]
// pub fn derive_table(input: TokenStream) -> TokenStream {
//     crate::derive_table::expand(input)
// }

/// The default root path using the `eosio` crate.
#[cfg(feature = "internal-use-only-root-path-is-eosio")]
const DEFAULT_ROOT_PATH: &str = "::eosio";

/// The default root path using the `eosio_core` crate.
#[cfg(not(feature = "internal-use-only-root-path-is-eosio"))]
const DEFAULT_ROOT_PATH: &str = "::eosio_core";

/// Get the root path for types/traits.
pub(crate) fn root_path(input: &DeriveInput) -> Path {
    let litstr = input
        .attrs
        .iter()
        .fold(None, |acc, attr| match attr.parse_meta() {
            Ok(meta) => {
                let name = meta.path().get_ident();
                if name
                    .as_ref()
                    .expect("please add trait root path")
                    .to_string()
                    == "iost_root_path"
                {
                    match meta {
                        Meta::NameValue(meta) => match meta.lit {
                            Lit::Str(s) => Some(s),
                            _ => panic!("iost_root_path must be a lit str"),
                        },
                        _ => acc,
                    }
                } else {
                    acc
                }
            }
            Err(_) => acc,
        })
        .unwrap_or_else(|| LitStr::new(DEFAULT_ROOT_PATH, Span::call_site()));
    litstr
        .parse_with(Path::parse_mod_style)
        .expect("bad path for iost_root_path")
}
