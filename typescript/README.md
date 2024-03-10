## Ch01: Getting to Know Typescript

### Item 2: start with `noImplicitAny` on

### Item 3: Code Generation is independent of Typescript types
  - `constructor(public width: number, public height: number)`
  - class introduces both a type and a value; interface only introduce a type
  - `as number` is a type operator, cannot affect runtime behavior
  - ways to query type at runtime: `Tagged unions`; `property checking`
  - `tagged unions`: `type Shape = Square | Rectangle;`

### Item 4: Structural Typing (duck typed)
  - have property beyond explicitly listed
  - facilitate unit testing
  - classes also follow structural typing rules; the same for `enum`

### Item 4: Limited user of the any type
  - `(as any)` is a type assertion

### Item 6: Editor and Language Services
  - autocomplete, inspection, navigation, and refactoring

### Item 7: Type as a set of values
  - this is the domain of the type
  - Itersection of types: 
    - `keyof (A&B) = (keyof A) | (keyof B)`
    - `keyof (A|B) = (keyof A) & (keyof B)`
  - `extends` as a constraint in a generic
    - `function sortBy<k extends keyof T, T>(vals: T[], key: K): T[] {...}`
  - `never`: empty set; `unknow`: universal set
  - `extends` means `assignable to`, `subtype of`, `subset of`
### Item 8: Type Space vs Value Space
  - many constructs have different means in two spaces (p.38)

### Item 9: prefer Type Declarations to Type Assertions

### Item 10: Avoid Object Wrapper Types
  - they are for the purpose to provid methods for primitive types
  - just use primitive types instead

### Item 11: Typescript has excessive property checking
  - see: `darkmode` vs `darkMode` example

### Item 12: Apply Types to Entire Function Expressions When Possible

  ```
  type BinaryFn = (a: number, b: number) => number;
  const add: BinaryFn = (a, b) => a + b;
  const sub: BinaryFn = (a, b) => a - b;
  ```

  ```
  const checkFetch: typeof fetch = async (input, init) => {...}
  ```

### Item 13: Know the Differences Between type and interface

- both support `index signature`
- `IState & { population: number }`
- there are `union types` but no union interface
- `declaration merging`: interface can be `augmented`: a good feature for library creators

### Item 14: Use Type Operations and Generics to DRY

```
type TopNavState = {
  userId: State['userId'];
  pageTitle: State['pageTitle'];
  recentFiles: State['recentFiles'];
}
```

```
type TopNavState = {
  [k in 'userId' | 'pageTitle' | 'recentFiles']: State[k]
}
```

```
type Pick<T, K> = { [k in K]: T[k] };
// or type Pick<T, K extends keyof T> = { [k in K]: T[k] };
type TopNavState = Pick<State, 'userId' | 'pageTitle' | 'recentFiles'>
```

```
type OptionUpdate = { [k in keyof Options]?: Options[k] }
```

```
type OptionKeys = keyof Options;
type OptionUpdate = { [k in OptionKeys]?: Options[k] }
```

standard library defines generic types for common patterns, e.g. create a named type for inferred return value

```
type UserInfo = ReturnType<typeof getUserInfo>;
```

- Generic types are the equivalent of functions for types

### Item 15: Use Index Signature for Dynamic Data

- the name for the keys is purely for documentation
- common use case if for parse CSV data
- Records type

can use 'undefined' for safer check

```
function safeParseCSV(input: string): { [colName: string]: string | undefined }[] {...}
```

```
type ABC = {[k in 'a' | 'b' | 'c']: k extends 'b' ? string: number}
```
