---
layout: default
title: Packages
---

# Packages

All Freight code is organized into **packages**. A package is any unit of code
that is collected together within a set of files. Unlike most compilers,
Freight doesn't consider files as a "boundary". Instead, it collects all files
in the current directory together, and uses the `package` and `module` keywords
within the source to form the boundaries that a programmer specifies.

A package is defined using the `package` keyword, and the name of the package:

```typescript
package 'foo' {
  // your code here
}
```

You can specify `package 'foo' { ... }` in any number of files, and they will
all be collected together as part of the same codebase. A single project (or
working directory) can have any number of packages as is desired.

## Imports and Exports

The `import` and `export` keywords are crucial to understanding the packaging
system of Freight. Similar to Python or JavaScript, these keywords specify the
modules you wish to import and the code that should be available outside of its
package, respectively.

You can import a module like so:

```typescript
import { Bar } from 'foo'
```

And export code like this:

```typescript
export package 'foo' {
  export module Bar {
    // your code here
  }
}
```

## Modules

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
