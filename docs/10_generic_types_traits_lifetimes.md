# Generic Types, Traits, and Lifetimes

- instead of creating the functions with conrete type we can use the _generics_ to abstract type.
- also we can use the _traits_ to define behaviour in generic way. We can combine the traits with generic types to contrain a generic type to accept only those types that have particular behaviour as just to opposed any type.
- _lifetimes_ - are variaty of generic hat give the compiler information about how refrences relato to each other. Lifetimes allow us to give the compiler enough information about borrowed values so that it can ensure refrences will be valid in more situations that it could without our help.

## Removing Duplication by Extracting a function

- generics allows to replace specific types with placeholder that represents multiple types to remove code duplication. First we can show how function without generics:

```rust
fn main() {
  let number_list = vec![2,321,3124,122];
  let mut largest = &number_list[0];

  for number in &number_list {
    if number > largest {
      largest = number;
    }
  }
  println!("The largest number is {}", largest);
}
```

- for multiple list:

```rust
fn find_largest_num_in_vector(vector: &[i32]) -> &i32 {
  let mut largest = &vector[0];

  for number in &vector {
    if number > largest {
      largest = number
    }
  }

  largest
}

fn main() {
  let number_list = vec![34, 50, 25, 100, 65];
  let result = find_largest_num_in_vector(&number_list);
  println!("The largest number is {}", result);

  let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
  let result = find_largest_num_in_vector(&number_list);
  println!("The largest number is {}", result);
}
```

- using generics where the input is can be also `i32` and also the `char` type

```rust
fn largest<T>(list: &[T]) -> &T {
  let mut largest = &list[0];

  for item in list {
    if item > largest {
      largest = item;
    }
  }
  largest
}

fn main() {
  let number_list = vec![34, 50, 25, 100, 65];
  let result = largest(&number_list);
  println!("The largest number is {}", result);

  let char_list = vec!['y', 'm', 'a', 'q'];
  let result = largest(&char_list);
  println!("The largest char is {}", result);
}
```

- this program will not compile as the compilator will promte us that we need to restrict the type parameter `T` and tells us to add trait to parameter `<T: std::cmp::PartialOrd>`
- the error states that body of `largest` won't work for all possible ypes that `T` could be
- because we want to compare values of type `T` in he body we can only use types whose values can be ordered
- to enable comparision -> we can use above standard library trait. And after adding the trait the program will compile because the std implements `PartialOrd` on both `i32` and `char`

## In Struct Definitions

- we can define structs to use genercic type parameter in one or more fields using `<>` syntax
- we need to specify all allowed type in struct that can be use in the object. In the `Point` it is integer `i32` or float `f64`.

```rust
struct Point<T> {
  x: T,
  y, U,
}
fn main() {
  let integer = Point {x: 4,y: 2};
  let float = Point {x:2.2,y 3.3};
}
```

## In Enum Definitions

- in the enums we ca specify also mutlimle generics types as in struct as well.

```rust
enum Result<T,E> {
  Ok(T),
  Err(E),
}
```

## In Method Definitions

```rust
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
     fn y(&self) -> &T {
        &self.y
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {} and p.y: {}", p.x(),p.y());
}
```

- note that we have to declare `T` just after `impl` so we can use `T` to specify that we are implemeniting methods on the type `Point<T>`
- By declaring it after `impl`, Rust can identify that the type in the angle vackets in `Point` is a generic type rather than concrate type
- we can also specify constrains on generic types when defining methods on the type.
- e.g. only for concrete type `f32`

```rust
impl Point<f32> {
  fn distance_from_origin(&self) -> f32 {
    (self.x.powi(2) + self.y.powi(2)).sqrt()
  }
}
```

- above code means the type `Point<f32` will have a `distance_from_origin` method and other instances of `Point<T>` where `T` is not of type `f32` will not have this method defined
- genercic type parmaters in a struct definitions are not the same as those yhou sue in that same struct's signature. For example:

```rust
struct Point<X1, Y1> {
  x: X1,
  y:Y1,
}

impl <X1, Y1> Point<X1,Y1> {
  fn mixup<X2,Y2>(self,other: Point<X2,Y2>) -> Point<X1,Y2> {
    Point {
      x: self.x,
      y: other.y
    }
  }
}

fn main() {
  let p1 = Point { x:5, y: 10.4};
  let p2 = Point {x: "Hello", y: 'c'}

  let p3 = p1.mixup(p2);

  println!("p3.x = {}, p3.y = {}",p3.x, p3.y);
}
```

## Performance of Code Using Generics

- using generics won't make program run any slower than it would with concrate types
- Rust accomplishes this performing monomorphization of the code using generics at compile time
- _Monomorphization_ is process of turning generic code into specific code by filling in tghe concrete types that are used when compliled
- how it work by using standard library's generic `Option<T>` enum:

```rust
let integer = Some(5);
let float = Some(3.3);
```

- when Rust compiles this code it performs monomorphization. During that process the compiler reads values that have been used in `Option<T>` instances and identifies two kinds of `Option<T>` => `i32` and `f64`. As such it expands the generic definition of `Option<T>` into two definitions, thereby replacing the genric definition with the specific ones

## Traits: Defining Shared Behavior

- _trait_ defines functionality a particular type has and can share with other types. We can use traits to define shared behaviour in an abstract way. (in other languages are traits called _interfaces_ although with some differences)

## Defining a Trait

- trait definition are a way to group method signatures together to define a set of behaviours necessary to accomplish some purpose

```rust
pub trait Summary {
  fn summarize(&self) -> String;
}
```

- the compiler will enforce that any type that have the `Summary` trait will have the method `summarize` defined with this signature exactly
- trait can have multiple methods in its body. the methods signatures are listed one per line and each line ends in a semicolon

## Implementig a Trait on a Type

- now when we define the desired signatures of the `Summary` trait's methods we can implement it on types in our media aggregator

```rust
pub struct NewsArticle {
  pub headline: String,
  pub location: String,
  pub author: String,
  pub content: String,
}

impl Summary for NewsArticle {
  fn summarize(&self) -> String {
      format!("{}, by {} ({})", self.headline, self.author, self.location)
  }
}

pub struct Tweet {
  pub username: String,
  pub content: String,
  pub reply: bool,
  pub retweet: bool,
}

impl Summary for Tweet {
  fn summarize(&self) -> String {
      format!("{}: {}", self.username, self.content)
  }
}
```

- implemeniting a trait on a type is similar to implemeniting regular methods
- difference is that afer `impl` we put the trait name we want to implement, than use the `for` keyword and then specify the name of the type we want to implement the trait for. Within block we put methods signatures that the trait deifntion has defined. Instead of semicolon we use curly brackets and fill in the method body with specific behaviour that we want the methodof the trait to have for the particular type.
- now we can call the trait methods on instances od `NewsAsticle` and `Tweet` in a same way we call regular methods. The only difference is that the user must bring he trait into scope as well as the types. E.g.:

```rust
use aggregator::{Summary,Tweet};

fn main() {
  let tweet = Tweet {
    username: String::from("ebooks"),
    content: String::from("some content here"),
    reply: false,
    retweet: false,
  };
  println!("this one tweet: {}", tweet.summarize());
}
```

- One restriction to note is that we can implement a trait on a type only if at least one of the trait or the type is local to our crate. For example, we can implement standard library traits like `Display` on a custom type like `Tweet` as part of our `aggregator` crate functionality, because the type `Tweet` is local to our `aggregator` crate. We can also implement `Summary` on `Vec<T>` in our `aggregator` crate, because the trait `Summary` is local to our `aggregator` crate.
- But we can't implement external traits on external types. For example, we can't implement the `Display` trait on `Vec<T>` within our `aggregator` crate, because `Display` and `Vec<T>` are both defined in the standard library and aren't local to our `aggregator` crate. This restriction is part of a property called _coherence_ and more specifically the _orphan rule_ so named because the parent is not present. This rule ensures that other people's code can not break your code and vice versa. Without this rule two crates can implement the same trait for same type and Rust would not know which implementation to use

## Default Implementation

- Sometimes it's useful to have default behavior for some or all of the methods in a trait instead of requiring implementations for all methods on every type.
- then we can implement the trait on a particular type, we can keep or override each methos's default behaviour

```rust
pub trait Summary {
  fn summarize(&self) -> String {
      String::from("(Read more...)")
  }
}
pub struct NewsArticle {
  pub headline: String,
  pub location: String,
  pub author: String,
  pub content: String,
}

impl Summary for NewsArticle {}

fn main() {
  let article = NewsArticle {
      headline: String::from("Penguins win the Stanley Cup Championship!"),
      location: String::from("Pittsburgh, PA, USA"),
      author: String::from("Iceburgh"),
      content: String::from(
          "The Pittsburgh Penguins once again are the best \
         hockey team in the NHL.",
      ),
  };
  println!("New article available! {}", article.summarize());
}
```

- we can expand the `Summary` trait to have a `summarize_author` method whose implementation is required and then define a `summarize` method that has a default implementation that calls the `summarize_athor` method

```rust
pub trait Summary {
  fn summarize_author(&self) -> String;
  fn summarize(&self) -> String {
    format!("(Read more from {}...)", self.summarize_author())
  }
}
```

- To use this version of `Summary`, we only need to define `summarize_author` when we implement the trait on a type:

```rust
impl Summary for Tweet {
  fn summarize_author(&self) -> String {
    format!("@{}", self.username)
  }
}
```

- after we define `summarize_author`, we can call `summarize` on instances of the `Tweet` struct, and the default implementation of `summarize` will call the definition of `summarize_author` that we've provided. Because we've implemented `summarize_author`, the `Summary` trait has given us the behavior of the `summarize` method without requiring us to write any more code.

```rust
fn main() {
  let tweet = Tweet {
    username: String::from("horse_ebooks"),
    content: String::from(
      "of course, as you probably already know, people",
    ),
    reply: false,
    retweet: false,
  };
  println!("1 new tweet: {}", tweet.summarize()); // 1 new tweet: (Read more from @horse_ebooks...)
}
```

- Note: that it isn't possible to call the default implementation from an overriding implementation of that same method.

## Traits as Parameters

- we can implement method that can take trait as parameter

```rust
pub fn notify(item: &impl Summary) {
  println!("Breaking news! {}", item.summarize());
}
```

- instead of concrate type for the `item` parameter we specify the `impl` keyword and the trait name. This parameter accepts any type that implements the specified trait.
- We can call notify and pass in any instance of `NewsArticle` or `Tweet`
- Code that calls the function with any other type, such as a `String` or an `bool`, won't compile because those types don't implement Summary.

## Triat Bound Syntax

- `impl Trait` syntax works for straightforward cases but is actually syntax sugar for longer form known as _triat bound_ and looks like this:

```rust
pub fn notify<T:Summary>(item:&T) {
  println!("Breaking news! {}", item.summarize());
}
```

- this is more verbose eqivalent. We place trait bounds with the declaration of the generic type parameter after a colon and inside angle brackets
- for more complexity with two parameters that implent `Summary`

```rust
pub fn notify(item1: &impl Summary, item2 :&impl Summary ) {}
```

- this allows to have different types (as long as both types implments `Summary`)
- if we want to force both parameters to have the same type , however, we must use a trait bound like this:

```rust
pub fn notify<T: Summary>(item1: &T,item2: &T) {}
```

## Specifing Multiple Trait Bounds with the `+` Syntax

- more than one trait bound we can use it like this:

```rust
pub fn notify<item: &(impl Summary + Display)>
// also valid syntax for generics
pub fn notify<T: Summary + Display>(item: &T)
```

## Clearer Trait Bounds with `where` Cluses

- too many trait bounds has its downside ->hard to read. Rust has alternate syntax for specifing trait bounds inside a `where` cluse after the function signature.
- so instead of this:

```rust
fn some_fun<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}
```

- we can use `where` cluse:

```rust
fn some_fun<T,U>(t: &T, u: &U) -> i32
where
  T: Display + Clone,
  U: Clone + Debug,
```

## Returning Types that Implement Traits

- We can also use the `impl Trait` syntax in the return position to return a value of some type that implements a trait, as shown here:

```rust
fn returns_summarizable() -> impl Summary {
  Tweet {
    username: String::from("horse_ebooks"),
    content: String::from(
      "of course, as you probably already know, people",
    ),
    reply: false,
    retweet: false,
  }
}
```

- we specify that the `returns_summarizable` function returns some type that implements the `Summary` trait without naming the concrete type
- In this case, `returns_summarizable` returns a `Tweet`, but the code calling this function doesn't need to know that.
- return type only by the trait it implements is especially useful in the context of closuers and iterators. Both of them create types that only the compiler knows ortypes that are very long to specify.
- However, you can only use impl Trait if you're returning a single type. You can use `if` & `else` in the body of function that returns type

## Using Trait Bounds to Conditionally Implement Methods

- By using a trait bound with an `impl` block that uses generic type parameters, we can implement methods conditionally for types that implement the specified traits.

```rust
use std::fmt::Display;

struct Pair<T> {
  x: T,
  y: T,
}
impl<T> Pair<T> {
  fn new(x:T,y:T) -> Self {
    Self {x, y}
  }
}
impl<T:Display + PartialOrd> Pair<T> {
  fn cmp_display(&self) {
    if self.x >= self.y {
      println!("The largerst number is x = {}" ,self.x);
    } else {
      println!("The largest number is y = {}", self.y);
    }
  }
}
```

- We can also conditionally implement a trait for any type that implements another trait. Implementations of a trait on any type that satisfies the trait bounds are called _blanket implementations_ and are extensively used in the Rust standard library
- e.g. standart lib implement `ToString` trait on any type that implements the `Display` trait and in the standard library it looks like this:

```rust
impl<T:Display> ToString for T{
  // --snip--
}
```

- thats why we can call `to_string()` function on the integers because the integers implemets the `Display` trait
- Blanket implementation appear in the documentation for the traint in the "Implementors" section

## Validating References with Lifetimes

- Lifetimes are another kind of generic that we've already been using. Rather than ensuring that a type has the behavior we want, lifetimes ensure that references are valid **as long as we need them to be**.
- Annotating lifetimes is not even a concept most other programming languages have

## Preventing Dangling References with Lifetimes

- main reason is to prevent _dangling references_ which cause a program to reference data other than it's intended to reference.

```rust
fn main() {
  let r;

  {
    let x = 5;
    r = &x;
  }
  println!("r: {}",r);
}
```

- The outer scope declares a variable named r with no initial value, and the inner scope declares a variable named x with the initial value of 5. Inside the inner scope, we attempt to set the value of r as a reference to x. Then the inner scope ends, and we attempt to print the value in r. This code won't compile because the value r is referring to has gone out of scope before we try to use it.

## Generic Lifetimes in Functions

- function that returns the longer of two string slices:

```rust
fn longest(x: &str, y: &str) -> &str {
  if x.len() > y.len() {
    x
  } else {
    y
  }
}

fn main() {
  let string1 = String::from("abcd");
  let string2 = "xyz";

  let result = longest(string1.as_str(), string2);
  println!("The longest string is {}", result);
}
```

- it won't compile, error: `missing lifetime specifier
this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from x or y`
- When we're defining this function, we don't know the concrete values that will be passed into this function, so we don't know whether the `if` case or the `else` case will execute
- We also don't know the concrete lifetimes of the references that will be passed in
- The borrow checker can't determine this either, because it doesn't know how the lifetimes of x and y relate to the lifetime of the return value
- To fix this error -> lifetimes anotation

## Lifetime Annotation Syntax

- lifetime anotation don't change how long any of the references live.
- rather -> they describe the relationships of the lifetimes of muliple reference to each other without affecting the lifetimes
- just as dunctions can accept any type when the signature specifies a generic type parameter, functions can accept references with any lifetimes by specifing a generic lifetime parameter.
- the names of lifetime must start with an apostrophe `'` and usually all lowercase and very short just like generic types.
- Examples:

```rust
&i32        // a reference
&'a i32     // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime
```

- One lifetime annotation by itself doesn't have much meaning, because the annotations are meant to tell Rust how generic lifetime parameters of multiple references relate to each other
