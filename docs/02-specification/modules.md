---
layout: page
title: Modules
parent: Language Specification
---

# Modules

A module is a collection of functions and/or structs that correspond to the
same concept. Modules are a way of organizing project code, and should be used
prior to refactoring code into separate packages, as modules within the same
package are all accessible to each other without needing to export. Modules
are specified using `PascalCase`, and they can be nested within each other
to form "sub-modules". It should be noted that sub-modules of exported
modules must also be exported themselves:

```typescript
export package 'foo' {
  export module Bar {
    export function hello() {
      Priv.do_something()
    }
  }
  module Priv {
    // this code is not accessible to users of `foo`
    function do_something() {
      // ...
    }
  }
}
```

A module can also have its own collection of submodules:

```typescript
package 'foo' {
  module Bar {
    module Baz {
      function hello() {
        // ...
      }
    }
  }
}
```

Both packages and modules are separated in code and runtime notation with the
`::` operator:

```typescript
package 'foo' {
  function hello() {
    Bar::Baz::hello()
  }
}
```

To refer to the current package/module scope absolutely, the `this` keyword
can be used:

```typescript
package 'foo' {
  function hello() {
    this::Bar::Baz::hello()
  }
}
`
```
