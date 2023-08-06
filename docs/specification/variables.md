---
layout: default
title: Variables
---

# Variables

Variables are declared and stored using the `=` operand. They're immediately
preceded by their type, as illustrated below:

```typescript
Integer x = 0
```

A variable cannot change type within the same program, or a compiler error will
be thrown.

## Mutability

All variables are **immutable by default**, so a special `mutable` keyword must
be used to mark a variable as mutable. This mutability only extends as far as
the current scope, so it cannot be mutated outside of the package or module
for example...

```typescript
mutable Integer x = 0
x = 1
```
