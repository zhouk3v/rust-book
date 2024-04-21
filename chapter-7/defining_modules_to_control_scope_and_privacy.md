Cheat sheet for modules:

- The compiler first looks in the crate root (usually src/lib.rs for a library crate or src/main.rs for a binary crate)
- You can declare new modules in the crate root file (e.g. `mod garden;`). The compiler will look for the module's code in the following:
  - Inline - between the curly brackets { } following `mod <module name>` (Do not put the ; in)
  - In the file `src/<module name>.rs`
  - In the file `src/<module name>.mod.rs`
- In any file other than the crate root, you can declare submodules (e.g. `mod vegetables;` in `src/garden.rs`). The compiler will look for the submodule's code in the following:
  - Inline - between the curly brackets { } following `mod <submodule name>` (Do not put the ; in)
    - In the file `src/<module name>/<submodule name>.rs`
  - In the file `src/<module name>/<submodule name>.mod.rs`
- Once a module is part of your crate, you can refer to code in that module from anywhere in the same crate (if the privacy rules allow it), using the path to the code.
  - Eg: An `Asparagus` type in the garden vegetables module would be found at `crate::garden::vegetables::Asparagus`
- Code within a module is private from its parent modules by default. Public modules are declared with `pub mod` instead of `mod`. To make items public within a public module, use `pub` before their declarations
- Within a scope, the `use` keyword creates shortcuts to items to reduce repetition of long paths.
  - E.g: In any scope that can refer to `crate::garden::vegetables::Asparagus`, you can create a shortcut `use crate::garden::vegetables::Asparagus` and then refer to it as `Asparagus` later on

Modules allow us to organize code within a create for readability and reuse and control the privacy of items. Items in modules are private by default, but we can choose to make modules and the items in them public, which will allow external code to use and depend on them.

Modules can hold structs, enums, constants, traits, functions and other modules

By using modules, we can group related definitions together and name why they're related.

`src/main.rs` and `src/lib.rs` are called crate roots, because the contents of either of thses two files form a module called `crate` at the root of the crate's module structure, known as the _module tree_

If module A is contained inside module B, then module A is the _child_ of module B and that _module_ B is the parent of module A.

Modules defined in the same module are _siblings_ of each other.
