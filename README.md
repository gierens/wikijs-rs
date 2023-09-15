<div align="center">

# wikijs-rs

![](https://raw.githubusercontent.com/gierens/wikijs-rs/readme/logo/logo-small.png)

API bindings, CLI client and FUSE filesystem for Wiki.js written in Rust.

[![Tests](https://github.com/gierens/wikijs-rs/actions/workflows/testing.yml/badge.svg)](https://github.com/gierens/wikijs-rs/actions/workflows/testing.yml)
![Crates.io](https://img.shields.io/crates/v/wikijs?link=https://crates.io/crates/wikijs)
![Crates.io](https://img.shields.io/crates/l/wikijs?link=https://crates.io/crates/wikijs)

</div>

**What's inside?**
- [Library](#library): Rust bindings to Wiki.js's entire GraphQL API as well
  as asset up- and download via the REST API.
  **Usable but not battle-tested**
- [CLI](#cli): Command-line client for those bindings allowing things like
  editing pages with your editor.
  **Usable but not complete yet**
- [Filesystem](#filesystem): A FUSE filesystem to mount your wiki and work on it locally.
  **Not usable yet, heavily work-in-progress.**

## Library

The library basically gives you a struct `Api` that you hand the url and
credentials for you Wiki.js instance to, and that exposes functions for all
the different GraphQL queries and mutations as well as asset down- and upload
via the REST API. For detailed information check the
[documentation](https://docs.rs/wikijs), otherwise here a short example:

### Usage

Add the following dependency to your `Cargo.toml`:
```toml
[dependencies]
wikijs = "0.1"
```
Create an instance of `Api` and use its functions:
```rust
use wikijs::{Api, Credentials};

let api = Api::new(
    "http://localhost:3000".to_string(),
    Credentials::Key("my-api-key".to_string()),
);
println!("{:?}", api.page_get(1).unwrap());

```
## CLI

This crate ships a command-line tool also called `wikijs` to directly interact
with your Wiki.js instance from the CLI. On top of exposing the library
functions it also offers additional functionality like editing pages with
your favorite editor.

### Build
```bash
cargo build --features=cli
```

### Install
```bash
cargo install wikijs --features=cli
```

### Usage
The tool takes the URL and credentials either from the arguments or
environment variables. For the latter option use something like this:
```bash
export WIKI_JS_BASE_URL=https://wiki.mydomain.com
export WIKI_JS_API_KEY=MY-SUPER-SECRET-API-KEY
```
Then you can for example create a page named `test`, list pages and edit
it with:
```bash
wikijs page create test
wikijs page list
wikijs page edit <ID>
```
where the `ID` is found in the `page list` output.

## Filesystem

**WARNING: Not really usable yet! Careful!**

This crate also ships FUSE filesystem called `wikifs` to mount your Wiki.js
instance locally and view and manipulate it with what ever programs you like.


### Build
```bash
cargo build --features=fuse
```

### Install
```bash
cargo install wikijs --features=fuse
```

### Usage
The tool takes the URL and credentials either from the arguments or
environment variables. For the latter option use something like this:
```bash
export WIKI_JS_BASE_URL=https://wiki.mydomain.com
export WIKI_JS_API_KEY=MY-SUPER-SECRET-API-KEY
```
Then you can mount the filesystem like so:
```bash
mkdir /tmp/wikijs
wikifs /tmp/wikijs
```
And in another terminal use it like this:
```bash
cd /tmp/wikijs
ls
cat test.md
```
provided you have a `markdown` page located at `/test` in your wiki.

## Contributing

Two parts where especially first-time contributions should be fairly easy and
are also really needed is writing docstrings and integration tests. Apart from
that the CLI still needs to implement many of the library functions.

Please check issues and PRs first and maybe make an issue or draft PR of your
own so we can coordinate work. This is especially important for the FUSE
filesystem, as I'm currently heavily refactoring that.
