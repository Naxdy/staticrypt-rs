use proc_macro_error2::abort;
use proc_macro2::{Span, TokenStream};
use syn::{LitStr, parse::Parse, parse2};

use crate::util::gen_decrypt_quote;

struct ScEnvInput {
    var_name: String,
}

impl Parse for ScEnvInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let litstr: LitStr = input.parse()?;
        Ok(Self {
            var_name: litstr.value(),
        })
    }
}

pub fn sc_env(input: TokenStream) -> TokenStream {
    let input: ScEnvInput = match parse2(input) {
        Ok(e) => e,
        Err(e) => return e.into_compile_error(),
    };

    let env_contents = match std::env::var(input.var_name) {
        Ok(e) => e,
        Err(e) => abort! {
            Span::call_site(),
            "Failed to read contents of environment variable: {}", e
        },
    };

    gen_decrypt_quote(env_contents.as_bytes(), true)
}
