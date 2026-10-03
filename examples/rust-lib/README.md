<!-- BEGIN:AVATAR -->
![Avatar](avatar.jpg)
<!-- END:AVATAR -->

<!-- BEGIN:BADGES -->
[![Build Status](https://github.com/cliffano/crust/workflows/CI/badge.svg)](https://github.com/cliffano/crust/actions?query=workflow%3ACI)
[![Dependencies Status](https://img.shields.io/librariesio/release/cargo/rustlibexample)](https://libraries.io/cargo/rustlibexample)
[![Code Scanning Status](https://github.com/cliffano/crust/workflows/CodeQL/badge.svg)](https://github.com/cliffano/crust/actions?query=workflow%3ACodeQL)
[![Coverage Status](https://coveralls.io/repos/github/cliffano/crust/badge.svg?branch=main)](https://coveralls.io/r/cliffano/crust?branch=main)
[![Security Status](https://snyk.io/test/github/cliffano/crust/badge.svg)](https://snyk.io/test/github/cliffano/crust)
[![Published Version](https://img.shields.io/crates/v/rustlibexample.svg)](https://crates.io/crates/rustlibexample)
<!-- END:BADGES -->

# Rust Lib Example

Rust Lib Example is a Rust Lib example project .

## Installation

```bash
cargo add rustlibexample
```

## Usage

Create a configuration file, e.g. `rustlibexample.yaml`:

```yaml
---
text: Hello World
```

Create a `Display` and format its message:

```rust
use rustlibexample::display::Display;

let display = Display::new("rustlibexample.yaml").unwrap();
let text = display.format(false, "lower");
println!("{text}");
```

## Configuration

These are the configuration properties that you can use with `rustlibexample`.
Some example configuration files are available on [examples](examples) folder.

| Property | Type | Description | Example |
|----------|------|-------------|---------|
| `text` | String | The message text | Hello World |

## Colophon

<!-- BEGIN:DEVELOPERS_GUIDE -->
[Developer's Guide](https://cliffano.github.io/developers-guide-rust.html)
<!-- END:DEVELOPERS_GUIDE -->

<!-- BEGIN:BUILD_REPORTS -->
Build reports:

* [Code complexity report](https://cliffano.github.io/crust/complexity/)
* [Test coverage report](https://cliffano.github.io/crust/coverage/index.html)
* [API Documentation](https://cliffano.github.io/crust/doc/doc/rustlibexample/index.html)

<!-- END:BUILD_REPORTS -->
