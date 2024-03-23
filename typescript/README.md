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

### Item 16: Arrays are Objects, so keys are strings

### Item 17: Use readonly to Avoid Errors Associated with Mutations

- `readonly number[]` is a type distinct from `number[]`, `number[]` is a subtype
- if your function does not mutate its parameters, declare them `readonly`
- readonly is contagious: mark all functions it calls to `readonly`; or, (`param as number[]`)
- `const` vs `let + readonly`: trade one sort of mutability for another
  - `const`: cannot change variable it point to
  - `let + readonly`: the arrays themselve is not allowed to change
- `readonly` is shallow

```
let obj: {readonly [k: string]: number} = {}; // Readonly<{[k:string]: number}>
obj.hi = 45;   // error

obj = {...obj, hi:12}; //Ok
```

### Item 18: Use Mapped Types to Keep Related Values in Sync

```
const REQUIRES_UPDATE: {[k in typeof ScatterProps]: boolean} = {
  xs: true,
  ys: true,
  ...
  onClick: false,
}
```

### Item 19: Avoid Cluttering Your Code with Inferable Types

- Ideal code includes type annotations for function/method signatures but not for the local variables
- keep noise to the minimum
- focus on the implementation logic
- Writing the full type signature first helps get you the function you want: `Promise<number>` example.
- As the complexity of inferred return type increase, better provide a name

### Item 20: Avoid resuing variables for differently typed values

### Item 21: Understand Type Widening

- `type` = a set of possible values
- Widening: decide on a set of possible values from the single value specified.

### Item 22: Understand Type Narrowing

- null checking
- Array.isArray
- tagged union and discriminated union
- `user-defined type guard`

```
function isInputElement(el: HTMLElement): el is HTMLInputElement {
  return 'value' in el;
}

function getElementContent(el: HTMLElement) {
  if (isInputElement(el)) return el.value;
  return el.textContent;
}
```

### Item 23: Create Objects All at Once

- spread operator: `...`
- conditionally add properties to an object

### Item 24: Be Consistent in Your Use of Aliases

- Aliasing can prevent TypeScript from narrowing types
- Use destructuring syntax to encourage consistent naming

### Item 25: Use `async` Functions Instead of Callbacks for Asynchronous Code

- `pyramid of doom`
- A `sync` function always returns a Promise
- if a function returns Promise, declare it async

```
const [response1, response2, response3] = await Promise.all([
  fetch(url1), fetch(url2), fetch(url3)
]);

async function fetchWithTimeout(url: string, ms: number) {
  return Promise.race([fetch(url), timeout(ms)])
}
```

```
const getNumber = async () => 42;
// is equivalent to 
const getNumber = () => Promise.resolve(42);
```
