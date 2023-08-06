# Freight Programming Language

A package-oriented programming language.

## Usage

Create a program in a `.f` file.

```typescript
import { IO } from "freight"

export package "hello" {
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
