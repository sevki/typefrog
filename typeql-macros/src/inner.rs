use proc_macro2::TokenStream;
use quote::{ToTokens, TokenStreamExt, quote};
use std::fmt::Display;
use typeql::query::{
    SchemaQuery,
    schema::{Define, Redefine, Undefine},
};

pub fn transform(input: TokenStream) -> TokenStream {
    let input = input.to_string();
    // repatriate punctuation
    let input = input.replace("$ ", "$");
    let input = input.replace(" , ", ", ");
    let input = input.replace(" ;", ";");
    let input = input.replace(" :", ":");
    let input = input.replace(" - ", "-");
    let input = input.replace(" . ", ".");

    let query = typeql::parse_query(input.trim_end()).unwrap();
    transform_query(Query(query))
}

pub struct Query(typeql::Query);

impl Display for Query {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use typeql::pretty::Pretty;
        match &self.0.structure {
            typeql::query::QueryStructure::Schema(SchemaQuery::Define(Define {
                span: _,
                definables,
            })) => {
                for definable in definables {
                    Pretty::fmt(definable, 0, f)?;
                    writeln!(f)?;
                }

                Ok(())
            }
            typeql::query::QueryStructure::Schema(SchemaQuery::Undefine(Undefine {
                span: _,
                undefinables,
            })) => {
                for definable in undefinables {
                    Pretty::fmt(definable, 0, f)?;
                    writeln!(f)?;
                }
                Ok(())
            }
            typeql::query::QueryStructure::Schema(SchemaQuery::Redefine(Redefine {
                span: _,
                definables,
            })) => {
                for definable in definables {
                    Pretty::fmt(definable, 0, f)?;
                    writeln!(f)?;
                }
                Ok(())
            }
            typeql::query::QueryStructure::Pipeline(pipeline) => {
                for stage in &pipeline.stages {
                    match stage {
                        typeql::query::stage::Stage::Insert(insert) => {
                            Pretty::fmt(insert, 0, f)?;
                            writeln!(f)?;
                        }
                        typeql::query::stage::Stage::Put(put) => {
                            Pretty::fmt(put, 0, f)?;
                            writeln!(f)?;
                        }
                        typeql::query::stage::Stage::Update(update) => {
                            Pretty::fmt(update, 0, f)?;
                            writeln!(f)?;
                        }
                        typeql::query::stage::Stage::Fetch(fetch) => {
                            Pretty::fmt(fetch, 0, f)?;
                            writeln!(f)?;
                        }
                        typeql::query::stage::Stage::Delete(delete) => {
                            Pretty::fmt(delete, 0, f)?;
                            writeln!(f)?;
                        }
                        typeql::query::stage::Stage::Operator(operator) => match operator {
                            typeql::query::stage::Operator::Select(select) => {
                                Pretty::fmt(select, 0, f)?;
                                writeln!(f)?;
                            }
                            typeql::query::stage::Operator::Sort(sort) => {
                                Pretty::fmt(sort, 1, f)?;
                                writeln!(f)?;
                            }
                            typeql::query::stage::Operator::Offset(offset) => {
                                Pretty::fmt(offset, 1, f)?;
                                writeln!(f)?;
                            }
                            typeql::query::stage::Operator::Limit(limit) => {
                                Pretty::fmt(limit, 1, f)?;
                                writeln!(f)?;
                            }
                            typeql::query::stage::Operator::Reduce(reduce) => {
                                Pretty::fmt(reduce, 1, f)?;
                                writeln!(f)?;
                            }
                            typeql::query::stage::Operator::Require(require) => {
                                Pretty::fmt(require, 1, f)?;
                                writeln!(f)?;
                            }
                            typeql::query::stage::Operator::Distinct(distinct) => {
                                Pretty::fmt(distinct, 1, f)?;
                                writeln!(f)?;
                            }
                        },
                        typeql::query::stage::Stage::Match(stage) => {
                            Pretty::fmt(stage, 0, f)?;
                            writeln!(f)?;
                        }
                    }
                    writeln!(f)?;
                }
                Ok(())
            }
        }
    }
}

impl ToTokens for Query {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append(proc_macro2::Literal::string(format!("{}", self).as_str()))
    }
}

fn transform_query(query: Query) -> TokenStream {
    quote! {
        static QUERY: std::sync::LazyLock<typeql::Query> = std::sync::LazyLock::new(|| {
            use typeql::parse_query;
            let typeql_query = #query;
            parse_query(typeql_query).unwrap()
        });
    }
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_typeql_inner() {
        let tql = quote! {
            match $page isa page;
            fetch {
                "name": $page.name,
                "bio": $page.bio,
                "id": $page.page-id,
                "profile-picture": $page.profile-picture,
                "type": (
                    match
                    { $ty label person; } or { $ty label organization; } or { $ty label group; };
                    $page isa $ty;
                    return first $ty;
                ),
            };
        };
        let transformed: TokenStream = transform(tql);
        let formatted = quote! {
            #transformed
        };
        let syntax_tree: syn::File = syn::parse2(formatted.clone()).unwrap_or_else(|_| {
            panic!("Failed to parse TypeQL output: {:?}", formatted.to_string())
        });
        let output_str = prettyplease::unparse(&syntax_tree);

        insta::assert_snapshot!(output_str, @r#"
        static QUERY: std::sync::LazyLock<typeql::Query> = std::sync::LazyLock::new(|| {
            use typeql::parse_query;
            let typeql_query = "match\n$page isa page;\n\nfetch {\n    \"name\": $page.name,\n    \"bio\": $page.bio,\n    \"id\": $page.page-id,\n    \"profile-picture\": $page.profile-picture,\n    \"type\": (\n        match\n        {\n            $ty label person;\n        } or {\n            $ty label organization;\n        } or {\n            $ty label group;\n        };\n        $page isa $ty;\n        return first $ty;\n    )\n};\n\n";
            parse_query(typeql_query).unwrap()
        });
        "#);
    }

    #[test]
    fn trybuild_tests() {
        let t = trybuild::TestCases::new();
        t.pass("tests/pass/*.rs");
        t.compile_fail("tests/fail/*.rs");
    }
}
