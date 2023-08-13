---
layout: default
title: Packages
parent: Language Specification
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

There are 2 parts to an `import` statement:

```typescript
import Product from 'source' [@ vConstraint]
```

1. The **product** of the import, which is the first thing mentioned. This is
   what will be used in your codebase to refer to what is being imported.
2. The **source** of the import, which is everything after the `from` keyword.
   Package sources are either a URL to the package's relative location, or just
   the name of the package. In the latter case, packages will be looked up in
   all sources specified in your configuration.

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

Entire packages can be imported as well:

```typescript
import foo from 'foo'
```

## Versioning

By default, Freight will use the latest version of each package you import. To
control this behavior, you can specify a version constraint in the identifier:

```typescript
import legacy from 'foo' @ v1.0.0
```

This behavior is also extended to self-hosted package bundles:

```typescript
import pkg from 'https://example.com/pkg' @ v1.0.0
```

Version constraints do not have to be an exact specification. They can also
specify partial versions, which will be expanded into the latest version that
matches the constraint:

```typescript
import legacy from 'foo' @ v1.x
```

These partial version specifiers are as follows:

- **`vN.N.N`:** Match the exact specified version.
- **`vN.N.x`:** Match the specified minor version, but use the latest patch.
- **`vN.x`:** Match the specified major version, but use the latest minor.

## Publishing

Freight can resolve package names

Freight implements a mostly decentralized model for resolving 3rd-party
package names.

Packages are published to a registry, which is a directory of `.zip` files
containing all hosted package code.
