<div align="center">
<h1>digs</h1>

dig many at once.

<a href="https://github.com/BiznetGIO/digs/actions/workflows/ci.yml">
<img src="https://github.com/BiznetGIO/digs/actions/workflows/ci.yml/badge.svg">
</a>
<a href="https://crates.io/crates/digs">
<img src="https://img.shields.io/crates/v/digs.svg">
</a>

<p></p>

![A digs demo](docs/demo.gif)

</div>

---

_digs_ is a DNS command-line client that is able to query many DNS servers at once.

## Why?

We work with DNS records a lot. Having a tool that inspects multiple
records across different machines at once is a lifesaver.

## Features

- Prevent invalid input before querying, such as invalid record types or configuration.
- No panics, good error handling.
- [much faster](docs/benchmark.md) compared to previous `digs.py`.
- Fancy error messages and colorful output.
- Cross-platform and single binary.

## Usage

Prepare a configuration file that should look like this:

```toml
[[servers]]
address = "8.8.8.8"
name = "Google"

[[servers]]
address = "9.9.9.9:54" # Custom port, default: 53
name = "Quad9"
```

The `servers` blocks can be as many as you want.

Example commands:

```
digs example.net A                         Query a domain using the configuration in the current directory
digs example.net A --config custom.toml    ...using custom configuration
```

Run `digs --help` to see more available options.

## Installation

### From binaries

The [release page](https://github.com/BiznetGIO/digs/releases) includes
pre-compiled binaries for GNU/Linux, macOS, and Windows.

### From source

Using [cargo-binstall](https://github.com/cargo-bins/cargo-binstall)

```bash
cargo binstall digs
```

Using Rust's package manager [cargo](https://github.com/rust-lang/cargo):

```bash
cargo install digs
```

## Development

```bash
git clone https://github.com/BiznetGIO/digs
cd digs

# Run unit tests and integration tests
cargo test

# Install
cargo install --path .
```

## Contributing

To learn more read the [contributing guide](docs/dev/README.md)

## Licence

digs source code is licensed under the [MIT](https://choosealicense.com/licenses/mit/).
