use quote::ToTokens;

pub(crate) use crate::ir::{r#struct::*, r#trait::*, relation::*};

mod relation;
mod r#struct;
mod r#trait;

#[macro_export]
#[doc(hidden)]
macro_rules! cased_ident {
    ($name:expr, $case:path) => {
        format_ident!("{}", $name.to_case($case))
    };
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct IR {
    pub(crate) structs: Vec<Struct>,
    pub(crate) traits: Vec<Trait>,
    pub(crate) relations: Vec<Relation>,
}

impl ToTokens for IR {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.structs.iter().for_each(|s| s.to_tokens(tokens));
        self.traits.iter().for_each(|t| t.to_tokens(tokens));
        self.relations.iter().for_each(|r| r.to_tokens(tokens));
    }
}
