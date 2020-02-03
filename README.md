## Restaurant


This is not a code project really since no functionality is implemented.

This project serves to serve as an illustration of the various ways of managing a growing project in rust using the following

- Modules
- Files
- Packages
- Crates



## Notes
Each project is implicitly wrapped in a root module known as crate from which we can build relative and absolute paths to nested modules and
functions

We can bring modules in scope by using the `use module_name` syntax this brings the modules into scope in such a way they can be used.

We do the same for Structs and Enums with the differences being that, When an Enum is brought into scope all it's variants are
exposed as opposed to when a struct is brought into scope and we have to expose it's methods and fields

We can re expose functionality we bring into scope with the `pub use module_name` syntax

At whichever level of nestign we can access our ancestors scope using the `super::name` syntax


