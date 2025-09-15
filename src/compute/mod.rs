//! Compute logic for typeql type inheritance

use {
    crate::{
        arity::Binary,
        fact::{Atom, Fact, IntoFacts},
        internment::Interned,
        ir::{Field, Fn, Relation, Struct, Trait, IR},
        Result,
    },
    ascent::ascent,
    rustc_hash::FxHashMap,
    typeql::query::{QueryStructure::Schema, SchemaQuery},
};

mod prelude;

ascent! {
    include_source!(prelude::typefrog);
    // Rules
    sub(grandparent,grandchild) <-- sub(grandparent,parent), sub(parent,grandchild);
    owns(owner,owned) <-- sub(grandparent,owner), owns(grandparent,owned), attribute(owned);
    abstracts(name,field,type_) <-- abs(name), owns(name,field), attribute(field), value(field,type_);
    entities(name,field,type_) <-- entity(name), owns(name,field), attribute(field), value(field,type_);
    players(entity,scope,plays) <-- role(role), plays(entity, scope, plays);
}

/// Compute TypeQL queries.
pub fn compute(input: &str) -> Result<IR> {
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
                                        Fact::Relation(val) => prog.rel.push(val),
                                        Fact::Attribute(val) => prog.attribute.push(val),
                                        Fact::Role(val) => prog.role.push(val),
                                        Fact::Entity(val) => prog.entity.push(val),
                                        Fact::Abstract(val) => prog.abs.push(val),
                                        Fact::Value(value) => prog.value.push(value),
                                        Fact::Owns(owns) => prog.owns.push(owns),
                                        Fact::Relates(relates) => prog.relates.push(relates),
                                        Fact::Regex(regex) => prog.regex.push(regex),
                                        Fact::Sub(sub) => prog.sub.push(sub),
                                        Fact::Plays(plays) => prog.plays.push(plays),
                                        Fact::CardExact(fact) => prog.cardinality_exact.push(fact),
                                        Fact::CardRange(fact) => prog.cardinality_range.push(fact),
                                        // db side constraints
                                        Fact::Cascade(_interned) => {}
                                        Fact::Distinct(_interned) => {}
                                        Fact::Independent(_interned) => {}
                                        Fact::Key(_interned) => {}
                                        Fact::Unique(_interned) => {}
                                    }
                                }
                            }
                            typeql::Definable::Function(_function) => {}
                            typeql::Definable::Struct(_) => todo!(),
                        }
                    }
                }
            });
        }
        Err(e) => return Err(e.into()),
    }

    prog.run();

    // Group abstracts by entity name
    let mut abstract_map: FxHashMap<Interned<Atom>, Vec<Binary>> = FxHashMap::default();
    for (entity, field, type_) in prog.abstracts {
        abstract_map.entry(entity).or_default().push((field, type_));
    }

    // Group entities by entity name
    let mut entity_map: FxHashMap<Interned<Atom>, Vec<Binary>> = FxHashMap::default();
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
                let mut impls = vec![];
                for (parent, child) in &prog.sub {
                    if child.uid() == name.uid() {
                        for trait_ in &traits {
                            if trait_.name == parent.to_string() {
                                impls.push(trait_.clone());
                            }
                        }
                    }
                }
                Some(Struct {
                    name: name.to_string(),
                    fields: sorted_fields,
                    impls,
                })
            }
        })
        .collect();
    structs.sort_by(|a, b| a.name.cmp(&b.name));

    let relations = prog
        .role
        .iter()
        .map(|a| Relation {
            name: a.0.to_string(),
        })
        .collect::<Vec<_>>();

    Ok(IR {
        structs,
        traits,
        relations,
    })
}
