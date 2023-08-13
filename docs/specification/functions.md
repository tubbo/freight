---
layout: page
title: Functions
parent: Language Specification
---

# Functions

Functions are first-class citizens in Freight. This means you can assign a
function to a variable, pass functions into other functions, and return
functions from other functions. They are also the main way of performing
logic in Freight.

A function can be defined using the `function` keyword:

```typescript
function example() {
  // your code here
}
```

As with all other structures in Freight, functions are only available in the
scope in which they're defined unless you `export` it:

```typescript
export function public() {
  // your public code here
}
```

Defining a function also defines its type. A function's type is made up of its
name, arguments, and return value:

```typescript
export function handler(request: Request) -> Response {
  new Response {
    // ...
  }
}
```

You can use this type in other functions if you'd like to pass it in:

```typescript
function listen(port: Integer, handler: handler) {
  // ... call the handler when a request comes in ...
}
```

Functions can also be `abstract`, so you can define a specification for a
callback function of some kind:

```typescript
abstract function Handler(request: Request) -> Response
```
