# Understanding Ownership

- ownership enables Rust to make memory safty guarantees without a garbage collector
- it is set of rules that govern how Rust program manages memory.

## Stack and Heap

- stack stores values in the order it gets them and removes the values the values in opposite order. -> Last in first out -> LIFO
- adding data is called _pushing onto the stack_ and removing as _popping off the stack_
- all data stored on the stack must have a known, fixed size.
- data with an unknown size at compile time or the size that might change must be stored on the heap instead.
-
- heap is less organize -> when tou put data on the heap, you request a certain amount of space.
- memory allocator finds an empty spot in the heap that is big enough, marks it as being in use and returns a pointer -> which is the address of the location. This process is called _allocating on the heap_ or just _alocating_
- because the pointer to the heap is known, fixed size, we can store pointer on the stack, but when you want the actual data we need to follow the pointer
- keeping track of what parts of code are using what data on the heap, minimizing the amount of duplicate data on the heap, and cleaning up unused data on the heap so you don't run out of space are all problems that ownership addresses.
- main purpose of ownership is to manange heap data

## Ownership rules

1. Each value in Rust has an owner
2. There can only be one owner at a time
3. When the owner goes out of scope, the value will be dropped

## Variable scope

```rust
fn main() {
    {                      // s is not valid here, it’s not yet declared
        let s = "hello";   // s is valid from this point forward

        // do stuff with s
    }                      // this scope is now over, and s is no longer valid
}

```

- we need to look at data that is stored on the heap:
- `String` string type -> this type manages data alloceted on the heap and as such is able to store an amount of text that is unknown to us at compile time

```rust
fn main() {
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s); // This will print `hello, world!`
}
```

- double colon `::` operator allows us to namespace this particular `from` function under `String` type rather than using some sort of name like -> string_from.
- difference between `String` and string literals is how these two types deal with memory

## Memory and Allocation

- with `String` type , in order to support a mutable, growable piece of text, we need to allocate an amount of memory on the heap, unknown at compile time, to hold the content:

  - The memory must be requested from the memory allocator at runtime
  - we need a way of returing this memory to the allocator when we are done.

- first part is done by `String::from` -> implementation request memory it needs (universal in programing languages)
- in other languages we need to implement Garbage collector, in Rust the memory is automaticaly returned once the variable tha owns it goes out of scope.

```rust
fn main() {
    {
        let s = String::from("hello"); // s is valid from this point forward
        // do stuff with s
    }                                  // this scope is now over, and s is no
                                       // longer valid
}
```

- when `s` goes out of scope, Rust calls a special function for us. The funcion is called `drop` and it's where author od `String` can put the code to return the memory. Rust call `drop` automaticaly at the closing curly bracket.

### Ways Variables and Data Interact : Move

- multiple variables can interact with same data in different way in Rust.

```rust
fn main() {
    let x = 5;
    let y = x;
}
// String version
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;
}
```

- the String version is different as first version. `String` is made up of three parts ([picture](https://doc.rust-lang.org/book/img/trpl04-01.svg)) -> pointer to memory that holds the content of the string, a length, and capacity. On the right of the picture is the memory on the heap that holds the content
- when assign s1 to s2 the `String` data is copied, meaning we copy the pointer the length and the capacity htat are on the stack. We do not copy the data on the heap that the pointer refers to.
- it was said that if variable goes out of scope Rust automaticaly calls `drop` funciton adn cleans up the heap memory for that variable. This is problem: when s1 and s2 go out of scope htey will both try to free the same memory -> this is known as _double free_ error and is one of the memory safty bugs we mentiond previosly. Freeing memory twice can lead to memory coruption, which can lead to security vulnerabilites.
- to ensure memory safety the line `let s2 = s1` Rust considers `s1` as no longer valid.
- Therefor Rust doesn't need to free anything when `s1` goes out of scope

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{}, world!", s1);
}
// ERROR

// $ cargo run
//    Compiling ownership v0.1.0 (file:///projects/ownership)
// error[E0382]: borrow of moved value: `s1`
//  --> src/main.rs:5:28
//   |
// 2 |     let s1 = String::from("hello");
//   |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
// 3 |     let s2 = s1;
//   |              -- value moved here
// 4 |
// 5 |     println!("{}, world!", s1);
//   |                            ^^ value borrowed here after move
//   |
//   = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)

// For more information about this error, try `rustc --explain E0382`.
// error: could not compile `ownership` due to previous error


```

- in other languages their is concept of shallow and deep copy. Shallow copy is when the only pointer, length, capacity is copied. But becouse Rust invelidates the first variable, instead of shallow copy it's known as _move_. So we can say that s1 was _moved_ to s2.
- in addtion Rust wiil never automaticaly create "deep" copies of your data.
- automatic copying can be assumed to be inexpesive in terms od runtime performace

### The Ways Variables and Data Interact: Clone

- if we want to deeply copy the heap data of the `String` not just stack data we ca use a coomon method called `clone`

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}
```

## Ownership and Functions

- same as assigning value to variable
- passing a variable to a funciton will move or copy, just as assignment does.

```rust
fn main() {
    let s = String::from("hello");  // s comes into scope
    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here
    let x = 5;                      // x comes into scope
    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward
} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.
fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.
fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
```

- if we try to use `s` after `takes_ownership`, Rust would throw a compile-time error.

## Return Values and Scope

- returning values can also transfer ownwership

```rust
fn main() {
    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1
    let s2 = String::from("hello");     // s2 comes into scope
    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.
fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it
    let some_string = String::from("yours"); // some_string comes into scope
    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}
// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope
    a_string  // a_string is returned and moves out to the calling function
}
```

- ownership of a variable follows the same pattern every time: assigning a value to another variable moves it. When a variable that includes data on the heap goes out of scope the value will be cleaned up by `drop` unless ownership of the data has been moved to anther variable.
- can be tedios to takes ownership of the value everytime and need to be passed back.
- we can return multiple values with Rust using tuples:

```rust
fn main() {
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String
    (s, length)
}
```

## Reference and Borrowing

- issue with returning tuple is we need to return `String` to the calling function so we can still use the `String` after call to `calculate_length` becouse `String` was moved to into `calculate_length` function. Instead we can provide a reference to the `String` value.
- a _reference_ is like a pointer in that it's an address we can follow to access the data stored at that address -> that data is owned by some other variable.
- unlike a pointer, a reference is guaranteed to point to a valid value of a particular type for the life of that reference.
- example of using reference:

```rust
fn main() {
  let s1 = String::from("hello");
  let len = calculate_length(&s1);
  println!("the length of '{}' is {}. ",s1,len);
}
fn calculate_length(s: &String) -> usize {
  s.len()
}// Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.
```

- `&` ampersand represend reference and allow us to refer to some value without taking ownership of if. picture to vizulize it -> [link](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
- Note: opposite of referencing `&` is dereferencing which is accompished wih the derefernce operator `*`
- as we don't own the value we only reference to it we don't need to gives back ownership as we don't have it.
- we call action of creating a reference a _borrowing_
- just as variable are immutable by default so are refrences. We can not modify something we have a reference to.

## Mutable references

- we can add `mut` keyword to make refrence mutable -> borrowed value

```rust
fn main() {
  let mut s = String::from("Hello");
  change(&mut s);
}
fn change(some_string: &mut String) {
  some_string.push_str(", world");
}
```

- creating mutable reference with `&mut s` where we call `change` function and update the function signature to accept a mutable refence.
- one restriction of mutable refrences is that if you have mutable refrence to a value you only can have one reference to that value.
- this is speciality of Rust. This restriction is for Rust to prevent data races at compile time.
- _data races_ is similar to race condition and happens when these three behaviours occur:

  1. Two or more pointers access the same data at the same time
  2. At least one of the pointers is being used to write to the data
  3. there is no mechnism being used to synchronize access to the data

- data races cause undefined behaviour and can be difficult to diagnose and fix

- we can always use curly brackets to crate a new scope, allowing for multiple mutalbe references, just not _simultaneous_ ones:

```rust
fn main() {
  let mut s = String::from("hello");
  {
    let r1 = &mut s;
  } // r1 goes out of scope here, so we can make a new reference with no problems
  let r2 = &mut s;
}
```

- we also can not have a mutable refence while we have an immutable one to the same value.
- this code will run because the last usage of the immutable refences the `println!` occurs before the mutable refence is introduced

```rust
fn main() {
  let mut s = String::from("hello");
  let r1 = &s;
  let r2 = &s;
  println!("{} and {}",r1,r2);

  let r3 = &mut s;
  println!("{}",r3);
} // these references don't overlap so it is no problem
```

- The ability of the compiler to tell that a refence is no longer being used at a point before the end of the scope is called _Non-Lexical Lifetimes_ (NLL for short)

## Dangling References

- in languages with pointers it is easy to create dangling pointers -> pointer that refences a location in memory that may have been given to someone else ==> by freeing some memory while preserving a pointer to that memory
- Rust compiler guarantees that reference will never be dangling refrence
- compiler will ensure that your data will not go out of scope before the reference to the data does.

```rust
fn main() {
  let reference_to_nothing = dangle();
}
fn dangle() -> &String {
  let s = String::from("hello");
  &s
}
// $ cargo run
//    Compiling ownership v0.1.0 (file:///projects/ownership)
// error[E0106]: missing lifetime specifier
//  --> src/main.rs:5:16
//   |
// 5 | fn dangle() -> &String {
//   |                ^ expected named lifetime parameter
//   |
//   = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
// help: consider using the `'static` lifetime
//   |
// 5 | fn dangle() -> &'static String {
//   |                ~~~~~~~~

// For more information about this error, try `rustc --explain E0106`.
// error: could not compile `ownership` due to previous error
```

- beacue the `s` is created inside `dangle` when the code is the of the `dangle` the `s` will be delocated. But we try to return reference to it. So reference will be pointing to an invalid `String`
- solution is to return `String` directly:

```rust
fn main() {
    let string = no_dangle();
}
fn no_dangle() -> String {
    let s = String::from("hello");
    s
}
```

## The Rules of References

- at any given time tou can have either one mutalbe reference or any number of immutable references.
- References must always be valid

## The Slice type

- _Slices_ let you refence a contigous sequence of elements in collection rather than the whole collection. A slice is a kind of refence, so it does not have ownership.
- on the function, that takes a string of words separeted by spaces and returns the first word it finds in the string, we explain the slices

```rust
fn first_word(s: String) -> usize {
  let bytes = s.as_bytes();

  for (i, &item) in bytes.iter().enumerate() {
    if item == b' '{
      return i;
    }
  }
  s.len()
}
fn main(){
  let mut s = String::from("hello world");
  let word = first_word(&s); // word will get the value 5
  s.clear(); // this empties the String, making it equal to ""
  // word still has the value 5 here, but there's no more string that
  // we could meaningfully use the value 5 with. word is now totally invalid!

}
```

- we use `iter` method that returns each element in a collection and that `enumerate` wraps the result of `iter` and returns each element as part of tuple instead. First in tuple is index and second is element as refence to the element. (better than calculating the index ourselves).
- we are returnin `usize` on its own -> but it is meaningful number in the context of the `&String`. Becouse its a separeted value from the `String` there no guarantee that it will still be valid in the future.
- getting to worry about index in `word` getting out of sync with the data in `s` is tedious and error prone. Iss more tedious if we need track second word with a new function -> new variables and so on.

## String slices

- _string slices_ is reference to part of a `String` and it looks like this:

```rust
fn main() {
  let s = String::from("heelo world");
  let hello = &s[0..5];
  let world = &s[6..11];
}
```

- rather to reference to entire `String` , variable `hello` is refrence to a portion of the `String`
- if you want to start at zero you can drop the value before the two periods like this `[..4]` or if you want to the end you can drop the last value -> `[6..]`
- if you want entire string -> `[..]`

```rust
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
// add #![allow(unused_variables)] at top of the file to prevent compile errors
fn main() {
    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
}
```

## Other Slices

- we can also refer to a part of an array:

```rust
#![allow(unused)]
fn main() {
  let a = [1,2,3,4];
  let slice = &a[1..3];
  assert_eq!(slice,&[2,3]);
}
```

- slices ensures memory safety in Rust Programs at compile time.
