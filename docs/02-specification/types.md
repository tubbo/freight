---
layout: default
title: Types
parent: Language Specification
---

# Types

Freight is a type-safe language, meaning you must specify the types of all
function arguments, return values, and declared variables inside your code. By
ensuring that types are specified inside the code, the compiler can optimize
certain code paths for better performance, as well as give better insight into
your programs as they are being developed.

The following types are available in all programs without needing to import:

- `Number`
- `Boolean`
- `String`

## Structs

You can create custom product types using the `struct` keyword:

```typescript
struct Foo {
  bar: String
}
```

This will allow you to instantiate objects of type `Foo`:

```typescript
new Foo { bar = 'foo bar' }
```

## Enums

Freight also supports sum types by use of the `enum` keyword:

```typescript
enum Result {
  Ok { value: String },
  Error
}
```

This sum type can be evaluated by pattern matching:

```typescript
function fn_that_returns_result() -> Result {
  if (!valid()) {
    new Result::Error
  } else {
    new Result::Ok { value = "foo" }
  }
}

value = match fn_that_returns_result() {
  Ok(value) => { "it worked!" }
  Error(error) => { "error" }
}
```

Sum types defined with `enum` allow functions to return different types of
values depending on the execution.

## Type Arguments

Arguments can be given to types in order to customize objects for other
use cases:

```typescript
struct Foo<Bar = String> {
  bar: Bar
}
```

You can now instantiate `Foo` with a custom type for `.bar`:

```
foo = new Foo<Integer> { bar: 1 }
```

## Abstract Types

You can designate types, functions, and even entire modules as `abstract` to
specify an interface to your library without writing dummy code.

```typescript
export package 'abstract' {
  export module Foo {
    abstract function Handler(input: String) -> Number

    export function Server(handler: Handler) {
      // compiler will assert that the function passed in implements `Handler`
    }
  }
}
```

Abstract modules are somewhat special, in that they designate everything under
their scope to be abstract if they are themselves abstract:

```typescript
export package 'abstract' {
  export abstract module Bar {
    // you don't need the `abstract` keyword for this function
    function Handler(input: String) -> Integer
  }
}
```

As such, modules cannot "know" if they are implementing a given interface, so
you can "teach" the compiler that a module should be implementing an abstract
module by using the `implements` keyword:

```typescript
export package 'abstract' {
  export abstract module Interface {
    function send(message: Message) -> Boolean
    function receive() -> String
  }

  export module Concrete implements Interface {
    // this will error if exportable send/receive implementations do not exist.
  }
}
```
