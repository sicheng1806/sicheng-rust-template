# Sicheng Rust Project Template

A template for generating generic Rust engineering projects using the [cargo-generate](https://github.com/cargo-generate/cargo-generate).

## Features

- Configures `GitHub Actions`.
- Uses [cargo-clippy](https://github.com/rust-lang/rust-clippy) as the linter.
- Uses [cargo-fmt](https://github.com/rust-lang/rustfmt) as the code formatting tool.
- Uses `cargo test` for test.
- Uses [cargo-doc](https://doc.rust-lang.org/rustdoc/index.html) as the documentation generation tool.
- Uses [cargo-make](https://github.com/sagiegurari/cargo-make) as the automation tool.
- Uses [typos](https://github.com/crate-ci/typos) as the source code spell checker tool.
- Uses [codecov](https://about.codecov.io/) as the code coverage and quality tool.
- Uses [pre-commit](https://pre-commit.com/) to configure automated review scripts before `git commit`.

## How to Use

1. Install [cargo-generate]((https://github.com/cargo-generate/cargo-generate)).
2. Use the template:
```bash
cargo generate sicheng1806/sicheng-rust-template template
```

### Project Structure

```text
```

## Project Develop Tools

This project requires the following tools to be installed:

- [git](https://git-scm.com/) Git is a free and open source distributed version control system designed to handle everything from small to very large projects with speed and efficiency.
- [rust with cargo](https://www.rust-lang.org/) A language empowering everyone
to build reliable and efficient software.
- [pre-commit](https://github.com/pre-commit/pre-commit) A framework for managing and maintaining multi-language pre-commit hooks.

In addition to that, you also need to use `cargo install` to install some auxiliary tools:

- [cargo-make](https://github.com/sagiegurari/cargo-make) Rust task runner and build tool.
- [typos](https://github.com/crate-ci/typos) Finds and corrects spelling mistakes among source code.

### Initial Project

- `git init` to initial a git repository.
- `pre-commit install` to add git hooks.

### Format code style

- `cargo check` check the code.
- `cargo clippy` run the linter.
- `cargo fmt` run the formatter.

or do it by cargo-make: `cargo make lint`

### Check spelling mistakes

- `typos` to find spelling mistakes.
- `typos -w` to fix spelling mistakes.

### Test

- `cargo test` to run the test.

or do it by cargo-make: `cargo make test`

### Build document

- `cargo doc` to build the document.

or do it by cargo-make: `cargo make docs`

### Git tag

Modify package version value, then commit.

Add tag

```sh
git tag -a v0.1.0
```

### Build

Build this tag distribution package.

```sh
cargo build
```

or do it by cargo make: `cargo make build`
