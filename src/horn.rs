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
use crate::fact::Fact;

/// Dialect of Horn clause.
pub enum Dialect {
    /// Prolog dialect.
    /// ```prolog,ignore
    /// entity(1).
    /// relation(1, 2).
    /// attribute(1, 2).
    /// ```
    Prolog,
    /// Ascent dialect.
    /// ```rust,ignore
    /// ascent!{
    ///     relation relation_(Interned<Atom>);
    ///     Rules
    ///     sub(grandparent,grandchild) <-- sub(grandparent,parent), sub(parent,grandchild);
    /// }
    /// ```
    Ascent,
}

/// Horn clause is a logical formula in conjunctive normal form with at most one positive literal.
pub trait Horn {
    /// Print a Horn clause in it's implication form.
    fn implication(&self) -> String;
}

impl Horn for Fact {
    fn implication(&self) -> String {
        let clause = match self {
            Fact::Entity(term) => Self::format_unary("entity", term),
            Fact::Relation(interned) => Self::format_unary("relation", interned),
            Fact::Attribute(interned) => Self::format_unary("attribute", interned),
            Fact::Role(interned) => Self::format_unary("role", interned),
            Fact::Abstract(interned) => Self::format_unary("abstract", interned),
            Fact::Cascade(interned) => Self::format_unary("cascade", interned),
            Fact::Distinct(interned) => Self::format_unary("distinct", interned),
            Fact::Independent(interned) => Self::format_unary("independent", interned),
            Fact::Key(interned) => Self::format_unary("key", interned),
            Fact::Unique(interned) => Self::format_unary("unique", interned),
            Fact::Value(a, b) => Self::format_binary("value", a, b),
            Fact::Owns(a, b) => Self::format_binary("owns", a, b),
            Fact::Relates(a, b) => Self::format_binary("relates", a, b),
            Fact::Sub(a, b) => Self::format_binary("sub", a, b),
            Fact::Plays(a, b, c) => Self::format_ternary("plays", a, b, c),
        };
        format!("{}\n", clause)
    }
}

impl Fact {
    fn format_unary<T>(predicate: &str, arg: &crate::internment::Interned<T>) -> String
    where
        T: std::fmt::Display + Eq + Clone,
    {
        format!("{}({}). % {}", predicate, arg, arg.uid())
    }

    fn format_binary<T>(
        predicate: &str,
        arg1: &crate::internment::Interned<T>,
        arg2: &crate::internment::Interned<T>,
    ) -> String
    where
        T: std::fmt::Display + Eq + Clone,
    {
        format!(
            "{}({}, {}). % {} {}",
            predicate,
            arg1,
            arg2,
            arg1.uid(),
            arg2.uid()
        )
    }

    fn format_ternary<T>(
        predicate: &str,
        arg1: &crate::internment::Interned<T>,
        arg2: &crate::internment::Interned<T>,
        arg3: &crate::internment::Interned<T>,
    ) -> String
    where
        T: std::fmt::Display + Eq + Clone,
    {
        format!(
            "{}({}, {}, {}). % {} {} {}",
            predicate,
            arg1,
            arg2,
            arg3,
            arg1.uid(),
            arg2.uid(),
            arg3.uid()
        )
    }
}
