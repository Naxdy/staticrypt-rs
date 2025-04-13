use proc_macro::TokenStream;
use syn::{LitStr, parse::Parse, parse_macro_input};

use crate::util::{byte_array_literal, decrypt_quote, encrypt, get_key};

struct ScInput {
    literal: Vec<u8>,
}

impl Parse for ScInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let litstr: LitStr = input.parse()?;

        Ok(Self {
            literal: litstr.value().as_bytes().to_vec(),
        })
    }
}

pub fn sc(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ScInput);

    let key = get_key();

    let (encrypted, nonce) = encrypt(&input.literal, &key);

    let encrypted_literal = byte_array_literal(&encrypted);

    let nonce_literal = byte_array_literal(&nonce);

    decrypt_quote(encrypted_literal, nonce_literal).into()
}
