---
title: Your First Freight Program
---

# "Hello, World!" - Your First Freight Program

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
