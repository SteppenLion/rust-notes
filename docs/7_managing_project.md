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
- **Declaring modules**: In the crate root file, you can declare new modules; say, you declare a "garden" module with `mod garden;`. The compiler will look for the module's code in these places:
  - Inline, within curly brackets that replace the semicolon following `mod garden`
  - In the file _src/garden.rs_
  - In the file _src/garden/mod.rs_
- **Declaring submodules**: In any file other than the crate root, you can declare submodules. For example, you might declare `mod vegetables;` in _src/garden.rs_. The compiler will look for the submodule's code within the directory named for the parent module in these places:
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

## Grouping Related Code in Modules

- _Modules_ let us organize code but also allow us to control the _privacy_ of items -> bacause the code within module is private by default.

## Paths for Reffering to an Item in the Module Tree

- a path can take two forms:
  - An _absolute path_ - full path starting from a crate root, the absolute path begins with the crate name, and for code fom the current crate, it starts with literal `crate`
  - A _relative path_ starts from the current module and uses `self`, `super` or an identifier in the current module.
- both absolute and relative paths are follwed by one or more identifiers separated by double colons (`::`)

```rust
mod front_of_hosue {
  mod hosting {
    fn add_to_waitinglist { }
  }
}

pub fn eat_at_restaurant() {
  // Absolute path
  crate::front_of_hosue::hosting::add_to_waitinglist();
  // Relative path
  front_of_hosue::hosting::add_to_waitinglist();
}
```

- items in parent module can't use the private items inside child modules, but items in child modules can use the items in their ancestor modules. This is because child modules wrap and hide their implementation details, but the child modules can see the context in which they're defined.
- if we want to expose inner parts of child modules code to outer ancestor modules by using `pub` keyword to make them public.
- The privacy rules apply to structs, enums, functions, and methods as well as modules.

## Exposing Paths with the `pub` Keyword

```rust
// to be able to access in above code the add_to_waitinglist() funtion we need to make mod and fn public
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();
    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
```

## Starting Relative Paths with `super`

- if we use `pub` before struct definition we make he struct public but the struct's fields will still be private.
- we can make each field public or not on a case-by-case basis.
- see example where some fields are public ( whcih bread customer can choose, private shef choose which fruit is acommpanies the meal)

```rust
mod back_of_house() {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast:&str) -> Breakfast {
            Breakfast {
              toast: String::from(toast),
              seasonal_fruit:String::from("peaches");
            }
        }
    }
}

pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
}
```

- enum pub example:

```rust
mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
```

- enums aren't very useful unless their variants are public. Structs are often useful without their fields being public.

## Bringing Paths into Scope with the `use` Keyword

- `use` keyword we can use to simplify the import of the functions. We only need to use the `use` once and shorter the name everywhere else in the scope.

```rust
mod front_of_hosue {
  pub mod hosting {
    pub fn add_to_waitinglist() {}
  }
}
use crate::front_of_hosue::hosting; // we get warning

pub fn eat_at_restaurant() {
  // use crate::front_of_hosue::hosting; // we don't get warning
  hosting::add_to_waitinglist();
}
```

- adding `use` and a path in a scope is similar to crating a symbolic link in the filesystem.
- Note that `use` only creates the shortcut for particular scope in which the `use` occurs.
- within the module example:

```rust
mod front_of_hosue {
  pub mod hosting {
    pub fn add_to_waitinglist() {}
  }
}
// use crate::front_of_hosue::hosting; // we get warning

mod customer {
    use crate::front_of_hosue::hosting; // we don't get warning
    pub fn eat_at_restaurant() {
        hosting::add_to_waitinglist();
    }
}
```

- we get warning in above code that `use` is no longer used in its scope -> to fix this problem, move the `use` within the `customer` module

## Creating Idiomatic `use` Paths

- it's idiomatic to specify a full path with `use` when we bring in structs,enums and other items.
- for example when we bring from standard library's `HashMap` struct into the scope of a binary crate.

```rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}
```

- there no reason behind this idiom, just convention.

## Providing New Names with the `as` Keyword

- we can bring two types of the same name into the same scope with `use` after the path we can specify `as` as new local name or _alias_ for the type.

```rust
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
    Ok(())
}

fn function2() -> IoResult<()> {
    // --snip--
    Ok(())
}
```

## Re-exporting Names with `pub use`

- when we bring a name into scope with the `use` keyword -> the name availablw in the new scope is private.
- to enable the code that calls our code to refer to that name as if it had been defined in the code's scope we can combine `pub` and `use` -> _re-exporting_.

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}
pub use crate::front_of_house::hosting;
pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

## Using External Packages

- define in the Cargo.toml, as for example external crate `rand`.

## Using Nested Paths to Clean Up Large `use` Lists

- we can shortend the use statements to be nested: see example below

```rust
/*
FROM this import
*/
use rand::Rng;
use std::cmp::Ordering;
use std::io;
// To this import
use rand::Rng;
use std::{cmp::Ordering, io};
// or
//use std::io;
// use std::io::Write; -> use std::io::{self, Write};
```

## The Glob Operator

- all public items defined in a path into a scope -> we can use the glob operator (`*`)

```rust
#![allow(unused)]
fn main() {
use std::collections::*;
}
```

- Be careful when using the glob operator! Glob can make it harder to tell what names are in scope and where a name used in your program was defined.
- The glob operator is often used when testing to bring everything under test into the tests module.

## Separating Modules into Different Files

- we'll extract the `front_of_house` module to its own file.
- Filename: src/lib.rs

```rust
mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

- The compiler knows to look in this file because it came across the module declaration in the crate root with the name front_of_house.
- Filename: src/front_of_house.rs

```rust
pub mod hosting {
    pub fn add_to_waitlist() {}
}
```
