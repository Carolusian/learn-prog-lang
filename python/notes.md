## Ch01: Python Data Model

- Dunder methods \_\_\*\_\_
  - Pros?
    - e.g. you can use `random.choice()` with object instance of a class having **len** and **getitem** methods, or you forloop the instance
    - e.g. operator overloading
    - e.g. unified interface for standard methods of different data models/ data structures
  - Python Data Model: object, value type
  - Object Model: Metaobject protocal - API for core language construct (extending a language to support new paradigms)
- Simple class with collections.namedtuple

## Ch02: Data Structure - sequence

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

## Ch03: Data Structure - dictionaries and sets (hashtables)

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

## Ch04: Text vs bytes

- literal: 字面常量 / code point: 码位 (the representation that can be understand by humans)
- code point > encoded using, e.g., UTF8, UTF16 > different bytes (the representation of the data in computer) depends on the selected encoding
- bytes can be decoded to code points
- bytes (immutable) vs bytearray(mutable)
- struct can format bytes in a memoryview
- package: `chardet` - detect character encoding heuristically
- BOM(byte-order mark): little-endian and big-endian, `\xff\xfe`
- The unicode sandwich
- local.getpreferedencoding

## Ch05: First-class functions

- first-class function means: functions as first-class objects: assigned to variable/ element in data structure; pass as argument; return in the function
- higher-order function: take function as argument or return function
- callable objects: `__call__`

## Ch06: Design pattern with first-class functions

- Strategy pattern: order discount strategies: Order as context, Abstract and Contrete strategies
- `abc` for abstract class and methods
- If the instance does not maintenance state, then it would be safe to replace with plain function object
- generator expression: `max(promo(order) for promo in promos)`
- `globals()`: to get a list of promotions from current global symbol table.
- you can use `inspect.getmembers` to do the same thing from a module
- Command pattern: object-oriented replacement for callbacks
- If generic sound name like `execute`, `run`, `doIt`. Then consider using first-class functions
- Other books introducing patterns

## Ch07: Function decorators and closures

- metaprogramming - changing program behavior at runtime
- decorator runs when the it is imported: import time
- One use case of decorator is to add functions to some central registry: web frameworks routing, optimize strategy pattern
- Codes that uses inner functions always depends on closures to operator correctly
- `dis` module: easy way to disassemble the bytecode of python functions
- Closures: average higher-order function example; free variables
- `nonlocal` declaration: flag a variable as `free variable`
- Decorator: `functools.wraps` - handles keyword arguments and copy `__name__` and `__doc__`
- `functools.lru_cache`: least recently used; must be invoked as regular function
- `singledispatch`: can be used to do function or method overloading - singledispatch and then `decorator.register`
- Parametrized decorators: a decorator factory takes those arguments and returns a decorator
- A decorator can has 3 `def`: 1. decorator parameters; 2. the decorated function; 3. the parameters for decorated function
- `**locals()`: allows any local variables to be passed as keyword arguments
- decorators are best coded as classes implementing `__call__`

## Ch08: Object references, mutability and recycling

- Python object is a label rather than a box
- The only mode of parameters passing in Python is `call by sharing`
- `list()`: can create copy of objects
- `==` vs `is` makes more sense than Java: equality vs aliases
- Every python has an identity: a type and a value
- augmented assignment with `+=`, `*=` creates new objects if left-hand variable is bound to an immutable object
- Assigning a value doesn't change the object, it is just `rebinding` to a different object (label to a different object)
- using mutable objects as default values for function parameters is dangerous
- In python, the algorithm for garbage collection is reference counting
- `__del__`: doesn't delete the real object, it just remove the label
- `__del__`: give the instance a chance to release external resources before deleted
- `weakref` is useful in implementing caching
- `weak references`: `list` and `dict` instances may not be referents
- `interning`: a technic for optimization by sharing literals or small integers

## Ch09: A Pythonic object

- `repr`, `str`, `format` must always return unicode strings, only `__bytes__` supposed to return a bytes sequence
- `__iter__` makes a object iterable, and also makes `unpacking` work
- `{!r}` in interpolating get `repr`
- build tuples help comparison
- `classmethod` vs `staticmethod`: see alternative constructor, static method is like a normal function
- `__hash__`: normally implemented with `xor` of the hash of the components
- used `property` and private `__` to make attributes readonly
- private attribute names are mangled by prefixing the \_ and class name
- `_` as protected attribute, it is by convention, nothing special
- use `__slots__` class attribute to save memory space
- `__slots__` have caveats, only use when absolutely need to save memory
- using `type`, make the methods safer to be inherited
- `__index__` to use a object as slice

## Ch10: Sequence hacking, hashing and slicing

- This chapter is mainly about `duck typing`: duck typing serves the purpose of protocal without inherit anything
- `duck typing` is actually providing informal protocol
- `cosine similarity`: the cosine of the angle between a query vector and a document vector: As the angle decrease the cosine approaches the maximum value of 1.
- `cosine similarity` is useful in information retrieval as documents can be represented as vectors
- How to implement a FrenchDeck using and duck typing and OOP
- How slicing works: need to determine whether the index parameter is a integer or a slice
- A slice object has 3 data attributes: `start`, `stop` and `step`
- `__getattr__` enables dynamic access to attributes, also need to create a customer `__setattr__` method for the purpose of consistency
- `reduce`: useful in computing the hash of all vector components
- Makes the `==` operator faster: `len(self) == len(other) and all(a == b for a, b in zip(self, other))`
- `zip_longest`: works for vectors with different length with a `fillvalue` parameter
- `__format__`: according to the trailing character of the format parameter to return different formatted string
- `xxx-like` object, e.g. file-like object, can be a vivid explanation of `duck typing`
- If you wnat the sum of list of items, you should write in a way that looks like "the sum of a list of items", not in a way that looks like "loop over these items, maintain another variable t, perform a sequence of additions"

## Ch11: Interfaces: from protocol to ABCs

- This chapters compares: `duck typing` and formal interfaces `Abstract Base Classes (ABC)`
- `ABC`, like descriptors and metaclasses, is mainly for building frameworks
- Informal protocols are only defined in documentation and convention, cannot be enforced
- Monkey patching: `def set_card(deck, position, card)`, then `FrenchDeck.__setitem__ = set_card`
- Monkey patching has a bad reputation, and may step on each other's toes
- `goose typing`: use `isinstance` to check if `cls` is an Abstract Base Class, metaclass
- ABCs are meant to encapsulate very general concepts, abstractions, introduced by a framework
- Concrete subclass can override methods inherited from ABC with more efficient implementations
- Most ABCs are defined in `collections.abc` module and `numbers` module
- The best way to declare an ABC is to subclass `abc.ABC`
- `abstractmethod` can be combined with `classmethod` and `staticmethod`
- `virtual subclass`: `ABC.register` even if it does not really inherit from the ABC
- `__subclasshook__`
- Inheritance is guided by `__mro__`: method resolution order
- Geese can behave as ducks
- `zope.interface`
- `Go` is `static duck typing`
- If type-checking is performed at compile time, the language is statically typed
- A metaphor fosters understanding by making constraints clear
- However, strict adherence to metaphors ties interfaces unnecessarily tightly to the workings of the physical world

## Ch12: Inheritance: for good or for worse

- Subclassing built-in types: you shall not overrides built-in methods, subscribe `UserDict`, `UserList` and `UserString` instead
- Multiple inheritance and method resolution: read from right to left, tips: see `__mro__`
- You can also call a method on a superclass directly, passing the instance as an explicit argument: `C.pong(self)`
- Coping multiple inheritance:
  - Distinguish `interface` and `implementation`: create sub-type (is-a relationship) vs. to facilitate code reuse
  - If a class defines interface, it should be an explicit ABC
  - Use mixins for code reuse
  - Make mixins explicit by naming, e.g. `XViewMixin`
  - An ABC may also be a mixin
  - Don't subclass from more than one concrete class
  - Provide aggregate classes to uses: e.g. Django `ListView`
  - Favor object composition over class inheritance
- Django class based generic views: View and TemplateResponseMixin

## Ch13: Operator overloading: doing it right

- Java left out operator overloading because too many people abuse it
- Good balance in python: cannot overload operators for built-in types; only able to overload existing operators
- Cannot overload `is`, `and`, `or`, `not`
- `x` and `+x` are not necessarily equal, e.g. different `decimal.getcontext` and counter
- Operator overloading shall not change their operands
- To support operations for different type, example:
  - `__add__` ? -> `NotImplemented`
  - `NotImplemented` -> `__radd__`? -> `NotImplemented`? -> `TypeError`
- `__radd__` is reversed special methods
- `__radd__` can be just delegated: ``return self + other`
- In the spirit of duct typing, we can refrain from testing the type of the other operand
- Augmented assignment: in-place operators overloading: `__ixxx__`
- Augmented assgiment special methods must return `self`
- `@` infix operation in python 3.5, for matrix multiplications
- Rich comparison
- Overloaded operators, when used sensibly, do make code easier to read and write
- In Python, you write this: `interest = principal * ((1 + rate) ** periods - 1)`
- In Java, you write this:

```
BigDecimal interest = principal.multiply(BigDecimal.ONE.add(rate).pow(periods).subtract(BigDecimal.ONE));
```

## Ch14: Iterables, iterators and generators

- `yield` keyword allows the construction of generators. Generators (implement iterator interface) are iterators
- generators can produce items "out of thin air", while iterators retrieve items from a collection
- Iterators are used internally to support:
  - for loops; looping over text files; list, dict and set comprehension;
  - collection types construction and extensions;
  - tuple unpacking; unpacking \* parameters.
- `__iter__` makes an sequence `iterable`, if `__iter__` is not implemented, fallback to `__getitem__` to fetch item in order
- `abc.Iterable` implements the `__subclasshook__`
- `iter(x)` and TypeError is better than `isinstance(x, abc.Iterable)`
- iterator vs iterable: Python obtains iterators from iterables
- A concrete `iterator` must implement `__next__`, and implements iterable by returning `self`
- `yield` must maintains state in the iterator object, more like a syntax sugar
- iterable produce new iterator, whereas iterator produce itself
- a generator function is a generator factory
- generators are iterators that produce the values of the expressions passed to yield
- when a generator function returns, the enclosing generator object raises `StopIteration`
- lazy implementation: `re.finditer` instead of `re.findall`
- generator expression: like list comprehension, factor of generators, e.g. generator expression in `__iter__`
- if the generator expression spans more than a couple of line, then use generator function instead
- generators in standard library: `itertools.compress`, `dropwhile`, `filter`, `filterfalse`, `islice`, `takewhile`
- mapping generator functions: `accumulate`, `enumerate`, `map`, `starmap`
- merge multiple input: `chan`, `chain.from_iterable`, `product`, `zip`, `zip_longest`
- expand input item: `combinations`, `count`, `cycle`, `permutations`, `repeat`
- rearranging: `groupby`, `reversed`, `tee`
- `yield from`: replace the inner for loop, like yield from one of multiple inner iterables
- return single values: `all`, `any`, `max`, `min`, `reduce`, `sum`.
- generators as coroutines, basically two concepts, merely syntax sugar, the creator don't like additional keywords:
  - generators produce data for iteration
  - coroutines are consumers of data
  - coroutines are not related to iteration

## Monkey patch

- pipes: https://github.com/andybrice/pypework/blob/master/pypework/__init__.py

## Python AST

- pipes with AST: https://github.com/robinhilliard/pipes/blob/master/pipeop/__init__.py

## Python type hinting

- How to user python type hinting: https://www.youtube.com/watch?v=yScuF1UgGU0
