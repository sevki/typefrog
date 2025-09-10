use {
    crate::cased_ident,
    convert_case::{
        Case::{Flat, Pascal},
        Casing,
    },
    proc_macro2::TokenStream,
    quote::{format_ident, quote, ToTokens},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) struct Struct {
    pub(crate) name: String,
    pub(crate) fields: Vec<Field>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) struct Field {
    pub(crate) name: String,
    pub(crate) ty: String,
}

impl ToTokens for Field {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ty = cased_ident!(self.ty, Pascal);
        let name = cased_ident!(self.name, Flat);
        tokens.extend(quote! {
            #name: #ty
        });
    }
}

impl ToTokens for Struct {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = cased_ident!(self.name, Pascal);
        let fields = self.fields.clone();
        tokens.extend(quote! {
            struct #name {
                #(#fields),*
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use crate::compute;

    use super::*;

    #[test]
    fn test_struct() {
        let input = "define
    attribute f0; f0 value string;
    attribute f1; f1 value string;
    attribute f2; f2 value string;
    attribute f3; f3 value string;

    entity A0 @abstract; A0 owns f0;
    entity A1 @abstract; A1 sub A0; A1 owns f1;
    entity A2 @abstract; A2 sub A1; A2 owns f2;
    entity A3 @abstract; A3 sub A2; A3 owns f3;

    entity L2 sub A2;
    entity L3 sub A3;";
        let output = compute(input).unwrap();
        let formated = quote! { #output };
        let syntax_tree: syn::File = syn::parse2(formated.clone()).unwrap_or_else(|_| {
            panic!("Failed to parse TypeQL output: {:?}", formated.to_string())
        });
        let output_str = prettyplease::unparse(&syntax_tree);

        insta::assert_snapshot!(output_str, @r"
        struct L2 {
            f0: String,
            f1: String,
            f2: String,
        }
        struct L3 {
            f0: String,
            f1: String,
            f2: String,
            f3: String,
        }
        trait A0 {
            fn f0(&self) -> String;
        }
        trait A1 {
            fn f0(&self) -> String;
            fn f1(&self) -> String;
        }
        trait A2 {
            fn f0(&self) -> String;
            fn f1(&self) -> String;
            fn f2(&self) -> String;
        }
        trait A3 {
            fn f0(&self) -> String;
            fn f1(&self) -> String;
            fn f2(&self) -> String;
            fn f3(&self) -> String;
        }
        ");
    }
}
