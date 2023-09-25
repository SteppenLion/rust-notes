# Functional Language Features: Iterators and Closures

- one of the inspiration for Rust is _functional programing_. Programing in functional style often includes using functions as values by passing them in arguments, returning them form other functions, assigning them to variables for later execution, etc.

## Closures: Anonymous Functions that Capture Their Environment

- Rust closures are anonymous functions you can save in a variable or pass as arguments to other functions.
- you can create the closure in one place and then call the closure elsewhere to evaluate it in a different context
- unlike functions the closures can capture values from the scope in which they are defined

## Capturing the Environment with Closures

- scenrio: t-shirt company gives away limited edition shirt:

```rust
#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
  Red,
  Blue,
}

struct Inventory {
  shirts: Vec<ShirtColor>
}

impl Inventory {
  fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
    user_preference.unwrap_or_else(||self.most_stocked())
  }

  fn most_stocked(&self) -> ShirtColor {
    let mut num_red = 0;
    let mut num_blue = 0;

    for color in &self.shirts {
      match color {
        ShirtColor::Red => num_red += 1,
        ShirtColor::Blue => num_blue += 1,
      }
    }
    if num_red > num_blue {
      ShirtColor::Red
    } else {
      ShirtColor::Blue
    }
  }
}

fn main() {
  let store = Inventory {
    shirts: vec![ShirtColor::Blue,ShirtColor::Red,ShirtColor::Blue]
  };
  let user_pref1 = Some(ShirtColor::Red);
  let giveaway1 = store.giveaway(user_pref1);
  println!("The user with preference {:?} gets {:?}", user_pref1, giveaway1);

  let user_pref2 = None;
  let giveaway2 = store.giveaway(user_pref2);
  println!("The user with no preference {:?} will gets {:?}",user_pref2, giveaway2 );
}
```

- The [`unwrap_or_else`](https://doc.rust-lang.org/std/option/enum.Option.html#method.unwrap_or_else) method on `Option<T>` is defined by the standard library. It takes one argument: a closure without any arguments that returns a value `T` (the same type stored in the Some variant of the `Option<T>`, in this case `ShirtColor`)
- We specify the closure expression |`| self.most_stocked()` as the argument to `unwrap_or_else`. This is a closure that takes no parameters itself (if the closure had parameters, they would appear between the two vertical bars)

## Closure Type Inference and Annotation

- there differences between functions and closures. Closures don't usually require you to annotate the types of the parameters or the return value like `fn` functions do.
- type annotations are required on functions because the types are part of an explict interface exposed to your users. Defining this interface rigidly is important for ensuring that everyone agrees on what types od values a function uses and returns
- closures on the other hand are not used in an exposed interface like this. They are stores in variables and used without naming them and exposing them to users of our library
- closures are typically short and relevant only within narrow context rater than in any arbitrary scenario
- as with variables we can add type annotations if we want to increase explicitness and clarity at the cost of being more verbose

```rust
let expensive_closure = |num: u32| -> u32 {
  println!("calculating slowly..");
  thred::sleep(Duration::from_secs(2));
  num
};
```

- with type annotation the syntax of the closures looks similar to syntax of functions.
- comparision of functions and closures:

```rust
fn add_one_v1 (x: u32) -> u32 {x + 1}    // funtion
let add_one_v2 = |x:u32| -> u32 {x + 1}; // closure with annotation
let add_one_v3 = |x|            {x + 1}; // closure without anotation
let add_one_v4 = |x|             x + 1;  // closure withut anotation and without curly bractes as there is only one expresion
```

- all above are valid definitions that will produce same behavior
- for closure definitions the compiler will infer one concrete type of their parameters and for their return type

## Capturing References or Moving Ownership

- closures can capture values from their enviroment in three way: borrowing immutably, borrowing mutably, and taking ownership.
- closure will decide which of these to use based on what the body of the function does with the capture values

```rust
fn main() {
  let list = vec![1,3,5];
  println!("Before defining closue: {:?}",list);

  let only_borrows = || println!("From closure: {:?}", list);

  println!("Before calling closure: {:?}",list);
  only_borrows();
  println!("After calling closure: {:?}",list);
}
```

- this example also illustrate that a variable can bind to a closure definition, and we can later call the closure by using the variable name and parentheses as if the variable name were a function name.
- because we can have multiple immutable references to `list` at same time. `list` is still accessible from the code before the closure definition, after the closure definition but before the closure is called , and after the closure is called.
- let change the closure to add an element to the vector:

```rust
fn main() {
  let list = vec![12,2,5];
  println!("Before defining closure: {:?}", list);

  let mut borrows_mutably = || list.push(7);
  borrows_mutably();
  println!("After calling closure: {:?}", list);
}
```

- We don't use the closure again after the closure is called, so the mutable borrow ends. Between the closure definition and the closure call, an immutable borrow to print isn't allowed because no other borrows are allowed when there's a mutable borrow.
- if we want to force the ownership to closure we can use the `move` keyword
- this technique is mostly useful when passing a closure to a new thread to move the data so that it is owned by the new thread

```rust
use std::thread;

fn main() {
  let list = vec![1,2,3];
  println!("Before defining closuer: {:?}",list);

  thread::spawn(move || println!("From thread: {:?}", list)).join().unwrap();
}
```

- In this example, even though the closure body still only needs an immutable reference, we need to specify that `list` should be moved into the closure by putting the `move` keyword at the beginning of the closure definition. The new thread might finish before the rest of the main thread finishes, or the main thread might finish first. If the main thread maintained ownership of `list` but ended before the new thread did and dropped `list`, the immutable reference in the thread would be invalid.

## Moving Captured Values Out of Closures and the `Fn` Traits

- once a closure has capture a reference or capture ownership of a value from the enviroment where the closure is defined, the code in the body of the closure defines what happens to the references or values when the closure is evaluated later. A closure body can do any of the following : move captured value out of closure, mutate the captured value ,neither move nor mutate the value, or capture nothing from the enviroment to begin with.
- the way a closure captures and handles values from enviroment affects which traits the closure implements, and traits are how functions and structs can specify waht kinds of closure they can use
- Closures will automaticlly implement one, two or all three of these `Fn` traits in addhisive fashion

1. `FnOnce` - applies to closure that can be called once. All closures implement at least this trait because all closures can be called. A closure that moves captured values out of its body will only implement `FnOnce` and none of the other `Fn` traits, because it can only be called once.
2. `FnMut` applies to closures that do not move captured values out of their body, but that might mutate the captured values. These closures can be called more than once.
3. `Fn` applies to closures that do not move captured values out of their body and that do not mutate captured values as well, as closures that capture nothing from their enviroment. These closures can be called more than once without mutating their enviroment, which is importatnt in cases such as calling a closure multiple times concurrently

```rust
impl<T> Option<T> {
  pub fn unwrap_or_else<F>(self, f: F) -> T
  where
    F: FnOnce() -> T
    {
      match self {
        Some(x) => x,
        None => f(),
      }
    }
}
```

- The trait bound specified on the generic type `F` is `FnOnce() -> T`, which means `F` must be able to be called once, take no arguments, and return a `T`. Using `FnOnce` in the trait bound expresses the constraint that `unwrap_or_else` is only going to call f at most one time
- Note: Functions can implement all three of the `Fn` traits too. If what we want to do doesn’t require capturing a value from the environment, we can use the name of a function rather than a closure where we need something that implements one of the `Fn` traits. For example, on an `Option<Vec<T>>` value, we could call `unwrap_or_else(Vec::new)` to get a new, empty vector if the value is `None`.
- see how th `sort_by_key` is [defined](https://docs.rs/radsort/latest/radsort/fn.sort_by_key.html):

```rust
pub fn sort_by_key<K, F>(&mut self, mut f: F)
  where
    F: FnMut(&T) -> K,
    K: Ord,
  {
    stable_sort(self, |a, b| f(a).lt(&f(b)));
  }
```

- This function is useful when you want to sort a slice by a particular attribute of each item

```rust
#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

fn main() {
  let mut list = [
    Rectangle { width: 10, height: 1 },
    Rectangle { width: 3, height: 5 },
    Rectangle { width: 7, height: 12 },
  ];

  list.sort_by_key(|r| r.width);
  println!("{:#?}", list);
}
```

- the reason is that the `sort_by_key` is using the `FnMut` closure that can be called multiple times: once for each item in the slice. The closure `|r| r.width` does not capture mutate or move out anything from enviroment so it means that the triat bound requirments are met
- the `Fn` triats are important when defining or using functions or types that makes use od closures

## Processing a Series of Items with Iterators

- in Rust , iterators are _lazy_ meaning they have no effect until you cll methods that consume the iterators to use it up.

```rust
let v1 = vec![1,2,3];
let v1_iter = v1.iter();

for val in i:v1_iter {
  println!("Got : {}", val);
}
```

## The `Iterator` Trait and the `next` Method

- all iterators implement a trait named `Iterator` tht is defined in standard library. Defintion:

```rust
pub trait Iterator {
  type Item;

  fn next(&mut self) -> Option<Self::Item>;

  // methods with default implementations elided
}
```

- this code says implementing the `Iterator` trait requires that you also define an `Item` type(which are defining an _associated type_ with this trait), and this `Item` type is used in the return type of the next method. In other words, the `Item` type will be the type returned from the iterator.
- The `Iterator` trait only requires implementors to define one method: the `next` method, which returns one item of the iterator at a time wrapped in `Some` and, when iteration is over, returns `None`.
- we can call the `next` method directly:

```rust
#[test]
fn iterator_demonstration() {
  let v1 = vec![1, 2, 3];
  let mut v1_iter = v1.iter();

  assert_eq!(v1_iter.next(), Some(&1));
  assert_eq!(v1_iter.next(), Some(&2));
  assert_eq!(v1_iter.next(), Some(&3));
  assert_eq!(v1_iter.next(), None);
}
```

- the values we get from the `next` method are immutable references to the values in the vector
- if we want to create iterator that takes ownership of `v1` and returns owned values we can call `into_iter`
- similarly if we want to iterate over mutable references we can call `iter_mut`

## Methods that Consume the `Iterator`

- The `Iterator` trait has number of different methods with default implementations provided by the standard library. Some of them call `next` method in their definition.
- methods that call `next` method are called _consuming adaptors_ because calling them uses up the iterator. E.g. `sum` method

```rust
#[test]
fn iterator_sum() {
  let v1 = vec![1,3,5];
  let v1_iter = v1.iter();
  let total: i32 = v1_iter.sum();
  assert_eq!(total,9);
}
```

- we are not allowed to use `v1_iter` after the call to `sum` because `sum` takes ownership of the iterator we call it on.

## Methods that Produce Other Iterators

- _Iterator adaptors_ are methods defined on the `Iterator` trait that don't consume the iterator. Instead they produce different iterators by changing some aspect of the original iterator
- Example of calling the iterator adaptor method `map`, which takes a closure to call on each item as the items are iterated through. The `map` method returns a new iterator that produces the modified items. The closure here creates a new iterator in which each item from the vector will be incremented by 1:

```rust
let v1: Vec<i32> = vec![1, 2, 3];
v1.iter().map(|x| x + 1);
```

- this code produces a warning:

```
$ cargo run
   Compiling iterators v0.1.0 (file:///projects/iterators)
warning: unused `Map` that must be used
 --> src/main.rs:4:5
  |
4 |     v1.iter().map(|x| x + 1);
  |     ^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: iterators are lazy and do nothing unless consumed
  = note: `#[warn(unused_must_use)]` on by default

warning: `iterators` (bin "iterators") generated 1 warning
    Finished dev [unoptimized + debuginfo] target(s) in 0.47s
     Running `target/debug/iterators`
```

- this code does not do anything -> the closure its never called. The warning reminds us why: iterator adaptors are lazy, and we need to consume the iterator here.
- to fix this we can call the `collect` method, which will consumes the iterator and collects resulting values into a collection data type

```rust
let v1: Vec<i32> = vec![3,5,56];
let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
assert_eq!(v2, vec![4,6,57]);
```

## Using Closures that Capture Their Environment
