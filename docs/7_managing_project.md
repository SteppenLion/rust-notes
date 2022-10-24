# Managing Growing Projects with Packages, Crates, and Modules

## Packages and Crates

- a _crate_ is the smallest amount of the code that the Rust compiler considers at a time.
- crates can obtain modules and the modules may be defined in other files that get compiled with the crate
- crate can come in two forms : _a binary crate_ and _a library crate_
- binary crates are programs you can compile to an exucutable that you can run such as a commnad-line program or servers. Each must have function `main`.
- library crates don't have `main` function and they don't compile to executable. Instead they define a functionality intended to be shared with multiple projects. For example `rand` library.
- _crate root_ is source file that the Rust compiler starts from and makes up the root module of your crate.
- a _package_ is a bundle of one or more crates that provides a set of functionality. A package containd Cargo.toml file that describes how to build those crates. A package can contain as many binary crate as we like bu at least one.

## Defining Modules to Control Scope and Privacy

- **Start from the crate root**: When compiling a crate, the compiler first looks in the crate root file (usually src/lib.rs for a library crate or src/main.rs for a binary crate) for code to compile.
- **Declaring modules**: In the crate root file, you can declare new modules; say, you declare a “garden” module with `mod garden;`. The compiler will look for the module’s code in these places:
  - Inline, within curly brackets that replace the semicolon following `mod garden`
  - In the file _src/garden.rs_
  - In the file _src/garden/mod.rs_
- **Declaring submodules**: In any file other than the crate root, you can declare submodules. For example, you might declare `mod vegetables;` in _src/garden.rs_. The compiler will look for the submodule’s code within the directory named for the parent module in these places:
  - Inline, directly following `mod vegetables`, within curly brackets instead of the semicolon
  - In the file _src/garden/vegetables.rs_
  - In the file _src/garden/vegetables/mod.rs_
- **Paths to code in modules**: Once a module is part of your crate, you can refer to code in that module from anywhere else in that same crate, as long as the privacy rules allow, using the path to the code. For example, an `Asparagus` type in the garden vegetables module would be found at `crate::garden::vegetables::Asparagus`.
- **Private vs public**: Code within a module is private from its parent modules by default. To make a module public, declare it with `pub mod` instead of `mod`. To make items within a public module public as well, use pub before their declarations.
- **The `use` keyword**: Within a scope, the `use` keyword creates shortcuts to items to reduce repetition of long paths. In any scope that can refer to `crate::garden::vegetables::Asparagus`, you can create a shortcut with `use` `crate::garden::vegetables::Asparagus;` and from then on you only need to write Asparagus to make `use` of that type in the scope.

- here is create a binary crate named `backyard` that ilustrated these rules:

```
backyard
|--Cargo.lock
|-- Cargo.toml
|-- src
    |--garden
    |  |--vegetables.rs
    |-- garden.rs
    |-- main.rs
```
