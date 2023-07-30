# Freight Programming Language

A package-oriented programming language.

```typescript
import { print } from "freight.io"

export package "hello" {
  export module World {
    export function hello_world() {
      print("hello world")
    }
  }
}
```

```sh
freight run "hello/World#hello_world()"
```
