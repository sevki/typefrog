//! Facts about a typeql schema.
use {
    crate::{
        arity::{Binary, Ternary, Unary},
        intern,
        internment::{Intern, Interned},
    },
    std::fmt::Display,
    typeql::{schema::definable::Type, token::Kind, value::IntegerLiteral, Annotation},
};

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
/// Atoms make up relationships.
pub enum Atom {
    /// A label term.
    Label(String),
    /// A value term.
    Value(typeql::token::ValueType),
    /// Integer
    Size(usize),
    /// Regex
    Regex(String),
}

impl Display for Atom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Atom::Label(label) => write!(f, "{label}"),
            Atom::Value(value_type) => match value_type {
                typeql::token::ValueType::Boolean => write!(f, "bool"),
                typeql::token::ValueType::Date => write!(f, "Date"),
                typeql::token::ValueType::DateTime => write!(f, "DateTime"),
                typeql::token::ValueType::DateTimeTZ => write!(f, "DateTimeTZ"),
                typeql::token::ValueType::Decimal => write!(f, "f64"),
                typeql::token::ValueType::Double => write!(f, "f64"),
                typeql::token::ValueType::Duration => write!(f, "Duration"),
                typeql::token::ValueType::Integer => write!(f, "i64"),
                typeql::token::ValueType::String => {
                    write!(f, "String")
                }
            },
            Atom::Size(value) => write!(f, "{value}"),
            Atom::Regex(regex) => write!(f, "{regex}"),
        }
    }
}

impl From<String> for Atom {
    fn from(label: String) -> Self {
        Atom::Label(label)
    }
}

impl From<IntegerLiteral> for Atom {
    fn from(value: IntegerLiteral) -> Self {
        Atom::Size(value.value.parse().unwrap())
    }
}

/// Facts
#[derive(Debug, Hash, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Fact {
    /// Entity fact.
    Entity(Unary),
    /// Relation fact.
    Relation(Unary),
    /// Attribute fact.
    Attribute(Unary),
    /// Role fact.
    Role(Unary),
    /// Abstract fact.
    Abstract(Unary),
    /// Cascade fact.
    Cascade(Unary),
    /// Distinct fact.
    Distinct(Unary),
    /// Independent fact.
    Independent(Unary),
    /// Key fact.
    Key(Unary),
    /// Unique fact.
    Unique(Unary),
    /// Value fact.
    Value(Binary),
    /// Owns fact.
    Owns(Binary),
    /// Relates fact.
    Relates(Binary),
    /// Sub fact.
    Sub(Binary),
    /// Plays fact.
    Plays(Ternary),
    /// Cardinality Exact
    CardExact(Binary),
    /// Cardinality range
    CardRange(Ternary),
    /// Regex fact.
    Regex(Binary),
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
            Some(Kind::Entity) => facts.push(Fact::Entity((self_label.clone(),))),
            Some(Kind::Relation) => facts.push(Fact::Relation((self_label.clone(),))),
            Some(Kind::Attribute) => facts.push(Fact::Attribute((self_label.clone(),))),
            Some(Kind::Role) => facts.push(Fact::Role((self_label.clone(),))),
            None => {}
        }
        fn process_annotations(
            for_label: Interned<Atom>,
            annotations: impl IntoIterator<Item = Annotation>,
        ) -> Vec<Fact> {
            let mut facts = Vec::new();
            for annotation in annotations {
                match annotation {
                    Annotation::Abstract(_) => facts.push(Fact::Abstract((for_label.clone(),))),
                    Annotation::Cascade(_) => facts.push(Fact::Cascade((for_label.clone(),))),
                    Annotation::Distinct(_) => facts.push(Fact::Distinct((for_label.clone(),))),
                    Annotation::Independent(_) => {
                        facts.push(Fact::Independent((for_label.clone(),)))
                    }
                    Annotation::Key(_) => facts.push(Fact::Key((for_label.clone(),))),
                    Annotation::Unique(_) => facts.push(Fact::Unique((for_label.clone(),))),
                    Annotation::Cardinality(cardinality) => match cardinality.range {
                        typeql::annotation::CardinalityRange::Exact(integer_literal) => facts.push(
                            Fact::CardExact((for_label.clone(), intern!(integer_literal))),
                        ),
                        typeql::annotation::CardinalityRange::Range(
                            integer_literal,
                            integer_literal1,
                        ) => facts.push(Fact::CardRange((
                            for_label.clone(),
                            intern!(integer_literal),
                            intern!(
                                integer_literal1.unwrap_or(IntegerLiteral { value: "0".into() })
                            ),
                        ))),
                    },
                    Annotation::Range(_range) => todo!(),
                    Annotation::Regex(regex) => {
                        facts.push(Fact::Regex((for_label.clone(), intern!(regex.regex.value))))
                    }
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
                    let sub_label = intern!(Atom::Label(sub.supertype_label.ident.to_string()));
                    facts.push(Fact::Sub((sub_label, self_label.clone())));
                }
                typeql::schema::definable::type_::CapabilityBase::Alias(_alias) => todo!(),
                typeql::schema::definable::type_::CapabilityBase::Owns(owns) => match &owns.owned {
                    typeql::TypeRefAny::Type(type_ref) => match type_ref {
                        typeql::TypeRef::Label(label) => {
                            let owned_label = intern!(Atom::Label(label.ident.to_string()));
                            facts.push(Fact::Owns((self_label.clone(), owned_label)))
                        }
                        typeql::TypeRef::Scoped(_scoped_label) => todo!(),
                        typeql::TypeRef::Variable(_variable) => todo!(),
                    },
                    typeql::TypeRefAny::List(_type_ref_list) => todo!(),
                },
                typeql::schema::definable::type_::CapabilityBase::Plays(plays) => {
                    let scope_label = intern!(Atom::Label(plays.role.scope.ident.to_string()));
                    let name_label = intern!(Atom::Label(plays.role.name.ident.to_string()));
                    facts.push(Fact::Plays((self_label.clone(), scope_label, name_label)))
                }
                typeql::schema::definable::type_::CapabilityBase::Relates(relates) => {
                    match &relates.related {
                        typeql::TypeRefAny::Type(type_ref) => match &type_ref {
                            typeql::TypeRef::Label(label) => {
                                let related_label = intern!(Atom::Label(label.ident.to_string()));
                                facts.push(Fact::Relates((self_label.clone(), related_label)))
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
                            let value_label = intern!(Atom::Label(label.ident.to_string()));
                            facts.push(Fact::Value((self_label.clone(), value_label)))
                        }
                        typeql::type_::NamedType::BuiltinValueType(builtin_value_type) => {
                            let typ_ = intern!(Atom::Value(builtin_value_type.token));
                            facts.push(Fact::Value((self_label.clone(), typ_)));
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
