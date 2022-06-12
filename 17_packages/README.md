# Managing Growing Projects

- Organize code by splitting it into multiple modules and then multiple files.
- A **package** can contain multiple **binary crates** and optionally one **library crate**.
- For very large projects, workspaces can be used to organize code.

Features:

- **Packages**: A Cargo feature that lets you build, test, and share crates
- **Crates**: A tree of modules that produces a library or executable
- **Modules** and use: Let you control the organization, scope, and privacy of paths
- **Paths**: A way of naming an item, such as a struct, function, or module.

## Packages and Crates

- **Package**: One or more crates that provide some functionality. A package contains
a Cargo.toml file that describes the crate and its dependencies.
- **Crate**: Either a library **crate** or **binary** crate:
  - **Binary**: Can be compiled to an executable program. Must have a `main` function.	
  - **Library**: Aren't compiled to an executable and define functionality that can be used by multiple projects.
- **Crate Root**: The source file that the Rust compiled starts from.

### Rules

- A package can contain at most one library crate.
- A package can contain many binary crates.
- A package must contain at least one crate.

## What Cargo does

- Understands ``src/main.rs`` as the crate root of a binary crate.
- Undertands ``src/lib.rs`` as the crate root of a library crate.
- In a new project, Cargo creates one package with a binary crate.

## Modules

- ``use`` brings in items from other crates.
- ``pub`` makes items available to other crates.

### Rules

- The compiler will first look at the crate root (``src/lib.rs or src/main.rs`` usually).
- With ``mod <module_name>;``, the compiler will look for a module with that name at:
  - **Inline**: Directly following the ``mod`` statement with curly brackets.
  - **In a file**: In the file ``src/<module_name>.rs``.
  - **In a directory**: In the file ``src/<module_name>/mod.rs``.
- With ``mod <submodule_name>;`` inside a module, the compiler will look for a module with that name at:
  - **Inline**: Directly following the ``mod`` statement with curly brackets.
  - **In a file**: In the file ``src/<module_name>/<submodule_name>.rs``.
  - **In a directory**: In the file ``src/<module_name>/<submodule_name>/mod.rs``.
- If the privacy allows it, an item can be imported with the path ``crate::<module_name>::<submodule_name>::<item_name>;``.
- To make a public module, use ``pub mod <module_name>;``.
- The ``use`` path allows to import items from other crates with a shorter name, like ``use crate::<module_name>::<submodule_name>::<item_name>;`` will allow to use the item with just``<item_name>``.