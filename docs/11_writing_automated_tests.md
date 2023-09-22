# Writing Automated Tests

-In the essay: The Humble Programmer, Edsger W. Dijkstra said that:

> Program testing can be a very effective way to show the presence of bugs, but it is hopelessly inadequate for showing their absence.

## The Anatomy of a Test Function

- simplest a test in Rust is a function that's annotated with the `test` attribute
- attributes are metadata about pieces of Rust code, one example is the `derive` attribute. To change function into test function we need to add `#[test]` on the line above `fn`
- when we run the program with `cargo test` command, Rust builds a test runner binary that runs the annotated functions and reports on whether each test function passes or fails
- we can create new project library by:

```sh
cargo new adder --lib
```

- it will create test in the src/lib.rs file

```rust
#[cfg(test)]
mod test {
  #[test]
  fn it_works() {
    let result = 2 + 2;
    assert_eq!(result, 4);
  }
}
```

### Checking Result with the `assert!` Macro

- the `assert!` macro provides by the standard library, it is useful when you want to ensure that some condition in a test evaluates to `true`

```rust
#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

#[allow(dead_code)]
impl Rectangle {
  fn can_hold(&self, other: &Rectangle) -> bool {
    self.width > other.width && self.height > other.height
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn larger_can_hold_smaller() {
    let larger = Rectangle {
      width: 8,
      height: 7,
    };
    let smaller = Rectangle {
      width: 5,
      height: 1,
    };
    assert!(larger.can_hold(&smaller));
  }
}
```

- Note: we've added a new line inside the `tests` module: `use super::*;`
- Because the `tests` module is an inner module we need to bring the code under test in the outer module into the scope of the inner module, We use the glob here so anything we define in the outer module is available to this `tests` module.

## Testing Equality with the `assert_eq!` and `assert_ne!` Macros

```rust
pub fn add_two(a: i32) -> i32 {
  a + 2
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_adds_two() {
    assert_eq!(4, add_two(2));
  }
}
```

- as the name of these macros suggest `assert_eq!` and `assert_ne!` are opposites and under surface they use `==` and `=!` operators.

## Adding Custom Failure Messages

- by adding optional arguments into `assert!`, `assert_eq!`, and `assert_ne!` macros.

```rust
   #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }
```

## Checking for Panics with `should_panic`

- In addition to checking return values, it's important to check that our code handles error conditions as we expect.

```rust
pub struct Guess {
  value: i32,
}

impl Guess {
  pub fn new(value: i32) -> Guess {
    if value < 1 || value > 100 {
      panic!("Guess value must be between 1 and 100, got {}.", value);
    }
    Guess { value }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  #[should_panic]
  fn greater_than_100() {
    Guess::new(200);
  }
}
```

- Tests that use `should_panic` can be imprecise. A `should_panic` test would pass even if the test panics for a different reason from the one we were expecting. To make `should_panic` tests more precise, we can add an optional `expected` parameter to the `should_panic` attribute.

## Using `Result<T,E>` in Tests

```rust
#[cfg(test)]
mod test {
  #[test]
  fn it_works() -> Result<(), String> {
    if 2 + 2 == 4 {
      Ok(())
    } else {
      Err(String::from("two plus two does not equal four"))
    }
  }
}
```

## Controlling How Tests are Run

- we can specify how to run the `cargo test` command. We can use flag `--help` which will show options you can use and running `cargo test -- --help` displays the options you can use after the separator (`--`)

## Running Test in Parallel or Consecutively

- when we want to run the test in parallel we need to ensure test don't depend on each other or on any shared state, including a shared environment such as the current working directory or environment variables. For example running test that reads data in the file and asserts that the file contains a particular value which is different in each test.
- alternatively we can run the test to different file or run the test one at a time.
- to not use parallelism: `cargo test -- --test-threads=1`

## Showing Function Output

```rust
fn prints_and_returns_10(a: i32) -> i32 {
  println!("I got the value {}", a);
  10
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn this_test_will_pass() {
    let value = prints_and_returns_10(4);
    assert_eq!(10, value);
  }
  #[test]
  fn this_test_will_fail() {
    let value = prints_and_returns_10(8);
    assert_eq!(5, value);
  }
}
```

- If we want to see printed values for passing tests as well, we can tell Rust to also show the output of successful tests with: `cargo test -- --show-output`.

## Running a Subset of Tests by Name

- we can run test one at the time with: `cargo test neme_of_the_test_function`
- or ignore the test by adding `#[ignore]` under the `#[test]` and run it when we needed with a flag `cargo test -- --ignored`

## Test Organization

- Rust community have two main categories for tests: unit tests and integration tests
- _Unit tests_ are small and more focused, testing one module in isolation at a time an can test private interfaces
- _Integration tests_ are entirely external to your library and use your code in the same way any other external code would, using only the public interface and potentially exercising multiple modules per test

## Unit Tests

- The purpose of unit tests is to test each unit of code in isolation from the rest of the code to quickly pinpoint where code is and isn't working as expected.
- You'll put unit tests in the src directory in each file with the code that they're testing. The convention is to create a module named tests in each file to contain the test functions and to annotate the module with cfg(test).

## The Tests Module and `#[cfg(test)]`

- The `#[cfg(test)]` annotation on the tests module tells Rust to compile and run the test code only when you run `cargo test`, not when you run `cargo build`
- The attribute cfg stands for _configuration_ and tells Rust that the following item should only be included given a certain configuration option

## Testing Private Functions

```rust
pub fn add_two(a: i32) -> i32 {
  internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
  a + b
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn internal() {
    assert_eq!(4, internal_adder(2, 2));
  }
}
```

## Integration Tests

- In Rust, integration tests are entirely external to your library
- the integration test only call functions that are part of your library's public API
- purpose is to test whether many parts of your library work together correcly
- to create integration tests we need to create _tests_ directory

```text
adder
├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests
    └── integration_test.rs
```

```rust
use adder;

#[test]
fn it_adds_two() {
  assert_eq!(4, adder::add_two(2));
}
```

- We don't need to annotate any code in tests/integration_test.rs with `#[cfg(test)]`. Cargo treats the `tests` directory specially and compiles files in this directory only when we run `cargo test`

## Submodules in Integration Tests

- As you add more integration tests, you might want to make more files in the tests directory to help organize them
- each file in the tests directory is compiled as its own separate crate, which is useful for creating separate scopes to more closely imitate the way end users will be using your crate
- However, this means files in the tests directory don’t share the same behavior as files in src do (Chapter 7)

## Integration Tests for Binary Crates

- If our project is a binary crate that only contains a _src/main.rs_ file and doesn’t have a _src/lib.rs_ file, we can’t create integration tests in the tests directory and bring functions defined in the _src/main.rs_ file into scope with a `use` statement. Only library crates expose functions that other crates can use; binary crates are meant to be run on their own.
