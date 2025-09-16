//! typefrog build module enables developers to use
//! typefrog during the build process of their projects.

use crate::{compute, Result};
use quote::quote;
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, sync::Arc, time::Duration};
use typedb_driver::{Credentials, DriverOptions, TypeDBDriver};
use url::Url;

/// From file takes a pathbuf and converts it to Rust code.
fn from_file(path: std::path::PathBuf) -> Result<String> {
    println!("cargo:rerun-if-changed={}", path.display());
    from_input(&std::fs::read_to_string(path)?)
}

/// Converts a string input to Rust code.
pub fn from_input(input: &str) -> Result<String> {
    let output = compute(input)?;
    let formatted = quote! { #output };
    let syntax_tree: syn::File = syn::parse2(formatted.clone())?;
    let output_str = prettyplease::unparse(&syntax_tree);
    Ok(output_str)
}

/// Gets the schema from the database and converts it to Rust code.
fn from_database(db: Arc<typedb_driver::Database>) -> Result<String> {
    from_input(&db.type_schema()?)
}

/// Gets the schema from the database and converts it to Rust code.
fn from_database_config(db: ConnectionString) -> Result<String> {
    let driver = Arc::new(
        TypeDBDriver::new(
            db.addresses.first().unwrap(),
            Credentials::new(&db.username, &db.password),
            DriverOptions::new(db.is_tls_enabled, None).unwrap(),
        )
        .unwrap(),
    );
    from_database(driver.databases().get(db.database_name.as_str())?)
}

fn usage(name: &str) -> ! {
    let a = toml::to_string_pretty(&TypefrogMetadata {
        source: Source::ConnectionString(ConnectionString {
            username: "".to_string(),
            password: "".to_string(),
            addresses: vec![],
            database_name: "".to_string(),
            connection_url: Url::parse(
                "typedb://admin:password@https://cluster.typedb.com:80/?name=carefree-nematode",
            )
            .unwrap(),
            is_tls_enabled: false,
        }),
    })
    .unwrap();

    let b = toml::to_string_pretty(&TypefrogMetadata {
        source: Source::File(File {
            path: PathBuf::from("path/to/file"),
        }),
    })
    .unwrap();

    panic!(
        "Invalid typefrog metadata for package {}
valid forms are:
{}
{}",
        name, a, b
    );
}

/// Gets the metadata from the Cargo.toml file and converts it to Rust code.
pub fn from_metadata(name: &str) -> Result<String> {
    println!("cargo:rerun-if-changed=Cargo.toml");

    let meta = cargo_metadata::MetadataCommand::new()
        .no_deps()
        .exec()
        .unwrap();

    let package = meta
        .packages
        .iter()
        .find(|p| p.name.to_string() == name)
        .unwrap_or_else(|| {
            let packages = meta
                .packages
                .iter()
                .map(|p| p.name.to_string())
                .collect::<Vec<String>>();

            panic!("Package {} not found in {:?}", name, packages)
        });

    let Some(p) = package.metadata.get("typefrog") else {
        usage(package.name.as_str())
    };

    let metadata: TypefrogMetadata =
        serde_json::from_value(p.clone()).unwrap_or_else(|_| usage(package.name.as_str()));

    match metadata.source {
        Source::File(file) => from_file(file.path),
        Source::ConnectionString(connection_string) => from_database_config(connection_string),
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct TypefrogMetadata {
    source: Source,
}

#[derive(Serialize, Deserialize, Debug)]
struct File {
    path: PathBuf,
}

#[derive(Serialize, Deserialize, Debug)]
struct ConnectionString {
    username: String,
    password: String,
    addresses: Vec<String>,
    database_name: String,
    connection_url: Url,
    is_tls_enabled: bool,
}

#[derive(Serialize, Deserialize, Debug)]
enum Source {
    #[serde(rename = "file")]
    File(File),
    #[serde(rename = "connection_string")]
    ConnectionString(ConnectionString),
}

#[derive(Serialize, Deserialize, Debug)]
struct Report {
    generated_packages: Vec<GeneratedPackage>,
}

#[derive(Serialize, Deserialize, Debug)]
struct GeneratedPackage {
    name: String,
    schema: String,
    duration: Duration,
    formatted: String,
}
