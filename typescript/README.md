## Ch01: Getting to Know Typescript

- Item 2: start with `noImplicitAny` on
- Item 3: Code Generation is independent of Typescript types
  - `constructor(public width: number, public height: number)`
  - class introduces both a type and a value; interface only introduce a type
  - `as number` is a type operator, cannot affect runtime behavior
  - ways to query type at runtime: `Tagged unions`; `property checking`
  - `tagged unions`: `type Shape = Square | Rectangle;`
- Item 4: Structural Typing (duck typed)
  - have property beyond explicitly listed
  - facilitate unit testing
  - classes also follow structural typing rules
