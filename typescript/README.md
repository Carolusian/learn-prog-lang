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

## Ch02: TypeScript's Type System

### Item 6: Editor and Language Services
  - autocomplete, inspection, navigation, and refactoring

### Item 7: Type as a set of values
  - this is the domain of the type
  - Itersection of types: 
    - `keyof (A&B) = (keyof A) | (keyof B)`
    - `keyof (A|B) = (keyof A) & (keyof B)`
  - `extends` as a constraint in a generic
    - `function sortBy<k extends keyof T, T>(vals: T[], key: K): T[] {...}`
  - `never`: empty set; `unknown`: universal set
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

## Ch03: Type Inference

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

### Item 26: Understand How Context Is Used in Type Inference

### Item 27: Use Functional Constructs and Libraries to Help Types Flow

- Lodash type alias. Dictionary<string> is the same as `{[key:string]: string}` or `Record<string, string>`
- `flat` method has signature like `T[][] => T[]`
- `_` wraps and `.value()` does the unwrap

```
const bestPaid = _(allPlayers)
  .groupBy(player => player.team)
  .mapValues(players => _.mapBy(players, p => p.salary)!)
  .values()
  .sortBy(p => -p.salary)
  .value()
```

## Ch04: Type Design

Show me your tables, and I won't usually need your flowcharts; they will be obvious --Fred Brooks, The Mythical Man Month

### Item 28: Prefer Types That Always Represent Valid States

tagged union trick

```
interface State {
  pageText: string;
  isLoading: boolean;
  error?: string
}
```

vs

```
interface RequestPending {
  state: 'pending';
}

interface RequestError {
  state: 'error';
  error: string;
}

interface RequestSuccess {
  state: 'ok';
  pageText: string;
}
type RequestState = RequestPending | RequestError | RequestSuccess;
interafce State {
  currentPage: string;
  requests: {[page:string]: RequestState};
}
```

Airbus 330 case

### Item 29: Be Liberal in What You Accept and Strict in What You Produce

- TCP implementation
- contract for functions: broad in what to accept, more specific in output
- `Omit` and `Partial`: see all utility types here: https://www.typescriptlang.org/docs/handbook/utility-types.html

### Item 30: Don't Report Type Information in Documentation

- `timeMs` and `temperatureC` is much cleaner 

### Item 31: Push Null Values to the Perimeter of Your Types

- Values are easier to work with when they're either completely null or null-null
- Turning on `strictNullChecks`

### Item 32: Prefer Unions of Interfaces to Interfaces of Unions

bad design: lead to mixed Layout and Paint

```
interface Layer {
  layout: FillLayout | LineLayout | PointLayout;
  paint: FillPaint | LinePaint | PointPaint;
}
```

- add "tag" to union, it is a good idea
- unions of interfaces are more precise

### Item 33: Prefer more Precise Alternatives to String Types

```
type RecordingType = 'studio' | 'live';
interface Album {
  artist: string;
  title: string;
  releaseDate: Date;
  recordingType: RecordingType;
}
```

- use `keyof T`

### Item 34: Prefer Incomplete Types to Inaccurate Types

- avoid uncanny valley of type safety
- acknowledge the gaps using any or unknown.

### Item 35: Generate Types from APIs and Specs, Not Data

- GraphQL types are nullable, ! after the type indicates not be null 
- many tools help go from GraphQL query to TypeScript types: `apollo client:codegen  --target typescript`
- single source of truth: the GraphQL schema
- If no spec or official schema, then have to generate from data: `quicktype`

### Item 36: Name Types Using the Language of Your Problem Domain

- Phil Karlton: two hard problems: cache invalidation and naming things
- try to reuse domain specialized vocabulary from the problem domain

### Item 37: Consider "Brands" for Nominal Typing

```
interface Vector2D {
  _brand: '2d';
  x: number;
  y: number;
}

function vec2D(x: number, y:number): Vector2D {
  return {x, y, _brand: '2d'}
}
```

- This makes it mathmatically correct
- See the AbsolutePath type guard example: `function isAbsolutePath(path: string): path is AbsolutePath`
- See the SortedList type guard example

## Ch05 Working with any

### Item 38: Use the narrowest possible scope for any

```
// bad: any return type is "contagious"
function f1() {
  const x:any = expressionFoo();
  processBar(x);
  return x;
}

// good: scoped to a single expression
function f2() {
  const x = expressionFoo();
  processBar(x as any);
  return x:
}

// bad
const config: Config = {
  a: 1,
  b: 2,
  c: {
    key: value
  }
} as any

// good
const config: Config = {
  a: 1,
  b: 2,
  c: {
    key: value as any
  }
} 
```

- can use `@ts-ignore` as well

### Item 39: Prefer More Precise Variants of any

- `any[]`
- `{[key: string]: any}`
- `type Fn0 = () => any;`
- `type Fn1 = (arg: any) => any;`
- `type FnN = (...args: any[]) => args.length;`

### Item 40: Hide Unsafe Type Assertions in Well-Typed Functions

```
function cacheLast<T extends Function>(fn: T): T {
  let lastArgs: anyp[] | null = null;
  let lastResult: any;

  return function(...args: any[]) {
    if (!lastArgs || !shallowEqual(lastArgs, args)) {
      lastResult = fn(...args);
      lastArgs = args;
    }
    return lastResult;
  } as unknown as T;
}
```

- `doubel assertion`: https://basarat.gitbook.io/typescript/type-system/type-assertion#double-assertion
- use `unknown` as a bridge to create a subtype relation
- harmless type assertion: `if(!(k in b) || aVal !== (b as any)[k])`
- this is much preferable to scattering iteration and assertions to check for object equality throughout your code

### Item 41: Understand Evolving any

```
const result = [];
result.push('a');
result // type is string[]
result.push(1);
result // type is (string | number)[]
```

- implicit `any` and `any[]` types are allowed to evolve

### Item 42: Use `unknown` Instead of `any` for Values with an Unknown Type

- `any` doesn't fit "thinking of types as sets of values": both a subset and superset of all other sets
- `unknown`: any type is assignable to `unknown`; unknown is only assignable to unknown, not other types
- `never`: on the opposite

```
const book = safeParseYAML(`
  name: Villette
  author: Charlotte Bronte
`) as Book;
```

- `unknown` is not assignable to other types, a type assertion is explicitly required (better than any).
- `unknown`: unknown is appropriate whenever you know that there will be a value but you don't know its type
- `object` and `{}` can be used in a similar way to `unknown`
- `unknown` is a type-safe alternative to any, use `unknown` to force your users to use a type assertion or do type checking

### Item 43: Prefer Type-Safe Approaches to Monkey Patching

- `augmentation`: special ability of interface (you can't hide it from other parts of your code or from libraries)

```
// in a module context, you'll need to add a declare global to make this work

export {};
declare global {
  interface Document {
    monkey: string
  }
}
```

- `type assertion`

```
interface MonkeyDocument extends Document {
  monkey: string
}

(document as MonkeyDocument).monkey = 'Macaque'
```

### Item 44: Track Your Type Coverage to Prevent Regressions in Type Safety

- `npx type-coverage`: keep track of how well-typed your program is.

## Ch06 Types Declaration and `@types`

### Item 45: Put TypeScript and @types in devDependencies

- DefinitelyTyped

### Item 46: Understand the Three Versions Involved in Type Declarations 

- version of package
- version of its type declarations (@types)
- version ot TypeScript
- Major and minor versions match but that patch versions do not.
- Official recommendation: bundle type declarations only if the library is written in TypeScript

### Item 47: Export All Types that Appear in Public APIs 

- for exported types, can use `Parameters` and `ReturnType` generic types

```
type MySanta = ReturnType<typeof getGift>;
type MyName = Parameters<typeof getGift>[0];
```

### Item 48: Use TSDoc for API Comments

- `@param` and `@returns`

### Item 49: Provide a Type for this in Callbacks

- there is no reason `this` had to be bound to an instance
- it could be bound to anything
- replace `onClick` with an arrow function 
- Don't forget about `this`, it is a part of your API, and you should include it in your type declarations

```
function addKeyListener(
  el: HTMLElement,
  fn: (this: HTMLElement, e: KeyboardEvent) => void
) {
  el.addEventListener('keydown', e => {
    fn.call(el, e);
  });
}
```

### Item 50: Prefer Conditional Types to Overloaded Declarations

The problem:

```
function double(x: number|string): number|string;

// the return type is imprecise
double(12) // string | number
double('x') // string |number

// better way
function double<T extends number|string>(x:T): T extends string? string: number
```

- Conditional types are like if statements in type space

### Item 51: Mirror Types to Sever Dependencies

```
// leverage structural typing
// just mirror the methods you need

interface CsvBuffer {
  toString(encoding?: string): string
}
```

### Item 52: Be Aware of the Pitfall of Testing Types

```
function asserType<T>(x: T) {}

assertType<number[]>(map(['john', 'pual'], name => name.length))
```

- `dtslint`

## Ch07 Writing and Running Your Code

### Item 53: Prefer ECMAScript Features to TypeScript Features

- alternatives to Enums: `type Flavor = 'vanilla' | 'chocolate' | 'strawberry'`
- has advantage of translating more directly to JavaScript
- Try to avoid hiding the design of your class by using a mix of parameter and non-parameter properties (inconsistence)
- Decorators

### Item 54: Know How to Iterate Over Objects

- `for (k in keyof ABC)`
- `for (const [k, v] of Object.entries(abc))`

### Item 55: Understand the DOM hierarchy

- normally you can ignore when writing JavaScript, but need to Undertand in TypeScript
- differences between `Node`, `Element`, `HTMLElement`, and `EventTarget`; `Event` and `MouseEvent`
- Use specific enough type for DOM, or give TypeScript context to infer

### Item 56: Don't Reply on Private to Hide Information

### Item 57: Use Source Maps to Debug Typescript

- sourceMap option in tsconfig.json

## Ch08 Migrating to TypeScript

### Item 58: Write Modern JavaScript 

- use Classes Instead of Prototypes
- use let/const (block scoped) instead of var 
- use for-of loop; use `array.forEach`
- prefer arrow functions to make `this` less confused
- use Compact Object Literals and Destructing Assignment
- shorthand for properties whose values are functions
- specify default values when destructuring: `const {a = 'defualt'} = obj.props`
- use default parameters
- use async/await instead of Raw Promises or Callbacks
- use Map and Set instead of objects for Associative Arrays

### Item 59: Use @ts-check and JSDoc to experiment with TypeScript

### Item 60: Use allowJs to Mix TypeScript and JavaScript

### Item 61: Convert Module by Module Up Your Dependency Graph

- starting with leaves
- `madge` tool
- As you migrate your code, focus on adding types rather than refactoring
- make a list of future refactors

### Item 62: Don't Consider Migration Complete Until You Enable `noImplicitAny`
