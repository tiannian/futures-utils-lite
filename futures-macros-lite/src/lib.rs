use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn join(_args: TokenStream, input: TokenStream) -> TokenStream {
    input
}
