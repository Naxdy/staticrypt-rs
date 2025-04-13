mod file;
mod literal;
mod util;

use proc_macro::TokenStream;

#[proc_macro]
pub fn sc_bytes(input: TokenStream) -> TokenStream {
    file::sc_bytes(input)
}

#[proc_macro]
pub fn sc(input: TokenStream) -> TokenStream {
    literal::sc(input)
}

#[proc_macro]
pub fn use_staticrypt(_: TokenStream) -> TokenStream {
    util::init().into()
}
