# tygrep

Type signature based code exploration.

Search for functions based by the type signature.
This tool is heavily inspired by Haskell's [Hoogle](https://hoogle.haskell.org/), but aims
to provide a similar experience in other languages.


## Usage

This is a CLI application, and the main way to interact with it by passing a query:

```sh
tygrep 'String -> String'
```


## Installation

For now the best / only way to install the application is using `cargo` from the
root of the directory:

```sh
cargo install
```

## Development

This is a Rust based application, so you will need a normal Rust toolchain with `cargo`.


## How this works

`tygrep` offers a DSL to specify type signatures that works across languages, and then uses 
existing tools to generate a language agnostic representation of the source code, and then
a search engine on top of that. 
See [design](docs/design.md) for more information on the internals.

