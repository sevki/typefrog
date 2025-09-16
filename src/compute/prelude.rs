ascent::ascent_source! {
    /// prelude
    typefrog:
    #[allow(non_snake_case)]
    // Facts prelude
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
    relation relates(Interned<Atom>, Interned<Atom>);
    relation plays(Interned<Atom>, Interned<Atom>, Interned<Atom>);
    relation players(Interned<Atom>, Interned<Atom>, Interned<Atom>);
    relation cardinality_exact(Interned<Atom>, Interned<Atom>);
    relation cardinality_range(Interned<Atom>, Interned<Atom>, Interned<Atom>);
    relation regex(Interned<Atom>, Interned<Atom>);
}
