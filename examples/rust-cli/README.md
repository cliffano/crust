<!-- BEGIN:AVATAR -->
![Avatar](avatar.jpg)
<!-- END:AVATAR -->

<!-- BEGIN:BADGES -->
[![Build Status](https://github.com/cliffano/crust/workflows/CI/badge.svg)](https://github.com/cliffano/crust/actions?query=workflow%3ACI)
[![Dependencies Status](https://img.shields.io/librariesio/release/cargo/rustcliexample)](https://libraries.io/cargo/rustcliexample)
[![Code Scanning Status](https://github.com/cliffano/crust/workflows/CodeQL/badge.svg)](https://github.com/cliffano/crust/actions?query=workflow%3ACodeQL)
[![Coverage Status](https://coveralls.io/repos/github/cliffano/crust/badge.svg?branch=main)](https://coveralls.io/r/cliffano/crust?branch=main)
[![Security Status](https://snyk.io/test/github/cliffano/crust/badge.svg)](https://snyk.io/test/github/cliffano/crust)
[![Published Version](https://img.shields.io/crates/v/rustcliexample.svg)](https://crates.io/crates/rustcliexample)
<!-- END:BADGES -->

# Rust CLI Example

Rust CLI Example is a Rust CLI example project .

## Installation

```bash
cargo install rustcliexample
```

## Usage

Create a configuration file, e.g. `rustcliexample.yaml`:

```yaml
---
text: Hello World
```

Run rustcliexample with display command:

```bash
rustcliexample display
```

Run rustcliexample with specified config file:

```bash
rustcliexample --config-file rustcliexample.yaml display
```

Run rustcliexample with specified config file and custom flags:

```bash
rustcliexample --config-file rustcliexample.yaml display --reverse true --transform upper
```

Show help guide:

```bash
rustcliexample --help
```

## Configuration

These are the configuration properties that you can use with `rustcliexample` CLI.
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
* [API Documentation](https://cliffano.github.io/crust/doc/doc/rustcliexample/index.html)

<!-- END:BUILD_REPORTS -->
