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

- combination of two existing strings, one way to do it is with `+` symbol

```rust
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2; // s1 is moved here so it can no loger be used
```

- operator `+` is using _add_ method whose signature looks someting like this:

```rust
fn add(self, s: &str) -> String{}
```

- s2 is only reference , this is becasue s parameter in the `add` function we can only add a &str to a String , we can't add two String values together. But why the `&s2` is `&String` and not `&str` as specified in second parameter in add function?
- the reason is that add method can _coerce_ the `&String` argument into a `&str`
- when we call the add method, Rust uses _deref coecion_ which here turns `&s2` into `&s2[..]`
- becasue the add method does not take ownership od the `s` parameter. `s2` will be a valid `String` after operation.
- we can also use the `format!` macro for concatenate the strings

```rust
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = format!("{s1}-{s2}-{s3}");
```

- the `format!` macro works like `println!` but instead of printing the output to the screen, it returns a `String` with the contents. This macro uses references so that this call does not take ownership of any of its parameters.

**Indexing into Strings**

- indexing of individual charakters in the string by referencing the index cannot be done in the Rust. You will get en error. Rust does not support indexing becouse is how it stores the strings in memory.
- A `String` is wrapper over `Vec<u8`.

Example:

```rust
let hello = String::from("Hola");
```

- in this case `len` is 4 which means that the vector storing the string "Hola" is 4 bytes long.
- each letter takes 1 byte when encoded to UTF-8. In different languages (cyrilic -> "Здравствуйте") it can be more than charakters becouse that is number of bytes that UTF-8 stores this word cyrilic word.
- therefor the scalar value will not always corralate to a valid Unicode scalar value.
- We can look at the strings from Rust's perspective as: **bytes**, **scalar values** and **grapheme clusters**
- Rust also don't allow the indexing of strings becasue indexing operation should be always take constant time (O(1)). But this can not be guarantee with `String` becasue Rust will have to go through whole text to validate every charackter in the text.

**Slicing Stings**

- indexing with slices in not good idea as we show in previous example. If we know that every charakter have 2 bytes we can do slices but if the charakter is only 1 byte it Rust program will panic.
- the ranges only do it with caution

**Methods for Iterating Over Strings**

- best way is to be explict whether you want characters or bytes.
- we can use `chars` method to return only characters

```rust
for c in "Здравствуйте".chars() {
  println!("{c}");
}
```

- alternativly we can iterate over bytes with `bytes()` method.

```rust
for b in "Здравствуйте".bytes() {
  println!("{b}");
}
```

**Stings are not so Simple**

- this trade off with strings in Rust is only because as we already show that we can encounter non-ASCII charakters and it will cause panic in Rust program.
- standard library ofers methods like `contains` and `replace` to handle such a cases.

## Storing Keys with Associated Values in Hash Maps

- type `HashMap<K,V>` store a mapping of keys of type `K` to values of type `V` using a _hashing function_ which determines how it places these keys and values into memory.
- Hash maps are useful when we want to look up data not by using an index as with vectors but by using a key that can be of any type.

**Creating a New Hash Map**

- we can create an empty hash map with `new` and adding elements with `insert`

```rust
use std::collections:HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"),10);
scores.insert(String::from("Red"),20);
```

- HashMap have less support from standart library, there is no built-in macro to construct them
- just like vectors hash maps are stored in heap. Above HashMap has keys of type `String` and values of type `i32`. Like vectors hash map are homogenous. all keys must have the same type as each other and also the values must have same type.

**Accessing Values in Hash Map**

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("Blue"),10);
scores.insert(String::from("Red"),20);

let team_name = Sting:from("Blue");
let score = scores.get(&team_name).copied().unwrap_or(0);
```

- `get` method returns an `Option<&V`. If there is no value for htat key in the hash map, `get` will return `None`. This program handles the `Option` by calling coppied to get an `Option<i32>` rather than an `Option<&i32>`. then uses `unwrap_or` to set score to zero if `scores` does not have an entry for the key.
- we can iterate over each key/value pain in hash map in a similar manner as we do with vectors

```rust
for (key,value) in &scores {
    println!("this is key: {key} and this is value: {value} in the hash map");
}
```

**Hash Maps and Ownership**

- for types that implement the `Cope` trait like `i32` the values are coped into hash map.
- for owned vaues like `String` the values will be moved and the hash map will be owner of those values:

```rust
use std::collections::HashMap;

let field_name = String::from("Favorite color");
let field_value = String::from("Blue");
let mut map = HashMap::new();
map.insert(field_name, field_value);
// field_name and field_value are invalid at this point
```

- if we insert reference to hash map the values won't be moved into a hash map.
- the values that the refernces point to must be valid for at least as long as the hash map is valid

**Updating a Hash Map**

- owerwrite the value:

```rust
let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Blue"), 25);
```

- adding a key and vlue only if key isn't present: Hash map have special API for that called `entry`.
- `entry` thas the keyh you want to check as parameter. The return value of the entry method is an enum called `Entry` that represents a value that might or might not exist.

```rust
use std:: collections::HashMap;

let mut scores - HashMap::new();
scores.insert(String::from("Blue"),10);

scores.entry(String::from("Yellow")).or_insert(50);
scores.entry(Sting::from("Blue")).or_insert(50);

println!("{}",scores);
```

- `on_insert` method on `Entry` is defined to retrun a mutable reference to the value for the corresponding `Entry` key if that key exists, and if not , inserts the paratmer as the new value for this key and returns a mutable reference to the new vlaue.
- if we want to update value based on the old value:

```rust
use std::collections::HashMap;

let text = "hello world wonderful world";
let mut map = HashMap::new();
for word in text.split_whitespace() {
  let count = map.split_whitespace(word).or_insert(0);
  *count += 1;
}

println!("{:?}", map);
```

- `split_whitespace` method returns an iterator over sub-slices separated by whitespace of the iteratrating value. `or_insert` returns mutable reference (`&mut V`) to the value for the specified key. The mutable reference we stored in the `count` variable, in order to assign to that value we must first _dereference_ `count` using the asterisk `*`. Mutalbe reference goes out of scope at the end of the loop.

## Hashing Functions

- by default `HashMap` is using function called _SipHash_ that can provide resistance to Denial of Service (Dos) attacks involving hash tables. This is not fastest hashing algorithm available but trade off for better security taht comes with the drop in performance is worth it. You can switch to differnt hashing function specifing in the `BuiltHasher` trait.
