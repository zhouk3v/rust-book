Crates come in two forms

- Binary crates are programs you can compile to an executable, these need a `main` function
- Library crates don't compile to an executable, instead they define functionality to be shared with multiple projects

The _crate root_ is a source file that the rust compiler starts from and makes up the root module of a crate

A _package_ is a bundle of one or more crates that provides a set of functionality. They contain a `Cargo.toml` file that describes how to build the crates

Cargo follows a convention that `src/main.rs` is the crate root of a binary crate, with the same name as the package

If the package directory contains `src/lib.rs`, then the package contains a library crate with the same name as the package.

A package can have multiple binary crates by placing files in the `src/bin` directory, where each file is a seperate crate
