---
layout: page
title: Types
---

# Types

Freight is a type-safe language, meaning you must specify the types of all
function arguments, return values, and declared variables inside your code. By
ensuring that types are specified inside the code, the compiler can optimize
certain code paths for better performance, as well as give better insight into
your programs as they are being developed.

The following types are available in all programs without needing to import:

- `Integer`
- `Float`
- `Boolean`
- `String`
- `Record<$KEY,$VALUE>`

## Structs

You can create custom object types using the `struct` keyword:

```typescript
struct Foo {
  bar: String
}
```

This will allow you to instantiate objects of type `Foo`:

```typescript
Foo { bar: 'foo bar' }
```

## Type Arguments

Arguments can be given to types in order to customize objects for other
use cases:

```typescript
struct Foo<Bar = String> {
  bar: Bar
}
```

You can now instantiate `Foo` with a custom type for `Foo.bar`:

```
Foo<Integer> { bar: 1 }
```
