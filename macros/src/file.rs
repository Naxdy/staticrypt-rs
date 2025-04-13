use std::path::PathBuf;

use darling::FromMeta;
use proc_macro::TokenStream;
use quote::quote;
use syn::{LitStr, parse::Parse, parse_macro_input};

use crate::util::{byte_array_literal, encrypt, get_key, staticrypt_crate_name};

struct ScBytesInput {
    file_path: String,
}

impl Parse for ScBytesInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let litstr: LitStr = input.parse()?;
        Ok(Self {
            file_path: litstr.value(),
        })
    }
}

pub fn sc_bytes(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ScBytesInput);

    let file_contents =
        std::fs::read(PathBuf::from_string(&input.file_path).expect("Failed to get path"))
            .unwrap_or_else(|e| {
                panic!("Failed to read contents of file {}: {e:?}", input.file_path)
            });

    let key = get_key();

    let (encrypted, nonce) = encrypt(&file_contents, &key);

    let encrypted_literal = byte_array_literal(&encrypted);

    let nonce_literal = byte_array_literal(&nonce);

    let crate_name = staticrypt_crate_name();

    quote! {
        {
            const ENCRYPTED: &[u8] = &#encrypted_literal;
            const NONCE: &[u8] = &#nonce_literal;

            #crate_name::decrypt(ENCRYPTED, NONCE, crate::STATICRYPT_ENCRYPT_KEY)
        }
    }
    .into()
}
