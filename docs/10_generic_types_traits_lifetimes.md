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
