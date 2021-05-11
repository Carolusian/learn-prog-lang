# Book: Stylish F

## Ch2: Designing Functions

- Discriminated Unions (duplicated naming): https://fsharpforfunandprofit.com/posts/discriminated-unions/
- Label == Tag == Case Identifier: must start with an Upper case letter
- Giving the name “T” to the main business type in the module if the Type Name is the same with the Module name
- Double naming maybe confusing, so be aware of it in F# 4.0+

## Ch3: handling missing data

- Option usage in F#:
  - Option.iter: like scala for comprehension
  - Option.map: extract > apply function > return a option
  - Option.bind: TBC
