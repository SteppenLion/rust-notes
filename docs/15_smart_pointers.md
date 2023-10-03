# Smart Pointers

- A _pointer_ is general concept for a variable that contains an address in memory
- this address refers to or "points to" some other data
- most common pointer in Rust is reference. References (`&`) don't have any special capabalities other than referring to data and have no overhead
- _Smart pointers_ on the other hand are data structures that act like pointers but also have additional metadata and capabilities
- For example _reference counting_ smart pointer type. This pointer enables us to allow data to have multiple owners by keeping track of the number of owners and, when no owners remain, it will clean up the data
- Rust with concept of ownership and borrowing has an additional difference between references and smart pointers. While references only borrow data in many cases smart pointers _own_ the data they point to.
- few smart pointers we already encountered, including `String` and `Vec<T>`. Both counts as smart pointers because they own some memory and allow us to manipulate it. They also have metadata and extra capabalities or gurantees
- `String` stores its capacity as metadata and has extra ability to ensure its data will always be valid UTF-8
- Smart pointers are usually implemented using structs. Unlike an ordanary structs, smart pointers implement the `Deref` and `Drop` triats
- `Deref` trait allows an instance of the smart pointer struct to behave like a reference so you can write your code that's run when an instance of smart pointer goes out of scope
- Many libraries have their own smart pointers and we can also write are own

## Using `Box<T>` to Point to Data on the Heap

- most straightward smart pointer is a _box_ : type `Box<T>`
- Boxes allow us to store data on the heap rather than the stack. What remains on the stack is the pointer to the heap data
- Boxes do not have performance overhead, other than storing their data on the heap instead on the stack
- They are used in these situations:
  - When we have a type whose size can't be known at compile time and you want to use a value of that type in a context that requires an exact size
  - When you have large amount of data and you want to transfer ownership but ensure the data won't be copied when you do so
  - When you want to own a value and you care only that it is type that implements a paricular trait rather than being of specic type

## Using a `Box<T>` to Store Data on the Heap

- how to interact with values stored within a `Box<T>`:

```rust
fn main() {
  let b = Box::new(5);
  println!("b = {}", b);
}
```
