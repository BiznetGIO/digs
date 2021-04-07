<div align="center">
<h1>digs</h1>

dig many at once.

<a href="https://github.com/BiznetGIO/digs/actions/workflows/ci.yml">
<img src="https://github.com/BiznetGIO/digs/workflows/ci/badge.svg">
</a>

<p></p>

![A digs demo](docs/demo.gif)

</div>

---

*digs* is a DNS command-line client that able to query many DNS servers at once.

## Features

- Prevent invalid input before querying. Such invalid records type or configuration.
- No panics, good error handling.
- [more faster](docs/benchmark.md) compared to previous `digs.py`.
- Colourful output.
- Cross-platform.
- Single binary.

## Usage

Prepare a configuration file that should look like this:

``` toml
[[servers]]
ip = "8.8.8.8"
name = "Google"

[[servers]]
ip = "9.9.9.9"
name = "Quad9"
```

The server can be as many as you want.

Example commands:

``` 
digs example.net A                    Query a domain using the configuration in current directory
digs example.net A -f custom.toml     ...using custom configuration
```

Run `digs --help` to see more available options.

## Installation

### Binary releases

Download the binary from the [release page](https://github.com/BiznetGIO/digs/releases)

### With cargo (from source)

``` bash
$ git clone 
$ cd digs
$ cargo install --path .
```


---

## Licence

digs source code is licensed under the [GPLv3](https://choosealicense.com/licenses/gpl-3.0/).
