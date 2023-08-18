# Installation

The Freight compiler and toolchain is distributed as a single native binary,
but contains other elements (such as the standard library) that must be
installed onto the machine. You can install Freight in a number of different
ways, depending on your machine configuration.

## Homebrew

The easiest way to get Freight installed is by using our [Homebrew][] tap, which
is located in this repository. To download the tap, run:

```bash
brew tap tubbo/freight --url 'https://github.com/tubbo/freight'
```

You can now install the `freight` CLI for your system by running:

```bash
brew install freight
```

## Manual

Precompiled binaries are distributed for Linux, macOS, and Windows through
[GitHub Releases][]. If you aren't using Homebrew, you can download the `.zip`
manually from this page and install it to your system.

Instructions for installing on each system is located within this package.

## Cargo

You can also download Freight from <https://crates.io>. Freight is distributed
as a [Rust][] crate since it has a library available to other developers who
want to embed Freight within their Rust program. As such, Freight can be
installed with Cargo by running:

```bash
cargo install freight
```

[GitHub Releases]: https://github.com/tubbo/freight/releases
[Homebrew]: https://brew.sh
[Rust]: https://rust-lang.org
