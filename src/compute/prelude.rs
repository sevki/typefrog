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
}
