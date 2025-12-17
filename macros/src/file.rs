use std::path::PathBuf;

use darling::FromMeta;
use proc_macro_error2::abort;
use proc_macro2::Span;
use proc_macro2::TokenStream;
use syn::{LitStr, parse::Parse, parse2};

use crate::util::gen_decrypt_quote;

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
    let input: ScBytesInput = match parse2(input) {
        Ok(e) => e,
        Err(e) => return e.into_compile_error(),
    };

    let file_contents =
        match std::fs::read(PathBuf::from_string(&input.file_path).expect("Failed to get path")) {
            Ok(e) => e,
            Err(e) => abort! {
                Span::call_site(),
                "Failed to read contents of file {}: {:?}", input.file_path, e
            },
        };

    gen_decrypt_quote(&file_contents, false)
}
