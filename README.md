![Avatar](avatar.jpg)

[![Build Status](https://github.com/cliffano/crust/actions/workflows/ci-workflow.yaml/badge.svg)](https://github.com/cliffano/crust/actions/workflows/ci-workflow.yaml)

# Crust

Crust is a Makefile for building Rust packages.
It provides utility targets for linting, testing, and publishing Rust binary crates (CLI) and library crates.

Have a look at [examples/](examples/) for example projects which use Crust.

## Installation

1. Download `src/Makefile-crust` as the `Makefile` of your project: `curl https://raw.githubusercontent.com/cliffano/crust/main/src/Makefile-crust -o Makefile`
2. Create configuration file `crust.yml` with properties described in [Configuration](#configuration) section
3. Run the available `Makefile` targets described in [Usage](#usage) section

## Configuration

Create a Crust configuration file called `crust.yml` which contains the following properties:

| Property | Description | Example |
|----------|-------------|---------|
| package_name | The name of the Rust package, matching `Cargo.toml`'s `package.name` | `someapp` |
| author | The author of the package | `Some Author` |

## Usage

The following targets are available:

| Target | Description |
|--------|-------------|
| ci | CI target to be executed by CI/CD tool, end to end build of the Rust package |
| stage | Ensure `stage/gh-pages/` directory exists |
| clean | Remove all temporary (staged, generated) files, including the `target/` build cache |
| deps | Install Clippy/rustfmt toolchain components and supporting cargo subcommands |
| deps-upgrade | Upgrade dependencies using `cargo update` |
| deps-extra-apt | Install extra tools using `apt`: markdownlint |
| rmdeps | `cargo clean` (Cargo doesn't vendor dependencies locally) |
| update-to-latest | Update Makefile to the latest version tag |
| update-to-main | Update Makefile to the main branch |
| update-to-version | Update Makefile to the version defined in `TARGET_crust_VERSION` parameter |
| style | Format code using [rustfmt](https://github.com/rust-lang/rustfmt) |
| lint | Check formatting, run [Clippy](https://doc.rust-lang.org/clippy/), [cargo-audit](https://github.com/rustsec/rustsec), and [markdownlint](https://github.com/DavidAnson/markdownlint) |
| test | Run unit tests (inline `#[cfg(test)]` modules) using `cargo test --lib --bins` |
| test-integration | Run integration tests under `tests/` using `cargo test --test '*'` |
| test-examples | Run every example in `examples/*.rs` via `cargo run --example` |
| coverage | Generate HTML and lcov coverage reports using [cargo-llvm-cov](https://github.com/taiki-e/cargo-llvm-cov) |
| complexity | Generate a complexity report using [rust-code-analysis](https://github.com/mozilla/rust-code-analysis) |
| doc | Generate API documentation using `cargo doc` |
| package | Build the release binary and the distributable crate package |
| publish | Publish the crate to [crates.io](https://crates.io) |
| release-major | Create a major release using [rtk](https://github.com/cliffano/rtk) |
| release-minor | Create a minor release using [rtk](https://github.com/cliffano/rtk) |
| release-patch | Create a patch release using [rtk](https://github.com/cliffano/rtk) |

## Colophon

Related Projects:

* [generator-rust](https://github.com/cliffano/generator-rust) - Rust projects generator using Plop
