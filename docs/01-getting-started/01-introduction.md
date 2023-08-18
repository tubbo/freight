---
title: Introduction
---

# The Freight Programming Language

Welcome to the documentation for **Freight**, the package-oriented programming
language! Freight is a compiled and type-safe language for general programming,
and is centered around the concept of supply chain management and reusable code.

## Why Freight?

Most, if not all, programming languages tend to consider package management and
the supply chain as an afterthought. Even languages with "official" package
managers and supply chain systems do not actually have syntax support for these
endeavors, instead entrusting such concepts to a third-party set of developers.
Freight was conceived to emphasize the idea of reusable code as a "first-class"
citizen of the language, rather than an afterthought, in order to help combat
the issues programmers face with supply chain attacks and increased strain
on repositories for delivering such packages.

## Hello, World!

Here's a simple "hello world" program in Freight:

```typescript
import { IO } from "freight"

export package "hello" {
  export function world() {
    IO.print_line("hello world")
  }
}
```

To run this program, use `freight run` and specify the location of the function
that is to be executed:

```sh
$ freight run 'hello::world()'
hello, world!
```

You can also compile the program to a binary, and run the built binary instead:

```sh
$ freight build 'hello::world()' ./hello-world
$ ./hello-world
hello, world!
```

If you're making a library that is designed to be consumed by other programs,
you can publish your package to a registry:

```sh
$ freight ship 'hello' 'v1.0.0'
```