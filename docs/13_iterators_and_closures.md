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
