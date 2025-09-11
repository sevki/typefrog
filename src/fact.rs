//! Facts about a typeql schema.
use {
    crate::{
        intern,
        internment::{Intern, Interned},
    },
    std::fmt::Display,
    typeql::{schema::definable::Type, token::Kind, Annotation},
};

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
/// Atoms make up relationships.
pub enum Atom {
    /// A label term.
    Label(String),
    /// A value term.
    Value(typeql::token::ValueType),
}

impl Display for Atom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Atom::Label(label) => write!(f, "{label}"),
            Atom::Value(value_type) => match value_type {
                typeql::token::ValueType::Boolean => write!(f, "boolean"),
                typeql::token::ValueType::Date => write!(f, "date"),
                typeql::token::ValueType::DateTime => write!(f, "datetime"),
                typeql::token::ValueType::DateTimeTZ => write!(f, "datetime-tz"),
                typeql::token::ValueType::Decimal => write!(f, "decimal"),
                typeql::token::ValueType::Double => write!(f, "double"),
                typeql::token::ValueType::Duration => write!(f, "duration"),
                typeql::token::ValueType::Integer => write!(f, "integer"),
                typeql::token::ValueType::String => {
                    write!(f, "string")
                }
            },
        }
    }
}

impl From<String> for Atom {
    fn from(label: String) -> Self {
        Atom::Label(label)
    }
}

/// Facts
#[derive(Debug, Hash, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Fact {
    /// Entity fact.
    Entity(Interned<Atom>),
    /// Relation fact.
    Relation(Interned<Atom>),
    /// Attribute fact.
    Attribute(Interned<Atom>),
    /// Role fact.
    Role(Interned<Atom>),
    /// Abstract fact.
    Abstract(Interned<Atom>),
    /// Cascade fact.
    Cascade(Interned<Atom>),
    /// Distinct fact.
    Distinct(Interned<Atom>),
    /// Independent fact.
    Independent(Interned<Atom>),
    /// Key fact.
    Key(Interned<Atom>),
    /// Unique fact.
    Unique(Interned<Atom>),
    /// Value fact.
    Value(Interned<Atom>, Interned<Atom>),
    /// Owns fact.
    Owns(Interned<Atom>, Interned<Atom>),
    /// Relates fact.
    Relates(Interned<Atom>, Interned<Atom>),
    /// Sub fact.
    Sub(Interned<Atom>, Interned<Atom>),
    /// Plays fact.
    Plays(Interned<Atom>, Interned<Atom>, Interned<Atom>),
}

impl From<Interned<Atom>> for Fact {
    fn from(label: Interned<Atom>) -> Self {
        Fact::Entity(label)
    }
}

/// IntoFacts is a trait for converting a type into a collection of facts.
pub trait IntoFacts {
    /// Converts the type into a collection of facts.
    fn into_facts(self) -> impl IntoIterator<Item = Fact>;
}

impl IntoFacts for &Type {
    fn into_facts(self) -> impl IntoIterator<Item = Fact> {
        let mut facts = Vec::new();

        let self_label = { intern!(Atom::Label(self.label.ident.to_string())) };
        match &self.kind {
            Some(Kind::Entity) => facts.push(Fact::Entity(self_label.clone())),
            Some(Kind::Relation) => facts.push(Fact::Relation(self_label.clone())),
            Some(Kind::Attribute) => facts.push(Fact::Attribute(self_label.clone())),
            Some(Kind::Role) => facts.push(Fact::Role(self_label.clone())),
            _ => {}
        }
        fn process_annotations(
            for_label: Interned<Atom>,
            annotations: impl IntoIterator<Item = Annotation>,
        ) -> Vec<Fact> {
            let mut facts = Vec::new();
            for annotation in annotations {
                match annotation {
                    Annotation::Abstract(_) => facts.push(Fact::Abstract(for_label.clone())),
                    Annotation::Cascade(_) => facts.push(Fact::Cascade(for_label.clone())),
                    Annotation::Distinct(_) => facts.push(Fact::Distinct(for_label.clone())),
                    Annotation::Independent(_) => facts.push(Fact::Independent(for_label.clone())),
                    Annotation::Key(_) => facts.push(Fact::Key(for_label.clone())),
                    Annotation::Unique(_) => facts.push(Fact::Unique(for_label.clone())),
                    Annotation::Cardinality(_cardinality) => todo!(),
                    Annotation::Range(_range) => todo!(),
                    Annotation::Regex(_regex) => todo!(),
                    Annotation::Subkey(_subkey) => todo!(),
                    Annotation::Values(_values) => todo!(),
                }
            }
            facts
        }
        facts.extend(process_annotations(
            self_label.clone(),
            self.annotations.clone(),
        ));
        for cap in &self.capabilities {
            match &cap.base {
                typeql::schema::definable::type_::CapabilityBase::Sub(sub) => {
                    let sub_label = { intern!(Atom::Label(sub.supertype_label.ident.to_string())) };
                    facts.push(Fact::Sub(sub_label, self_label.clone()));
                }
                typeql::schema::definable::type_::CapabilityBase::Alias(_alias) => todo!(),
                typeql::schema::definable::type_::CapabilityBase::Owns(owns) => match &owns.owned {
                    typeql::TypeRefAny::Type(type_ref) => match type_ref {
                        typeql::TypeRef::Label(label) => {
                            let owned_label = { intern!(Atom::Label(label.ident.to_string())) };
                            facts.push(Fact::Owns(self_label.clone(), owned_label))
                        }
                        typeql::TypeRef::Scoped(_scoped_label) => todo!(),
                        typeql::TypeRef::Variable(_variable) => todo!(),
                    },
                    typeql::TypeRefAny::List(_type_ref_list) => todo!(),
                },
                typeql::schema::definable::type_::CapabilityBase::Plays(plays) => {
                    let scope_label = { intern!(Atom::Label(plays.role.scope.ident.to_string())) };
                    let name_label = { intern!(Atom::Label(plays.role.name.ident.to_string())) };
                    facts.push(Fact::Plays(self_label.clone(), scope_label, name_label))
                }
                typeql::schema::definable::type_::CapabilityBase::Relates(relates) => {
                    match &relates.related {
                        typeql::TypeRefAny::Type(type_ref) => match &type_ref {
                            typeql::TypeRef::Label(label) => {
                                let related_label =
                                    { intern!(Atom::Label(label.ident.to_string())) };
                                facts.push(Fact::Relates(self_label.clone(), related_label))
                            }
                            typeql::TypeRef::Scoped(_scoped_label) => todo!(),
                            typeql::TypeRef::Variable(_variable) => todo!(),
                        },
                        typeql::TypeRefAny::List(_type_ref_list) => todo!(),
                    }
                }
                typeql::schema::definable::type_::CapabilityBase::ValueType(value_type) => {
                    match &value_type.value_type {
                        typeql::type_::NamedType::Label(label) => {
                            let value_label = { intern!(Atom::Label(label.ident.to_string())) };
                            facts.push(Fact::Value(self_label.clone(), value_label))
                        }
                        typeql::type_::NamedType::BuiltinValueType(builtin_value_type) => {
                            let typ_ = intern!(Atom::Value(builtin_value_type.token));
                            facts.push(Fact::Value(self_label.clone(), typ_));
                        }
                    }
                }
            }
            facts.extend(process_annotations(
                self_label.clone(),
                cap.annotations.clone(),
            ));
        }
        facts
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::intern;
    use crate::horn::Horn;

    #[test]
    fn test_value_type_in_facts() {
        // Test that ValueType atoms work correctly when used in actual Facts
        let entity_atom = intern!(Atom::Label("test_entity".to_string()));
        let boolean_atom = intern!(Atom::Value(typeql::token::ValueType::Boolean));
        let date_atom = intern!(Atom::Value(typeql::token::ValueType::Date));
        let integer_atom = intern!(Atom::Value(typeql::token::ValueType::Integer));
        
        // Create facts using the value type atoms 
        let boolean_fact = Fact::Value(entity_atom.clone(), boolean_atom);
        let date_fact = Fact::Value(entity_atom.clone(), date_atom);
        let integer_fact = Fact::Value(entity_atom.clone(), integer_atom);
        
        // Test that the facts can be converted to logic programming syntax
        assert_eq!(boolean_fact.implication(), "value(test_entity, boolean). % 0 1\n");
        assert_eq!(date_fact.implication(), "value(test_entity, date). % 0 2\n");  
        assert_eq!(integer_fact.implication(), "value(test_entity, integer). % 0 3\n");
    }
}
