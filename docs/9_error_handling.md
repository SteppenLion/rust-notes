# Error Handling

- rust groups errors into two major categories: _recoveravble_ and _unrecoverable_ errors.
- recoverable error, such as file not found, we want to just report the problem to the user and retry the operation
- unrecoverable errors are always symptoms of bugs, it will stop the program all together
- Rust does not have exceptions, instead it has the type `Result<T, E>` for recoverable errors and the `panic!` macro that stops th program encounters an unrecoverable error

## Unrecoverable Errors with `panic!`

- two ways to panic in practice: by taking the action which results our code to panic (such as accesing an array past the end) or by explicity calling the `panic!` macro.
- **Unwinding the Stack or Aborting in Respsonse to a Panic**: by default when a panic occurs the program starts _unwinding_, which means Rust wallks back up the stack and cleans up the data from each function it encounts. This clean up is a lot of work so Rust allows you to choose the alternative of immediatelly _aborting_ , which ends the program wihtout cleaning up.
- memory that was used by program will then need to be clean up by the operating system. You can change the panic from unwinding to aborting by adding `panic = 'abort'` to the appropriate `[profile]` section in _Cargo.toml_ file. If you want also this option in release mode you need to added in the `[profile.release]`

```rust
fn main() {
  panic!("crash and burn");
}
```

## Using a `panic!` Backtrace

- we can set **RUST_BACKTRACE** enviroment variable to get backtrace of exactly what happend to cause the error. A _bactrace_ is a list of all the functions that have been called to this point.

```sh
RUST_BACKTRACE=1 cargo run
```

## Recoverable Errors with Result

- the `Result` enum is defined as having two variants:

```rust
enum Result<T, E> {
  Ok(T),
  Err(E),
}
```

- `T` represents the type of the value that will be returned in a success case within th `Ok` variant, and `E` in case of `Err`.
- lets see example with opeing the file:

```rust
use std::fs::File;

fn main() {
  let greeting_file_result = File::open("hello.txt");
}
```

- the return type of `File::open` is `Result<T, E>`, In case the file is found and it is accessible the variable `greeting_file_result` will be instance of `Ok`, otherwise it will be instance of `Err`
- to add actions when do what we can use `match` statement

```rust
use std::fs::File;

fn main() {
  let greeting_file_result = File::open("hello.txt");

  let greeting_file = match greeting_file_result {
    Ok(file) => file,
    Err(error) => panic!("Problem with opening the file: {:?}", error),
  };
}
```

- Note that like `Option` enum the `Result` enum and its variants have been brought into scope by the prelude so we do not have to specify the `Result::` before the `Ok` and `Err` variants in the match arms
- after the match (ownership has been ship to the variable `greeting_file`) we can handle the reading and writing.

## Matching on Different Errors

- if we want to also create a file if the file is not found or it is not accessible we can do it in the Error type within the match arms.

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
  let greeting_file_result = File::open("hello.txt");

  let _greeting_file = match greeting_file_result {
    Ok(file) => file,
    Err(error) => match error.kind() {
      ErrorKind::NotFound => match File::create("hello.txt") {
        Ok(fc) => fc,
        Err(ec) => panic!("Problem creating the file: {:?}", ec),
      },
      other_error => {
        panic!("Problem with opening the file: {:?}",other_error);
      }
    },
  };
}
```

- The type of the value that `File::open` returns inside the `Err` variant is `io::Error` which is struct provided by the standard library . This struct has method called `kind` that we cal call to get an `io::ErrorKind` value.
- **Alternatives to Using `match` with `Result<T, E>`**: we can use the closures. E.g. with `unwrap_or_else`:

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
  let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
      if error.kind() == ErrorKind::NotFound {
          File::create("hello.txt").unwrap_or_else(|error| {
              panic!("Problem creating the file: {:?}", error);
          })
      } else {
          panic!("Problem opening the file: {:?}", error);
      }
  });
}
```

## Shortcut for Panic on Error: `unwrap` and `expect`

- the `unwrap` method is a shortcut method implemented just like `match` expression.

```rust
use std::fs::File;

fn main() {
  let greeting_file = File::open("hello.txt").unwrap();
}
```

- similarly the `expect` method lets us choose the `panic!` error message:

```rust
use std::fs::File;

fn main() {
  let greeting_file = File::open("hello.txt")
      .expect("hello.txt should be included in this project");
}
```

## Propagating Errors

- we can propagate (return) the errors to the calling code so we can decide what to do if it returns the error. e.g:

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
  let username_file_result = File::open("hello.txt");
  let mut username_file = match username_file_result {
      Ok(file) => file,
      Err(e) => return Err(e),
  };
  let mut username = String::new();
  match username_file.read_to_string(&mut username) {
      Ok(_) => Ok(username),
      Err(e) => Err(e),
  }
}
```

- if the code that calls this function succeeds it will receive an `Ok` value that holds a `String`. If it fails it will hold an `Err` value with instance of `io::Error` that contains more information.
- if the file is not found insted of the `panic!` macro we use `return` keyword to return early out of the function entirely and pass the error value from `File::open` now in the pattern variable `e` back to the calling code as this function's error value.
- **Shortcut for Propagating Errors with the `?` Operator**: e.g.:

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
  let mut username_file = File::open("hello.txt")?;
  let mut username = String::new();
  username_file.read_to_string(&mut username)?;
  Ok(username)
}
```

- there is diffrence between the `match` and `?` operator does. The error values that goes through the `?` operator the `from` function defined in the `From` trait in the standard library, which is used to convert values from one type into another. When the `?` operator calls the `from` function the error type recived is converted into the error type defined in the return type of the current function. This is useful when a function returns one error type to represent all the ways a function might fail, even if parts might fail for many different reasons.
- we also can use chaining methods calls to shortend the function.

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
  let mut username = String::new();
  File::open("hello.txt")?.read_to_string(&mut username);
  Ok(username)
}
```

- even shorter version using `fs::read_to_string`

```rust
use std::fs;
use std::io;

fn read_username_from_file() -> Result<String, io::Error> {
  fs::read_to_string("hello.txt")
}

```

### Where the `?` operator can be used

- the `?` operator can only be used in functions whose return type is compatible with the value the `?` is used on. If we use the `?` operator in `main()` function it will panic as the main function does not have return type of `Result` by only `()`.
- only functions that have returns `Result`, `Option` or another type that implements `FromResidual`
- the functions that returns `Option<T>` type is similar to `Result<T, E>` if the value is `None` the `None` will be return eaely from the fucntion at that point. Id the value is `Some` the value inside the `Some` is the resulting alue of the expresion and the function continues.

```rust
fn last_char_of_the_first_line(text: &str) -> Option<char> {
  text.lines().next()?.chars().last()
}
```

- `next()` is called to get first value form the iterator that have been returned from the `lines()` function.
- if the next() function does not return any string if will panic and end the execution with returnig the error type (in this case `None`).
- Note you can use the `?` operator on a `Result` in the function that returns `Result`. Same also with `Option`. But you can mix them . The operator won't convert the `Result` to `Option` or vice versa. In this cases you can use `ok` method on `Result` or the `ok_or` method on `Option` to do the conversion explicitly.
- main function can also return a a `Result<(),E>`

```rust
use std::error::Error;
use std::fs::File;

fn main() -> Result<(),Box<dyn Error>> {
  let greeting_file = File::open("hello.txt")?;

  Ok(())
}
```

- the `Box<dyn Error>` type is a trait object -> we can read this object to mean "any kind of error".
- When a `main` function returns a `Result<(), E>`, the executable will exit with a value of `0` if `main` returns `Ok(())` and will exit with a nonzero value if `main` returns an `Err` value. Executables written in C return integers when they exit: programs that exit successfully return the integer `0`, and programs that error return some integer other than `0`. Rust also returns integers from executables to be compatible with this convention.
- The `main` function may return any types that implement the `std::process::Termination trait` -> [doc](https://doc.rust-lang.org/std/process/trait.Termination.html), which contains a function `report` that returns an `ExitCode`. Consult the standard library documentation for more information on implementing the `Termination` trait for your own types.

## Creating Custom Types for Validation
