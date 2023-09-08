# Common Collections

- Rust's standard library includes a number of very useful data structures called _collections_.
- Unlike the built-in array and tuple types, the data these collections point to is stored on the heap, which means the amount of data does not need to be known at compile time and can grow or shrink as the program runs.
  - A vector allows you to store a variable number of values next to each other.
  - A string is a collection of characters.
  - A hash map allows you to associate a value with a particular key. It's a particular implementation of the more general data structure called a map.

## Storing Lists of Values with Vectors

- `Vec<T>` also known as a _vector_. Allows to store more than one value in single data structure that puts all the values next to each other in memory. Vectors can only store values of same type.
- useful when we have a list of items such as lines of text in the file or the prices of items.

### Creating a New Vector

```rust
fn main() {
  let v: Vec<i32> = Vec::new();
}
```

- rust provides the the `vec!` macro which holds the values.

```rust
fn main(){
  let v = vec![1,2,3];
}
```

### Updating a Vector

```rust
fn main() {
  let mut v = Vec::new();
  v.push(5);
  v.push(6);
}
```

### Reading Elements of Vector

- via indexing or using `get` method

```rust
fn main() {
  let v = vec![1,2,3,4,5];
  let third: &i32 = &v[2];
  println!("the third element is {}",third);

  let third: Option<&i32> = v.get(2);
  match third {
    Some(third) => println!("The third element is {}",third),
    None => println!("There is no third element."),
  }
}

```

- if we use indexing and the element is not present in the vector => this will cause the program to panic.
- by using `get` method we can passed this issue by returning `None` without panicing.

```rust
let v = vec![2,4,6,2];
let does_not_exist = &v[100]; // it will panic becouse we are referencing not existing element
let does_not_exist = v.get(100); // returns None without panicking
```

### Iterating over the Values in a Vector

- example of `for` loop

```rust
fn main() {
  let v = vec![133,44,52];
  for i in &v {
    println!("{}",i);
  }
}
```

- also iterate over mutable references to each element in a mutable vector to make changes to all the elements.

```rust
fn main() {
  let mut v = vec![24,53,63];
  for i in &mut v {
    *i += 50;
  }
}
```

- to change the value that the mutable reference refers to we have to use `*` dereference operator to get to the value in `i` before we can use `+=` operator.

### Using an Enum to Store Multiple Types

- becasue the vector can store values only with same types we need to use enums to store multiple types.

```rust
fn main() {
  enum SpredsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
  }
  let row = vec![
    SpredsheetCell::Int(3),
    SpredsheetCell::Text(String::from("blue")),
    SpredsheetCell::Float(10.12),
  ]
}
```

- other methods we can find in the standard library of [std::vec::Vec](https://doc.rust-lang.org/std/vec/struct.Vec.html) -> as `pop`, `clear`, `insert`, etc.

### Droping a Vector Drops Its Elements

- as other `struct` a vector is freed when it goes out of scope

```rust
fn main() {
  {
    let v = vec![2,4,5,2];
    // do stuff with v
  } // <- v goes out of scope and is freed here
}
```

## Storing UTF-8 Encoded Text with String

- Rust has only one sting type in the core language, which is the string slice `str` that is usually seen in its borrowed form `&str`

### Creating a New String

- many operations available with `Vec<T>` are available with `String` as well. Because `String` is implemented as a wrapper around a vector of bytes with some extra guarantees, restrictions and capabilites.

```rust
fn main() {
  let mut v = String::new();
}
```

- creates a new empty string, which we can then load data into.
- often we have startinf string, we can use `to_string` method, which is available on any type that implements the `Display` trait

```rust
fn main() {
  let data = "initial contents";
  let s = data.to_string();
  // the method also works on a literal directly:
  let s = "initial contents".to_string();
}
```

- we can also use the function `String::from` to crate `String` from a string literal.(equivalent)

### Updating a String

- to push more data to `Vec<T>` we can use `+` operator or the `format!` macro to concatenate `String` values

**Apeending to a String with `push_str` and `push`**

```rust
fn main() {
  let mut s = String::from("foo");
  s.push_str("bar");
}
```

- `push_str` method takes a string slice becouse we don't necessarily want to take ownership of the parameter. for example we can to be able to use `s2` after appending its content to `s1`

```rust
fn main() {
  let mut s1 = String::from("foo");
  let s2 = "bar";
  s1.push_str(s2);
  println!("s2 is {s2}");
}
```

**Concatennation wiht the `+` operator or the `format!` macro**
