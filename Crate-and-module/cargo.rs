// Cargo:
// Official package manager for Rust
// Helps automate tasks such as creating a new project, building, running, testing,managing dependencies and publishing the
// project
// Crate is a compilation unit of Rust code
// crates.io is the official Rust package registry
//
// Crate:
// crate can either be a binary or a library
// Binary crate:
// Compiled into an executable binary
// Basicallly, it is a program that can be run
//
// Library crate:
// Compiled into a library
// It is a collection of functions and types that can be used in other programs
//
// Crate root:
// The source file that the Rust compiler starts from and makes up the root module of your crate
// In binaries: src/main.rs
// In libraries: src/lib.rs
//
// Modules:
// way of organizing code by grouping related items together
// Can be imported using namespaces avoiding naming collisions
// Also control privacy of its items like functions, structs, enums, etc.
// When compiling the compiler starts fromt the crate root , then checks if modules are declared
// and looks for submodules in the directory structure.
// Submodules are declared using the mod keyword.
// Submodules could be directly writtern inline within curly braces , in a file which has the
// module name ending with .rs or in directory which has teh name of the module and a mod.rs file
// inside it
