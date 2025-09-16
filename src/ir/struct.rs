use {
    crate::{
        cased_ident,
        ir::{r#trait::Fn, Trait},
    },
    convert_case::{
        Case::{Pascal, Snake},
        Casing,
    },
    proc_macro2::TokenStream,
    quote::{format_ident, quote, ToTokens},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) struct Struct {
    pub(crate) name: String,
    pub(crate) fields: Vec<Field>,
    pub(crate) impls: Vec<Trait>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) struct Field {
    pub(crate) name: String,
    pub(crate) ty: String,
}

impl ToTokens for Field {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ty = format_ident!("{}", self.ty);
        let name = cased_ident!(self.name, Snake);
        tokens.extend(quote! {
            #name: #ty
        });
    }
}

struct For {
    name: String,
    trait_: Trait,
}

struct FnImpl {
    fn_: Fn,
}

impl ToTokens for FnImpl {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = cased_ident!(self.fn_.name, Snake);
        let ty = format_ident!("{}", self.fn_.return_type);
        tokens.extend(quote! {
            fn #name(&self) -> &#ty {
                &self.#name
            }
        });
    }
}

impl ToTokens for For {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = cased_ident!(self.name, Pascal);
        let trait_name = cased_ident!(self.trait_.name, Pascal);
        let fn_impls = self
            .trait_
            .funcs
            .iter()
            .map(|a| FnImpl { fn_: a.clone() })
            .collect::<Vec<_>>();
        tokens.extend(quote! {
            impl #trait_name for #name {
                #(#fn_impls)*
            }
        });
    }
}

impl ToTokens for Struct {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = cased_ident!(self.name, Pascal);
        let fields = self.fields.clone();
        let impls = self
            .impls
            .iter()
            .map(|n| For {
                name: self.name.clone(),
                trait_: n.clone(),
            })
            .collect::<Vec<_>>();
        tokens.extend(quote! {
            struct #name {
                #(#fields),*
            }
            #(#impls)*
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
            f_0: String,
            f_1: String,
            f_2: String,
        }
        impl A2 for L2 {
            fn f_0(&self) -> &String {
                &self.f_0
            }
            fn f_1(&self) -> &String {
                &self.f_1
            }
            fn f_2(&self) -> &String {
                &self.f_2
            }
        }
        impl A1 for L2 {
            fn f_0(&self) -> &String {
                &self.f_0
            }
            fn f_1(&self) -> &String {
                &self.f_1
            }
        }
        impl A0 for L2 {
            fn f_0(&self) -> &String {
                &self.f_0
            }
        }
        struct L3 {
            f_0: String,
            f_1: String,
            f_2: String,
            f_3: String,
        }
        impl A3 for L3 {
            fn f_0(&self) -> &String {
                &self.f_0
            }
            fn f_1(&self) -> &String {
                &self.f_1
            }
            fn f_2(&self) -> &String {
                &self.f_2
            }
            fn f_3(&self) -> &String {
                &self.f_3
            }
        }
        impl A2 for L3 {
            fn f_0(&self) -> &String {
                &self.f_0
            }
            fn f_1(&self) -> &String {
                &self.f_1
            }
            fn f_2(&self) -> &String {
                &self.f_2
            }
        }
        impl A1 for L3 {
            fn f_0(&self) -> &String {
                &self.f_0
            }
            fn f_1(&self) -> &String {
                &self.f_1
            }
        }
        impl A0 for L3 {
            fn f_0(&self) -> &String {
                &self.f_0
            }
        }
        trait A0 {
            fn f_0(&self) -> String;
        }
        trait A1 {
            fn f_0(&self) -> String;
            fn f_1(&self) -> String;
        }
        trait A2 {
            fn f_0(&self) -> String;
            fn f_1(&self) -> String;
            fn f_2(&self) -> String;
        }
        trait A3 {
            fn f_0(&self) -> String;
            fn f_1(&self) -> String;
            fn f_2(&self) -> String;
            fn f_3(&self) -> String;
        }
        ");
    }
}
