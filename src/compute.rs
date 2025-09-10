//! Compute logic for typeql type inheritance

use ascent::ascent;
use rustc_hash::FxHashMap;
use {
    crate::{
        fact::{Atom, Fact, IntoFacts},
        internment::Interned,
        ir::{Field, Fn, Struct, Trait, IR},
    },
    typeql::query::{QueryStructure::Schema, SchemaQuery},
};

mod arity {
    use super::*;
    pub type Binary = (Interned<Atom>, Interned<Atom>);
}

ascent! {
    #[allow(non_snake_case)]
    // Facts prelude
    // TODO(sevki): use ascent_source!{} macro to split this out.
    relation owns(Interned<Atom>, Interned<Atom>);
    relation sub(Interned<Atom>, Interned<Atom>);
    relation value(Interned<Atom>, Interned<Atom>);
    relation abs(Interned<Atom>);
    relation attribute(Interned<Atom>);
    relation role(Interned<Atom>);
    relation entity(Interned<Atom>);
    relation rel(Interned<Atom>);
    relation abstracts(Interned<Atom>, Interned<Atom>, Interned<Atom>);
    relation entities(Interned<Atom>, Interned<Atom>, Interned<Atom>);
    // Rules
    sub(grandparent,grandchild) <-- sub(grandparent,parent), sub(parent,grandchild);
    owns(owner,owned) <-- sub(grandparent,owner), owns(grandparent,owned), attribute(owned);
    abstracts(name,field,type_) <-- abs(name), owns(name,field), attribute(field), value(field,type_);
    entities(name,field,type_) <-- entity(name), owns(name,field), attribute(field), value(field,type_);
}

/// Compute TypeQL queries.
pub fn compute(input: &str) -> Result<IR, String> {
    let mut prog = AscentProgram {
        ..Default::default()
    };

    match typeql::parse_queries(input) {
        Ok(queries) => {
            queries.iter().for_each(|query| {
                if let Schema(SchemaQuery::Define(define)) = &query.structure {
                    for definable in &define.definables {
                        match definable {
                            typeql::Definable::TypeDeclaration(type_) => {
                                for fact in type_.into_facts() {
                                    match fact {
                                        // these are type declarations, as far as I can tell, an object cannot be both an abstract and a concrete entity
                                        Fact::Relation(interned) => {
                                            prog.rel.push((interned,));
                                        }
                                        Fact::Attribute(interned) => {
                                            prog.attribute.push((interned,));
                                        }
                                        Fact::Role(interned) => {
                                            prog.role.push((interned,));
                                        }
                                        Fact::Entity(interned) => {
                                            prog.entity.push((interned,));
                                        }
                                        Fact::Abstract(abs) => {
                                            prog.abs.push((abs,));
                                        }
                                        Fact::Value(ident, type_) => {
                                            prog.value.push((ident, type_))
                                        }

                                        Fact::Cascade(_interned) => todo!(),
                                        Fact::Distinct(_interned) => todo!(),
                                        Fact::Independent(_interned) => todo!(),
                                        Fact::Key(_interned) => todo!(),
                                        Fact::Unique(_interned) => todo!(),
                                        Fact::Owns(owner, owned) => {
                                            prog.owns.push((owner, owned));
                                        }
                                        Fact::Relates(_a, _b) => {}
                                        Fact::Sub(parent, child) => {
                                            prog.sub.push((parent, child));
                                        }
                                        Fact::Plays(_a, _b, _c) => {}
                                    }
                                }
                            }
                            typeql::Definable::Function(_function) => todo!(),
                            typeql::Definable::Struct(_) => todo!(),
                        }
                    }
                }
            });
        }
        Err(e) => return Err(e.to_string()),
    }

    prog.run();

    // Group abstracts by entity name
    let mut abstract_map: FxHashMap<Interned<Atom>, Vec<arity::Binary>> = FxHashMap::default();
    for (entity, field, type_) in prog.abstracts {
        abstract_map.entry(entity).or_default().push((field, type_));
    }

    // Group entities by entity name
    let mut entity_map: FxHashMap<Interned<Atom>, Vec<arity::Binary>> = FxHashMap::default();
    for (entity, field, type_) in prog.entities {
        entity_map.entry(entity).or_default().push((field, type_));
    }

    // Create traits from abstracts
    let mut traits: Vec<Trait> = abstract_map
        .into_iter()
        .map(|(name, fields)| {
            let mut funcs: Vec<Fn> = fields
                .into_iter()
                .map(|(field, type_)| Fn {
                    name: field.to_string(),
                    return_type: type_.to_string(),
                })
                .collect();
            funcs.sort_by(|a, b| a.name.cmp(&b.name));
            Trait {
                name: name.to_string(),
                funcs,
            }
        })
        .collect();
    traits.sort_by(|a, b| a.name.cmp(&b.name));

    // Create structs from entities (but exclude abstracts that also appear in entities)
    let mut structs: Vec<Struct> = entity_map
        .into_iter()
        .filter_map(|(name, fields)| {
            // this can come in the form of a type_.kind or an annotation on a type.
            // this is to guard against the case where it's
            // `entity A @abstract;`
            // not
            // `abstract A;`
            if prog.abs.iter().any(|(abs,)| abs == &name) {
                None
            } else {
                let mut sorted_fields: Vec<Field> = fields
                    .into_iter()
                    .map(|(field, type_)| Field {
                        name: field.to_string(),
                        ty: type_.to_string(),
                    })
                    .collect();
                sorted_fields.sort_by(|a, b| a.name.cmp(&b.name));
                Some(Struct {
                    name: name.to_string(),
                    fields: sorted_fields,
                })
            }
        })
        .collect();
    structs.sort_by(|a, b| a.name.cmp(&b.name));

    Ok(IR { structs, traits })
}
