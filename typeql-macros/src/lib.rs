use crate::inner::transform;
use proc_macro::TokenStream;

mod inner;

#[proc_macro]
pub fn typeql(input: TokenStream) -> TokenStream {
    transform(input.into()).into()
}
