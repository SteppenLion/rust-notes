# Enums and Patern Matching

- _enums_ a.k.a _enumerations_ allow us to define a type by enumerating its possible variants
- patern matching using `match` expresion which makes easy to run different code for different values of an enum.

## Defining an Enum

- Enums give us a way of saying a value is one of possible set of values.
- For example we may want to say that `Rectangle` is one of set possible shapes that also includes `Circle` and `Triangle`
- e.g IP addreses are either version 4 or 6 but not bouth at the same time

```rust
enum IpAddrKind {
  V4,
  V6,
}
fn main() {
  let four = IpAddrKind::V4;
  let six = IpAddrKind::V6;

  route(IpAddrKind::V4);
  route(IpAddrKind::V6);
}

fn route(ip_kind: IpAddrKind) {}
```

- IpAddrKind is now custom data type that we can use elsewhere in our code.

## Enum Values

- we can store the actual IP address data, right now we only know what kind it is.
- by using struct e.g.

```rust
fn main() {
    enum IpAddrKind {
        V4,
        V6,
    }
    struct IpAddr {
        kind: IpAddrKind,
        address : String,
    }

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
}
```

- but using only the enum is more consise ->

```rust
fn main() {
    enum IpAddr {
        V4(String),
        V6(String),
    }
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));
}

```

- we attach data to each variant of the enum directly, no need extra struct.
- another advantage is using enums is that each variant can have different types and amounts of associated data.
- IPV4 will always have four numeric components (u8) and if we want IPV6 be still represented as String -> this we wouldn't be able to with struct.

```rust
fn main() {
    enum IpAddr {
        V4(u8,u8,u8,u8),
        V6(String),
    }

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
}
```

- as storing ip adresses are common we can use standard library

```rust
#![allow(unused)]
fn main() {
struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
}

```

- another example with wide variety of types embedded in its variants:

```rust
enum Message {
    Quit,
    Move {x:i32, y:i32},
    Write(String),
    ChangeColor(i32,i32,i32),
}
fn main() {}
```

- Defining an enum with variants such as above is similar ot defining different kinds of struct definitions
- The following struct could hold the same data as proceding enum variant hold:

```rust
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct

fn main() {}
```

- as with stuct we can define methods on enums:

```rust
fn main() {
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    impl Message {
        fn call(&self) {
            // method body would be defined here
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();
}
```

## The Option Enum and Its Advantages Over Null Values

- `Option` is another enum define by the Standard library. `Option` type encodes the very common scenarion in which a value could be something or it could be nothing.
- Rust doesn't have null feature. _Null_ is the value that means that is no value there.
- as Rust don't have nulls but it does have an enum that can encode he concept of value being present or absent
- The enum `Option<T>` and it is defined by [standard library](https://doc.rust-lang.org/std/option/enum.Option.html):

```rust
#![allow(unused)]
fn main() {
    enum Option<T> {
        None,
        Some(T),
    }
}
```

- we can use `Some` and `None` variants directly without the `Option::` preffix.
- `<T>` can hold one piece of data of any type -> generic

```rust
fn main() {
    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;
}
```

- with absent_number we need to annoted the type becouse the compiler can't infer the type that the corresponding `Some` variant will hold by looking at `None` value.
- becouse `Option<T>` and `T` are differnt types, the compiler won't let us use an `Option<T>` value as if were definitely a valid value. For example this code won't compile becouse it's trying to add `Option<T>` and `i8`

```rust
fn main() {
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y;
}
// $ cargo run
//    Compiling enums v0.1.0 (file:///projects/enums)
// error[E0277]: cannot add `Option<i8>` to `i8`
//  --> src/main.rs:5:17
//   |
// 5 |     let sum = x + y;
//   |                 ^ no implementation for `i8 + Option<i8>`
//   |
//   = help: the trait `Add<Option<i8>>` is not implemented for `i8`
//   = help: the following other types implement trait `Add<Rhs>`:
//             <&'a f32 as Add<f32>>
//             <&'a f64 as Add<f64>>
//             <&'a i128 as Add<i128>>
//             <&'a i16 as Add<i16>>
//             <&'a i32 as Add<i32>>
//             <&'a i64 as Add<i64>>
//             <&'a i8 as Add<i8>>
//             <&'a isize as Add<isize>>
//           and 48 others
//
// For more information about this error, try `rustc --explain E0277`.
// error: could not compile `enums` due to previous error
```

- we need to convert `Option<T>` to `T` before we can perform operations with it. This helps catch error: assuming that something isn't null when in fact it is.

## The `match` Control Flow Construct

- Rust has control flow construct called `match` that allows you to compare a value against a series of patterns and then execute code based on which pattern matches.
- Example:

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin)-> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {}
```

- it is very similar to `if` expresion but instead of Boollean value it returns any type.

## Patterns that Binds to Values

- useful feature of match arms is that they can bind to the parts og the values that match the pattern.
- This is how we can extract values out of enum variants

```rust
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn main() {
    value_in_cents(Coin::Quarter(UsState::Alaska));
}
```

## Matching with `Option<T>`

```rust
fn main() {
    fn plus_one(x:Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}
```

- if we don't handle the None case The rust will display error to extend match pattern to catch and None value.

## Catch-all Patterns and the `_` Placeholder

- we can use catch-all patern `_`, which is special pattern that matches any value and does not bind to that value.
- This tells Rust we aren't going to use the value, so Rust won't warn us about an usnued variable

```rust
fn main() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        5 => remove_fancy_hat(),
        _ => reroll(),
        // or - => () // () unit value (empty tuple type) to express that nothing will happend
    }
    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn reroll() {}
}
```

## Consise Control Flow wiht `if let`

```rust
fn main() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (), // don't do anything with None variant
    }
}
```

- instead of above code we can write:

```rust
fn main() {
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
}
```

- `if let` syntax works same as `match`. Downside of less code is that we loose exhausive checking that `match` enforces.
- we can add also `else` clouse for example changing the Coins program:

```rust
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let coin = Coin::Penny;
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}
```
