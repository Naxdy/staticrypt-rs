use proc_macro2::TokenStream;
use syn::{LitStr, parse::Parse, parse2};

use crate::util::gen_decrypt_quote;

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
    let input: ScInput = match parse2(input) {
        Ok(e) => e,
        Err(e) => return e.into_compile_error(),
    };

    gen_decrypt_quote(&input.literal, true)
}
