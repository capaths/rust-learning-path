# Hello, Cargo!

- Cargo is a build system for Rust and package manager.
- Cargo can:
  - Build Rust code.
  - Download **dependencies** (libraries).
  - Building those dependencies.
  - More stuff!

- Cargo generates a `Cargo.toml` file in the root of the project. This file contains
information about the project and its dependencies (kinda like package.json on Node).

- Cargo can initialize a directory structure for a project using:
```bash
cargo new <project_name>
```

- Cargo can build the project to `target/debug/<project_name>` using:
```bash
cargo build
```

- And then run the project using (this command also rebuilds the project if the
source code changes):
```bash
cargo run
```

- To check the code if it compiles without building the project, you can use:
```bash
cargo check
```

- To build for a release (i.e. with optimizations), you can use:
```bash
cargo build --release
```