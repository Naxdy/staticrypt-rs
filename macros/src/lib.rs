mod env;
mod file;
mod literal;
mod util;

use proc_macro::TokenStream;
use proc_macro_error2::proc_macro_error;

#[proc_macro]
#[proc_macro_error]
pub fn sc_env(input: TokenStream) -> TokenStream {
    env::sc_env(input.into()).into()
}

#[proc_macro]
#[proc_macro_error]
pub fn sc_bytes(input: TokenStream) -> TokenStream {
    file::sc_bytes(input.into()).into()
}

#[proc_macro]
#[proc_macro_error]
pub fn sc(input: TokenStream) -> TokenStream {
    literal::sc(input.into()).into()
}

#[proc_macro]
pub fn use_staticrypt(_: TokenStream) -> TokenStream {
    util::init().into()
}
