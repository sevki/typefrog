//! Horn clause is a logical formula in conjunctive normal form with at most one positive literal.
//!
//! A horn clause is made of two components, head and body, the logic programming usually follows the
//! implication form of the Horn clause.
//!
//! It takes 3 forms;
//!
//! | Form            | Implication                 | Prolog               |
//! | --------------- | --------------------------- | -------------------- |
//! | Definite clause | `u ← (p ∧ q ∧ ... ∧ t)`     | `u :- p, q, ..., t.` |
//! | Fact            | `u ← true`                  | `u.`                 |
//! | Goal clause     | `false ← (p ∧ q ∧ ... ∧ t)` | `:- p, q, ..., t.`   |
//!
//! Mostly from <https://en.wikipedia.org/wiki/Horn_clause>
use crate::{arity::*, fact::Fact};

/// A dialect for Horn clause.
pub trait Dialect {
    /// The opening parenthesis character.
    const OPEN_PAREN: &'static str;
    /// The closing parenthesis character.
    const CLOSE_PAREN: &'static str;
    /// The clause end character.
    const CLAUSE_END: &'static str;
    /// The comment character.
    const COMMENT: &'static str;
    /// The conjunction character.
    const CONJ: &'static str; // AND
    /// The disjunction character.
    const DISJ: &'static str; // OR
}

/// Prolog dialect.
pub struct Prolog;

/// Ascent dialect.
pub struct Ascent;

impl Dialect for Prolog {
    const OPEN_PAREN: &'static str = "(";
    const CLOSE_PAREN: &'static str = ")";
    const CLAUSE_END: &'static str = ".";
    const COMMENT: &'static str = "%";
    const CONJ: &'static str = ","; // AND
    const DISJ: &'static str = ";"; // OR
}

impl Dialect for Ascent {
    const OPEN_PAREN: &'static str = "(";
    const CLOSE_PAREN: &'static str = ")";
    const CLAUSE_END: &'static str = ";";
    const COMMENT: &'static str = "//";
    const CONJ: &'static str = ","; // AND in ascent
    /// NOT TRUE
    const DISJ: &'static str = "||";
}

/// Horn clause is a logical formula in conjunctive normal form with at most one positive literal.
pub trait Clause {
    /// Print a Horn clause in it's implication form.
    fn implication<D: Dialect>(&self) -> String;
    /// Print a Horn clause in it's disjunction form.
    fn disjunction(&self) -> String;
}

impl Clause for Fact {
    fn implication<D: Dialect>(&self) -> String {
        let (pred, s, comment) = match self {
            Fact::Entity(a) => (
                "entity",
                format_unary::<D>(a),
                InternComment::intern_comment(a),
            ),
            Fact::Relation(a) => (
                "rel",
                format_unary::<D>(a),
                InternComment::intern_comment(a),
            ),
            Fact::Attribute(a) => (
                "attribute",
                format_unary::<D>(a),
                InternComment::intern_comment(a),
            ),
            Fact::Role(a) => (
                "role",
                format_unary::<D>(a),
                InternComment::intern_comment(a),
            ),
            Fact::Abstract(a) => (
                "abstract",
                format_unary::<D>(a),
                InternComment::intern_comment(a),
            ),
            Fact::Cascade(a) => (
                "cascade",
                format_unary::<D>(a),
                InternComment::intern_comment(a),
            ),
            Fact::Distinct(a) => (
                "distinct",
                format_unary::<D>(a),
                InternComment::intern_comment(a),
            ),
            Fact::Independent(a) => (
                "independent",
                format_unary::<D>(a),
                InternComment::intern_comment(a),
            ),
            Fact::Key(a) => (
                "key",
                format_unary::<D>(a),
                InternComment::intern_comment(a),
            ),
            Fact::Unique(a) => (
                "unique",
                format_unary::<D>(a),
                InternComment::intern_comment(a),
            ),
            Fact::Value(value) => (
                "value",
                format_binary::<D>(value),
                InternComment::intern_comment(value),
            ),
            Fact::Owns(owns) => (
                "owns",
                format_binary::<D>(owns),
                InternComment::intern_comment(owns),
            ),
            Fact::Relates(relates) => (
                "relates",
                format_binary::<D>(relates),
                InternComment::intern_comment(relates),
            ),
            Fact::Sub(sub) => (
                "sub",
                format_binary::<D>(sub),
                InternComment::intern_comment(sub),
            ),
            Fact::Plays(plays) => (
                "plays",
                format_ternary::<D>(plays),
                InternComment::intern_comment(plays),
            ),
        };
        format!("{pred}{s}{} {} {comment}\n", D::CLAUSE_END, D::COMMENT,)
    }

    fn disjunction(&self) -> String {
        todo!()
    }
}
