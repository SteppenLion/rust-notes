# Common Rust Programing Concepts

## Variables and Mutability

- variables are **immutable** by default. Once bound to a name, you can't change that value.
- by adding the `mut` we can change the mutability of variable.
- immutable variables -> _constants_ are values that are bound to a name and are not allowed to change. You can't change mutability with `mut` with the constants. Constants are valid for the entire time a program runs, within scope they were declared in.

### Shadowing

- we can shadow the variable with same name. E.g:

```rust
fn main() {
  let x = 5;
  let x = x + 1;
  {
    let x = x * 2;
    println!("The value of x in the inner scope is: {x}");
  }

  println!("The value of x is : {x}");
}
```

- Shadowing is different from making variable as `mut` becouse we'll ger a compile-time error if we accidentaly try to reassign to this variable without using the `let` keyword
- other difference is that becouse we are creating a new variable when we use the `let` keyword again -> we can change the type of the value but reuse the same name.

```rust
fn main() {
    let spaces = "   ";
    let spaces = spaces.len();
}
```

## Data Types

- rust is a _statically typed language_ => you must know the types of all variables at compile time.
- in case when many types are possible, e.g. converted _String_ to a numeric type using `parse` we must add a type annotation, like this:

```rust
#![allow(unused)]
fn main () {
  let guess: u32 = "42".parse().expect("Not a numnber");
}
```

## Scalar Types

- scalar types represents a single value. Rust has four primary scalar types: integers, floating-point, numbers, Boolean, and charakters.

### Integer Types

- An _interger_ is a number without a fractional component. Variants of integer value:

| Length  | Signed  | Unsigned |
| ------- | ------- | -------- |
| 8-bit   | `i8`    | `u8`     |
| 16-bit  | `i16`   | `u16`    |
| 32-bit  | `i32`   | `u32`    |
| 64-bit  | `i64`   | `u64`    |
| 128-bit | `i12`   | `u128`   |
| arch    | `isize` | `usize`  |

- Signed and unsigned refers to wheter it's possible for number to be negative (only possitive -> unsigned).
- Signed integers are stored using two's component representation.
- `isize` and `usize` types depend on the architecture of the computer your program is running on.
- integer literals can by writes in any forms shown in below table. Number literals can be write with sufix -> `5tu8` and slo can use underscore as visual separator => 1_000 is 1000

| Number literals  | Example     |
| ---------------- | ----------- |
| Decimal          | 98_222      |
| Hex              | 0xff        |
| Octal            | 0o77        |
| Binary           | 0b1111_0000 |
| Byte (`u8` only) | b'A'        |

## Floating-point types

- floating-point types are `f32` and `f64`
- floating-point numbers are represented according to the IEEE-754 standard. `f32` type is a single precision float and `f64` has double precision.

## Boolean type

- only `true` or `false` and you can specified e.g : `let f: bool = false`

## Charakter type

- `char` type is the language's most primitive alphabetic type. E.G:

```rust
fn main() {
  let c= 'z';
  let z: char = 'ß';
  let heart_eyed_cat = '😻';
}
```

- note that char types are specified with single quotes as opposed to sting literals which use double quotes.
- Rust `char` type is four bytes in size and represents a Unicode Scalar Value

## Compound Types

- Rust can group multiple values into one type. Rust has two primitive compound types: tuples and arrays

### Tuple Type

- tuple is general way of grouping together a number of values with variaty of types into one compound type.
- tuples have a fixed length: once declared they can not grow or shrink in size.

```rust
fn main() {
  let tup: (i32,f64,u8) = (500,6.3, 1);
}
```

- variable tup binds to the entire tuple, becouse a tuple is considered a single compound element
- to destracture tuple we can write something like this:

```rust
fn main() {
  let tup = (32,2.2,2);
  let (x,y,z) = tup;
  println!("The value of y is: {y}");
}
```

- we can also access a tuple element directly by using a period (`.`)

```rust
fn main(){
  let x: (i32, f64, u8) = (500, 6.4, 1);
  let five_hundred = x.0;
  let six_point_four = x.1;
  let one = x.2;
}
```

### Array Type

- useful when you want to ensure you always have a fixed number of elements. Array isn't flexible as vektor type.
- use when you know the number of elements will not need to change.

```rust
// simple array
fn main() {
    let a = [1, 2, 3, 4, 5];
}
// it will always contains 12 elements
#![allow(unused)]
fn main() {
  let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
}
// specified the type of each element, a semicolon, and then the number of elements
#![allow(unused)]
fn main() {
let a: [i32; 5] = [1, 2, 3, 4, 5];
}
// also initialize an array to contain the same value for each element by specifying
// the initial value
#![allow(unused)]
fn main() {
let a = [3; 5];
}
// same as let a = [3, 3, 3, 3, 3];

// accessing the array elements
fn main() {
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
}
```

## Functions

- Rust language uses _snake case_ as conventional style for functions and variable names in which all letters are lowercase and underscores separated words

```rust
fn main(){
  println!("HELLO WORLD");
  another_function();
}
another_function() {
  println!("Another function");
}
```

- you can define different functions before or after main function, Rust doesn't care, but it must be declared somewhere if it is initialize

## Statements and Expresions

- funcions definitions are also statements. Statements do not return values. Therefor you can't assign a `let` statement to another variable as the following code :

```rust
// ERROR
fn main() {
    let x = (let y = 6);
}
```
