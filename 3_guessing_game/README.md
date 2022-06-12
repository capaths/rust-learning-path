## Guessing game

- Some items in the standard library are brought to the scope of every program,
these are called the **prelude**.
- Those that are not must be imported explicitly using a ``use`` statement.
- By default, variables are immutable, but you can make them mutable by prefixing
them with ``mut``.
- Functions after a type and two colons are called **associated functions**, like
``String::new()``, which is an associated function of the type ``String``.

- To use a crate (collection of Rust source code files), you must add a
dependency to ``Cargo.toml``, and then build the project using ``cargo build`` that will install that dependency, and that dependency's dependencies.
- Crates can be found at [crates.io](https://crates.io/).
- Rust has a strong, static type system