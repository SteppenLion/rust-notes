# Common Collections

- Rust’s standard library includes a number of very useful data structures called _collections_.
- Unlike the built-in array and tuple types, the data these collections point to is stored on the heap, which means the amount of data does not need to be known at compile time and can grow or shrink as the program runs.
  - A vector allows you to store a variable number of values next to each other.
  - A string is a collection of characters.
  - A hash map allows you to associate a value with a particular key. It’s a particular implementation of the more general data structure called a map.

## Storing Lists of Values with Vectors

- `Vec<T>` also known as a _vector_. Allows to store more than one value in single data structure that puts all the values next to each other in memory. Vectors can onlt store values of same type.
-
