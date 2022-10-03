## Pointers

- `*`:
  - declaring (as variable or function parameters) as pointer (address)
  - dereference/consuming (use or assign at right side of = ) as value (memory store the value)
  - a variable pointing to address of another variable, e.g. pointer allocated in stack, pointing to memory address in heap
  - pass by pointer
- `&`:
  - No declaration
  - consuming as the address of a variable
  - `&variable_with_value`
  - a reference has the same memory address
  - pass by reference: only in c++ (similar to pass by pointer)
- `*` is closing related to array
- `int *addressOfDigit = &digit;`
- see: https://www.geeksforgeeks.org/passing-by-pointer-vs-passing-by-reference-in-c/

## Package Manager: vcpkg

### basic steps

- `cd ~/ && git clone https://github.com/microsoft/vcpkg`
- `~/vcpkg/bootstrap-vcpkg.sh`
- `sudo ~/vcpkg/vcpkg integrate install`: User-wide integration for VS or MSBuild
  - mkdir if folder does not exist
- create `CMakeLists.txt` and `~/vcpkg/vcpkg install` relevant packages
- create `src/main.cpp` file
- make `build` folder
- configure CMake: `cmake -B build -S . -DCMAKE_TOOLCHAIN_FILE={THE_PATH}`
- build the project: `cmake --build build/`

### Manifest mode: vcpkg.json

- all packages are managed for us

### With VSCode

- Shift + Cmd + P: open workspace settings (JSON)
- add `cmake.configureSettings`
- If vscode hints that any dependency package is missing: then need to delete `build/*` and reconfigure
- recommended extensions: https://www.40tude.fr/how-to-use-vcpkg-with-vscode-and-cmake/
  - `C/C++`, `CMake` and `CMake Tools`

## References:

- [Good explain to Pointers](https://www.freecodecamp.org/news/pointers-in-c-are-not-as-difficult-as-you-think/#1-what-exactly-are-pointers)
- [TCP Client Server Application | C++ in 2021](https://www.youtube.com/watch?v=DVMHEDhYEr4)
  - How to setup one workspace with multiple library projects
  - `.` vs `->`: dot applied to actual object, arrow used with a pointer to the object
  - `strongThis`: closure in lambda
  - list initialization
  - `new` needs `delete`
  - `using` alias: typedef
  - `shared_ptr`
