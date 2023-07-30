# Freight Programming Language

A package-oriented programming language.

## Usage

Create a program in a `.f` file. 

```typescript
import { IO } from "freight"

export package "hello" {
  export module World {
    export function hello_world() {
      IO.print("hello world")
    }
  }
}
```

Run the program:

```sh
$ freight run "hello::World.hello_world()"
hello world
```

Build to a binary:

```sh
$ freight build "hello::World.hello_world()" -o hello
$ ./hello
hello world
```

Install packages:

```sh
$ freight install "https://url.to/package"
```

Packages are installed globally, but can be referenced by version in your source code.

