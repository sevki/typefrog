use typeql_macros::typeql;

// Test that the macro generates valid Rust code
typeql! {
    match $person isa person;
}

fn main() {
    // Test that the generated QUERY variable exists and can be accessed
    let query = &QUERY;
    println!("{:?}", query.structure);
}
