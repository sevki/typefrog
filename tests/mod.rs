#![cfg(test)]

use typefrog::{fact::IntoFacts, horn::Horn};
use typeql::query::SchemaQuery;

#[test]
fn test_identifier_interning() {
    let input = "define
attribute fA; fA value string;
attribute fB; fB value string;
attribute fC; fC value string;
attribute fD; fD value string;

entity A @abstract;
entity B @abstract, sub A;
entity C @abstract, sub B;
entity D @abstract, sub C;

A owns fA;
B owns fB;
C owns fC;
D owns fD;

entity E sub A;
entity F sub B;
entity G sub C;";
    let results = match typeql::parse_queries(input) {
        Ok(queries) => queries
            .iter()
            .fold(vec![], |mut acc, query| match &query.structure {
                typeql::query::QueryStructure::Schema(SchemaQuery::Define(define)) => {
                    for definable in &define.definables {
                        match definable {
                            typeql::Definable::TypeDeclaration(type_) => {
                                acc.push(
                                    type_
                                        .into_facts()
                                        .into_iter()
                                        .map(|f| f.implication())
                                        .collect::<Vec<String>>(),
                                );
                            }
                            typeql::Definable::Function(_function) => todo!(),
                            typeql::Definable::Struct(_) => todo!(),
                        }
                    }
                    acc
                }
                _ => acc,
            }),
        Err(e) => {
            panic!("TypeQL parse error: {}", e);
        }
    };

    let combined = results.iter().flatten().fold(String::new(), |mut acc, s| {
        acc.push_str(s);
        acc
    });

    insta::assert_snapshot!(combined, @r"
    attribute(fA). % 0
    value(fA, string). % 0 1
    attribute(fB). % 2
    value(fB, string). % 2 1
    attribute(fC). % 3
    value(fC, string). % 3 1
    attribute(fD). % 4
    value(fD, string). % 4 1
    entity(A). % 5
    abstract(A). % 5
    entity(B). % 6
    abstract(B). % 6
    sub(A, B). % 5 6
    entity(C). % 7
    abstract(C). % 7
    sub(B, C). % 6 7
    entity(D). % 8
    abstract(D). % 8
    sub(C, D). % 7 8
    owns(A, fA). % 5 0
    owns(B, fB). % 6 2
    owns(C, fC). % 7 3
    owns(D, fD). % 8 4
    entity(E). % 9
    sub(A, E). % 5 9
    entity(F). % 10
    sub(B, F). % 6 10
    entity(G). % 11
    sub(C, G). % 7 11
    ");

    insta::assert_debug_snapshot!(typefrog::compute(input).unwrap(), @r#"
    IR {
        structs: [
            Struct {
                name: "E",
                fields: [
                    Field {
                        name: "fA",
                        ty: "string",
                    },
                ],
            },
            Struct {
                name: "F",
                fields: [
                    Field {
                        name: "fA",
                        ty: "string",
                    },
                    Field {
                        name: "fB",
                        ty: "string",
                    },
                ],
            },
            Struct {
                name: "G",
                fields: [
                    Field {
                        name: "fA",
                        ty: "string",
                    },
                    Field {
                        name: "fB",
                        ty: "string",
                    },
                    Field {
                        name: "fC",
                        ty: "string",
                    },
                ],
            },
        ],
        traits: [
            Trait {
                name: "A",
                funcs: [
                    Fn {
                        name: "fA",
                        return_type: "string",
                    },
                ],
            },
            Trait {
                name: "B",
                funcs: [
                    Fn {
                        name: "fA",
                        return_type: "string",
                    },
                    Fn {
                        name: "fB",
                        return_type: "string",
                    },
                ],
            },
            Trait {
                name: "C",
                funcs: [
                    Fn {
                        name: "fA",
                        return_type: "string",
                    },
                    Fn {
                        name: "fB",
                        return_type: "string",
                    },
                    Fn {
                        name: "fC",
                        return_type: "string",
                    },
                ],
            },
            Trait {
                name: "D",
                funcs: [
                    Fn {
                        name: "fA",
                        return_type: "string",
                    },
                    Fn {
                        name: "fB",
                        return_type: "string",
                    },
                    Fn {
                        name: "fC",
                        return_type: "string",
                    },
                    Fn {
                        name: "fD",
                        return_type: "string",
                    },
                ],
            },
        ],
    }
    "#);
}
