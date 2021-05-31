## Ch 1: Python Data Model

- Dunder methods \_\_\*\_\_
  - Pros?
    - e.g. you can use `random.choice()` with object instance of a class having **len** and **getitem** methods, or you forloop the instance
    - e.g. operator overloading
    - e.g. unified interface for standard methods of different data models/ data structures
  - Python Data Model: object, value type
  - Object Model: Metaobject protocal - API for core language construct (extending a language to support new paradigms)
- Simple class with collections.namedtuple

## Ch 2: Data Structure - sequence

- Container / Sequence / MutableSequence
- Forloop vs List comprehension
  - Forloop: scan or count or aggregate or ...
  - List Comprehension: build a new list
  - Generator expression: save expense
- Tuples: immutable and unnamed records
  - Tuple unpacking
    - \_ as dummy variable placeholder
    - \* to grab excess items
- Slicing:
  - Multiple dimensional slicing and ellipsis (...)
  - `setitem`, `getitem`, `setslice`, `getslice`: dunder or magic methods for duck typing
- Augmentted Assignment: Using + and \* with sequences
  - \* repeats the same sequence
  - build list with list, e.g. `[['_'] * 3] * 3`
- `bisect` not only for seaching but also for inserting items in sorted sequence
- `id`: get object id
- `bis.bis("s[a] += b")`: this can generate byte code of the code in the string

```
def grade(score, breakpoints=[], grades='FDCBA'):
   i = bisect.bisect(breakpoints, grades)
   retrun grades[i]

[grade(score) for score in [33, 99, 77, 70, 89]]
```

- `bisect.insort` insert and keep the order of the sequence
- List is not always the answer:
  - `array` for `floats`
  - `deque` for FIFO and LIFO

## Ch 3: Data Structure - dictionaries and sets (hashtables)

- `__builtins__.__dict__`
- `hashable`: value never change during its lifetime
- `dict` vs `defaultdict` vs `OrderedDict`
- `defaultdict`: `__missing__`
- `collections.ChainMap`, `collections.Counter`
- `UserDict`: desgined to be subclassed
- `MappingProxyType`: immutable dict/mappings
- semantic set operators
- practical consequences of hashtable:
  - elements must be hashable objects
  - have signifcant memory overhead: sparse array and dynamic space expension
  - efficient membership test
  - ordering depends on insertion
  - adding elements may change the order of other elements

## Ch4: Text vs bytes

- literal: 字面常量 / code point: 码位 (the representation that can be understand by humans)
- code point > encoded using, e.g., UTF8, UTF16 > different bytes (the representation of the data in computer) depends on the selected encoding
- bytes can be decoded to code points
- bytes (immutable) vs bytearray(mutable)
- struct can format bytes in a memoryview
- package: `chardet` - detect character encoding heuristically
- BOM(byte-order mark): little-endian and big-endian, `\xff\xfe`
- The unicode sandwich
- local.getpreferedencoding

## Ch5: First-class functions

- first-class function means: functions as first-class objects: assigned to variable/ element in data structure; pass as argument; return in the function
- higher-order function: take function as argument or return function
- callable objects: `__call__`

## Ch6: Design pattern with first-class functions

- Strategy pattern: order discount strategies: Order as context, Abstract and Contrete strategies
- `abc` for abstract class and methods
- If the instance does not maintenance state, then it would be safe to replace with plain function object
- generator expression: `max(promo(order) for promo in promos)`
- `globals()`: to get a list of promotions from current global symbol table.
- you can use `inspect.getmembers` to do the same thing from a module
- Command pattern: object-oriented replacement for callbacks
- If generic sound name like `execute`, `run`, `doIt`. Then consider using first-class functions
- Other books introducing patterns

## Ch7: Function decorators and closures

- metaprogramming - changing program behavior at runtime
- decorator runs when the it is imported: import time
- One use case of decorator is to add functions to some central registry: web frameworks routing, optimize strategy pattern
- Codes that uses inner functions always depends on closures to operator correctly
- `dis` module: easy way to disassemble the bytecode of python functions
- Closures: average higher-order function example; free variables
- `nonlocal` declaration

## Monkey patch

- pipes: https://github.com/andybrice/pypework/blob/master/pypework/__init__.py

## Python AST

- pipes with AST: https://github.com/robinhilliard/pipes/blob/master/pipeop/__init__.py

## Python type hinting

- How to user python type hinting: https://www.youtube.com/watch?v=yScuF1UgGU0
