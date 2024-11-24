# Get Started

This project requires the following tools to be installed:

- [git](https://git-scm.com/) Git is a free and open source distributed version control system designed to handle everything from small to very large projects with speed and efficiency.
- [rust with cargo](https://www.rust-lang.org/) A language empowering everyone
to build reliable and efficient software.
- [pre-commit](https://github.com/pre-commit/pre-commit) A framework for managing and maintaining multi-language pre-commit hooks.

In addition to that, you also need to use `cargo install` to install some auxiliary tools:

- [cargo-make](https://github.com/sagiegurari/cargo-make) Rust task runner and build tool.
- [typos](https://github.com/crate-ci/typos) Finds and corrects spelling mistakes among source code.
- [Tarpaulin](https://github.com/xd009642/tarpaulin) A code coverage reporting tool for the Cargo build system.

## Initial Project

- `git init` to initial a git repository.
- `pre-commit install` to add git hooks.

## Format code style

- `cargo check` check the code.
- `cargo clippy` run the linter.
- `cargo fmt` run the formatter.

or do it by cargo-make: `cargo make lint`

## Check spelling mistakes

- `typos` to find spelling mistakes.
- `typos -w` to fix spelling mistakes.

## Test

- `cargo test` to run the test.

or do it by cargo-make: `cargo make test`

## Build document

- `cargo doc` to build the document.

or do it by cargo-make: `cargo make docs`

## Git tag

Modify package version value, then commit.

Add tag

```sh
git tag -a v0.1.0
```

## Build

Build this tag distribution package.

```sh
cargo build
```

or do it by cargo make: `cargo make build`
