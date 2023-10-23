# Object-Oriented Programming Features of Rust

- Object-oriented programming (OOP) is a way of modeling programs. Objects as a programmatic concept were introduced in the programming language Simula in the 1960s. Those objects influenced Alan Kay’s programming architecture in which objects pass messages to each other. To describe this architecture, he coined the term object-oriented programming in 1967.

## Encapsulation that Hides Implementation Details

```rust
pub struct AveragedCollection {
  list: Vec<i32>,
  average: f64,
}

impl AveragedCollection {
  pub fn add(&mut self, value: i32) {
    self.list.push(value);
    self.update_average();
  }

  pub remove(&mut self) -> Option<i32> {
    let result = self.list.pop();
    match result {
      Some(value) => {
        self.update_average();
        Some(value)
      }
      None => None,
    }
  }

  pub fn average(&self) -> f64  {
    self.average
  }

  fn update_average(&mut self) {
    let total: i32 = self.list.iter().sum();
    self.average = total as f64 / self.list.len() as f64;
  }
}
```

## Inheritance as a Type System and as Code Sharing

- _Inheritance_ is a mechanism whereby an object can ingerit elements from another object's definition thus gaining the parent object's data and behaviour without you having to define them again.
- Rust does not have a way to to define struct that inherits the parent struct's field and methods implementations without using macro
- in Rust we can do inheritance in a limited way using default implementation of the `summrize` method on the `Summary` trait.
- The other reason to use inheritance relates to the type system: to enable a child type to be used in the same places as the parent type. This is also called polymorphism, which means that you can substitute multiple objects for each other at runtime if they share certain characteristics.

  **Polymorphism**
  To many people, polymorphism is synonymous with inheritance. But it’s actually a more general concept that refers to code that can work with data of multiple types. For inheritance, those types are generally subclasses.
  Rust instead uses generics to abstract over different possible types and trait bounds to impose constraints on what those types must provide. This is sometimes called _bounded parametric polymorphism_.

- The inheritance can make the a program desing less flexible. It also introduce the possiblity of calling methods on subclasses that don't make sense or that cause errors because the methods do not apply to the subclass. For these reasons the Rust takes different approach of using trait objects instead of inheritance.

## Using Trait Objects That Allow for Values of Different Types
