use {
    crate::cased_ident,
    convert_case::{Case::Pascal, Casing},
    quote::{format_ident, quote, ToTokens},
};
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Relation {
    pub name: String,
}

impl ToTokens for Relation {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = cased_ident!(self.name, Pascal);
        tokens.extend(quote! {
            pub trait #name {
            }
        });
    }
}
