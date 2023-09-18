# Error Handling

- rust groups errors into two major categories: _recoveravble_ and _unrecoverable_ errors.
- recoverable error, such as file not found, we want to just report the problem to the user and retry the operation
- unrecoverable errors are always symptoms of bugs, it will stop the program all together
- Rust does not have exceptions, instead it has the type `Result<T, E>` for recoverable errors and the `panic!` macro that stops th program encounters an unrecoverable error
