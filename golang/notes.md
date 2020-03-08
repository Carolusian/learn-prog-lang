# Golang study notes

- `...` unpacks to positional arguments
- Like `for`, `if` statement can start with a short statement to execute before condition
- An interface contains a set of method signatures
- A two-value assignment in Map tests the existence of a key

## Pointers

- Need to distingush `pointer variable` and `variable`
- `*ptr` has different meanings when been declared and been called
  - When declared, it indicates `ptr` is a pointer variable
  - When called, it indicates it is dereferences, so it returns the value rather than memory address
- In structs, `(*ptr).X` and `ptr.X` are the same, no explicit deferences is necessary

## Package management

- `go mod init charliechen.com/packagename`
- `go build -o main main.go`
  - this will find, download and extract dependencies automatically
  - `go.mod` and `go.sum` will be updated automatically

see more in: https://github.com/golang/go/wiki/Modules

## Defer, Panic, and Recover

- `defer` executes the deferred function to the end after the execution of surrounding functions
- `panic` is like throw in other languages, but `panic` can be recovered

## Misc

### Tags

You can add extra meta information to Go structs in the form of tags. [Here are some examples of use cases](https://stackoverflow.com/questions/10858787/what-are-the-uses-for-tags-in-go).

In this case, the json:"gateway" is used by the json package to encode the value of Gateway into the key gateway in the corresponding json object:

```
type NetworkInterface struct {
    Gateway              string `json:"gateway"`
    IPAddress            string `json:"ip"`
    IPPrefixLen          int    `json:"ip_prefix_len"`
    MacAddress           string `json:"mac"`
    ...
}
```

### Makefile

Makefile is good for golang project automation. `.PHONY` tells makefiles that the target is not associate with files.

### Folder and file naming

- `package-name` or outside folders
- `package_name` inside project, e.g. `rate_limiter/rate_limiter.go` in `godropbox` source code
- File name samples:
  - connection_tracker.go
  - load_balanced_pool.go
  - simple_pool.go

### Learn X in Y min

- https://learnxinyminutes.com/docs/go/
