# Writing Automated Tests

-In the essay: The Humble Programmer, Edsger W. Dijkstra said that:

> Program testing can be a very effective way to show the presence of bugs, but it is hopelessly inadequate for showing their absence.

## The Anatomy of a Test Function

- simplest a test in Rust is a function that's annotated with the `test` attribute
- attributes are metadata about pieces of Rust code, one example is the `derive` attribute. To change function into test function we need to add `#[test]` on the line above `fn`
- when we run the program with `cargo test` command, Rust builds a test runner binary that runs the annotated functions and reports on wheter each test function passes or fails
