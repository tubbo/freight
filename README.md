<h1 align="center">
  <a href="https://tubbo.github.io/freight">
    Freight
  </a>
</h1>
<h2 align="center">
  The package-oriented programming language.
</h2>

Freight is a general purpose programming language designed for the modern era.

## Usage

Create a program in a `.freight` file:

```typescript
import { IO } from freight

export package hello {
  export function world() {
    IO.print_line("hello world")
  }
}
```

Run the program:

```sh
$ freight run "hello::world()"
hello world
```

Build to a binary:

```sh
$ freight build "hello::world()" -o hello
$ ./hello
hello world
```

### As a Library

Freight is distributed over https://crates.io, and thus can be used in your Rust program as a library.

Find out more at https://docs.rs/freight
