# Using Structs to Structure Releated Data

- _struct_ or _structure_ is custom data type that lets you package together and name multiple related values that make up meaningful group.

## Defining and Instantiating Structs

- Structs are similar to tuples, they hold multiple related values.
- Defining struct:

```rust
struct User {
  active: bool,
  username: String,
  email: String,
  sign_in_count: u64,
}

fn main() {
  let user1 = User {
      email: String::from("someone@example.com"),
      username: String::from("someusername123"),
      active: true,
      sign_in_count: 1,
    };
}
```

- to get specific value from the a struct we use dot notation. If the instance is mutable we can change a value by using fot notation into a particular field

```rust
fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@example.com");
}
```

- NOTE: entire instance must be mutable; Rust does not allow us to mark only certain fields as mutable.

```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    let user1 = build_user(
        String::from("someone@example.com"),
        String::from("someusername123"),
    );
}
```

## Using the Field Init Shorthand

```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        email, // shorthand
        username, // shorthand
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    let user1 = build_user(
        String::from("someone@example.com"),
        String::from("someusername123"),
    );
}
```

## Creating Instance From Other Instances With Struct Update Syntax

- it useful to create a new instance of a struct that includes most of the values from another instance, but change some. -> _struct update syntax_

```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    // --snip--

    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
}
```

- The `..user1` must come last to specify that any remaining fields should get their values from the corresponding fields in `user1`, but we can choose to specify values for as many fields as we want in any order, regardless of the order of the fields in the struct’s definition.

## Using Tuple Structs without Named Fields to Create Different Types

- Rust support structs that look similar to tuples, called _tuple structs_
- they don't have the names associated with their fields, just they have the types of the fields
- useful when you want to give the whole tuple a name and make the tuple from other tuples

```rust
struct Color(i32,i32,i32);
struct Point(i32,i32,i32);

fn main() {
    let black = Color(0,0,0);
    let origin = Point(0,0,0);
}
```

## Unit-like Structs Without Any Fields

- we can also define structs that don't have any fields -> called _unit-like structs_
- Unit-like structs can be useful when you need to implement a trait on some type but don't have any data that you want to store in the type itself

```rust
struct AlwaysEqual;

fn main(){
    let subject = AlwaysEqual;
}
```

## Method Syntax

- _Methods_ are similar to functions -> declare them with `fn` keyword and name.
- they can have parameters and return value and they have some code that's run when the method is called from somewhere else.
- unlike functions, methods are defined within the context of `struct` (or `enum` or `trait` object)
- their first parameter is always `self` which reperesents of the strust the method is being called on

### Defining methods

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(&self) {
        self.width * self.height
    }
}
fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("The area of the rectangle is: {}", rect1.area());
}
```

- in the signature for `area` we use `&self` instead of `rectangle: &Rectangle`.
- the `&self` is short for `self: &Self`. Within `impl` block `Self` is an alais for the type that the `impl` block is for.
- method is borrows the `Self` instance

## Methods with More Parameters

- in this example we want an instance of Rectangle to take another instance of Rectangle and return true if second Rectangle can fit completly within self, otherwise it should return false

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
```

## Associated Funcions

- all functions defined within an `impl` block are called _associated function_ becouse they're associated with the type named after `impl`.
- we can define functions that don't have `self` as their first parameter (and thus are not methods) because they don;t need an instance of the type to work with. We are already use this function like: `String::from` function that's defined on the `String` type.
- Associated functions that aren't methods are often used for constructors that will return a new instance of the struct.
- these are often called `new`, but `new` isn't a special name and isn't built into the language.
- e.g. associated function named `square` that would have one dimension parameter and use that as bouth width and height , thus it easier to create a square `Rectangle` rather than having to specify the same value twice.

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width:size,
            height:size,
        }
    }
}
fn main() {
    let sq = Rectangle::square(3)
}
```

## Multiple `impl` Blocks

- each struct is allowed to hace multiple `impl` blocks.
- e.g. :

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

```
