use typeql_macros::typeql;

// Test that the macro generates valid Rust code
typeql! {
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
}

fn main() {
    // Test that the generated QUERY variable exists and can be accessed
    let query = &QUERY;
    println!("{:?}", query.structure);
}
