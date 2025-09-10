use {
    convert_case::{
        Case::{Flat, Pascal},
        Casing,
    },
    proc_macro2::TokenStream,
    quote::{format_ident, quote, ToTokens},
};

use crate::cased_ident;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) struct Trait {
    pub(crate) name: String,
    pub(crate) funcs: Vec<Fn>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) struct Fn {
    pub(crate) name: String,
    // pub(crate) args: Vec<String>, // always &self but maybe not?
    pub(crate) return_type: String,
}

impl ToTokens for Fn {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = cased_ident!(self.name, Flat);
        let return_type = cased_ident!(self.return_type, Pascal);

        tokens.extend(quote! {
            fn #name(&self) -> #return_type;
        });
    }
}

impl ToTokens for Trait {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = cased_ident!(self.name, Pascal);
        let funcs = &self.funcs;

        tokens.extend(quote! {
            trait #name {
                #(#funcs)*
            }
        });
    }
}
