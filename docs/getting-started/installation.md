---
title: Installation
layout: default
---

# Installation

Freight is a single binary compiled for all targets using Rust.

## Homebrew

If you use [Homebrew][] for either macOS or Linux, you can use our tap to get
the latest Freight releases as built binaries for your system:

    brew tap https://github.com/tubbo/freight
    brew install freight

## Manual

Download the latest release from [GitHub Releases][] for your OS, and install
the binary into an executable directory:

    unzip freight-$TARGET.zip
    sudo mv freight-$TARGET/freight /usr/bin/

## From Source

If you'd like to install Freight's latest trunk version, use the `main` branch
from our Git repository. You'll need [Rust][] and [Git][] installed, but that's
about it...

    git clone https://github.com/tubbo/freight
    cargo build --release

Run with:

    ./target/freight

[GitHub Releases]: https://github.com/tubbo/freight/releases
[Homebrew]: https://brew.sh
[Rust]: https://rust-lang.org
[Git]: https://git-scm.org
