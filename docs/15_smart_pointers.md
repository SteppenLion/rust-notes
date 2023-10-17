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

## Enabliing Recursive Types with Boxes

- a value of _recursie type_ can have another value of the same type a part of it self. Recursive types hae an issue vecause at compile time Rust needs to know how much space a type takes up
- nesting of values of recursive types could theoretically continue infinitely
- because the Boxes have known size we can enable recursive types by inserting a box in the recursive type definition.
- example of recursive type: _cons list_. This data type commonly found in functional programming languages
- this data structure comes from Lisp programming language and is made up of nested pairs (Lisp version of linked list). The name come from `cons` function which is short for construct function -> construct a new pair from its two arguments. By calling `cons`on pair consisting of value and another pair we can construct cons list made up of recurisive pairs
- for example this is pseudocode reprepresentation of cons list containing list 1,2,3 with each pair in parentheses:

```
(1, (2, (3, Nil)))
```

- each item in a cons list contains two elements: the value of current item and the next item. The last item in the list contains only a value called `Nil` without a next item. A cons list is produced by recursively calling the `cons` function.
- The canonical name to denote the base case of the recursion is `Nil`. Note that this not the same as the "null" or "nil" concept, which is invalid or absent value

### Using `Box<T>` to Get Recursive Type with a Know Size

- Because Rust can't figure out how much space to allocate for recursively defined types, the compiler gives an error with this helpful suggestion:

```
help: insert some indirection (e.g., a `Box`, `Rc`, or `&`) to make `List` representable
  |
2 |     Cons(i32, Box<List>),
  |               ++++    +
```

- In this suggestion, “indirection” means that instead of storing a value directly, we should change the data structure to store the value indirectly by storing a pointer to the value instead.

```rust
enum List {
  Cons(i32,Box<List>),
  Nil,
}

use create::List::{Cons,Nil};

fn main() {
  let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
```

- Boxes provide only the indirection and heap allocation; they don't have any other special capabilities, like those we'll see with the other smart pointer types.
- The `Box<T>` type is a smart pointer because it implements the `Deref` trait, which allows `Box<T>` values to be treated like references. When a `Box<T>` value goes out of scope, the heap data that the box is pointing to is cleaned up as well because of the `Drop` trait implementation.

## Treating Smart Pointers Like Regular References with the `Deref` Trait

- implementing the `Deref` trait allows you to customize the behavior of the _dereference operator_ `*`
- implementing in such a way that a smart pointer can be treated like a regular reference, we can write code that operates on references and use that code with smart pointers too.

## Following the Pointer to the Value

- regular reference is a type of pointer and one way to think of a pointer is as an arrow to a value stored somewhere else

```rust
fn main() {
  let x = 5;
  let y = &x;

  assert_eq!(5, x);
  assert_eq!(5, *y);
}
```

- comparing a number and a reference to a number is not allowed because they are different types. We must use the dereference operator to follow the reference to the value it is pointing to.

## Using `Box<T>` Like a Reference

- we can rewrite reference to use the `Box<T>` instead:

```rust
fn main() {
  let x = 5;
  let y = Box::new(x);

  assert_eq!(5,x);
  assert_eq!(5, *y);
}
```

- main difference between two examples is that here we set `y` to be an instance of `Box<T>` pointing to copied value of `x` rather than a reference pointing to the value of `x`.

## Defining Our Own Smart Pointer

```rust
struct MyBox<T>(T);

impl<T> MyBox<T> {
  fn new(x: T) -> MyBox<T> {
    MyBox(x)
  }
}

fn main() {
  let x = 5;
  let y = MyBox::new(x);

  assert_eq!(5,x);
  assert_eq!(5, *y);
}
```

- it won't compile because Rust doesn't know how to dereference MyBox
- Our `MyBox<T>` type can't be dereferenced because we haven't implemented that ability on our type. To enable dereferencing with the `*` operator, we implement the `Deref` trait.

## Treating a Type Like a Reference by Implementing the `Deref` Trait

- the `Deref` trait requires to implement one method `deref` that borrows `self` and returns a reference to the inner data.

```rust
use std::ops::Deref;

impl<T> Deref for MyBox<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}
```

- the `type Target = T;` defines an assosiated type for the `Deref` trait to use.
- Assosiated types are a slightly different way of declaring a generic parameter
- `deref` method with `&self.0` will return a reference to the value we want to access with the `*` operator (`.)` will access the first value in tuple struct)
- without the `Deref` trait the compiler can only dereference `&` references. The `deref` method gives the compiler the ability to take a value of any type that implements `Deref` trait and call the `deref` method to get `&` reference that it knows how to dereference
- the Rust will actually run this code when we call the `*y` -> `*(y.deref())`
- the reason the `deref` method returns a reference to a value and that the plain dereference outside the parentheses in `*(y.deref())` is still necessary is to do with the ownership system. If the `deref` method returned the value directly instead of a reference to the value the value would be moved out of `self`

## Implicit `Deref` Coercions wiht Functions and Methods

- _Deref coercion_ converts a reference to a type that implements the `Deref` trait into reference to another type. E.g. deref coercion can convert `&String` to `&str` because both implements `Deref` trait. It happens automatically when we pass a reference to a particular type's in th function or method definition.
- When the `Deref` trait is defined for the types involved, Rust will analyze the types and use `Deref::deref` as many times as necessary to get a reference to match the parameter's type. The number of times that the `Deref::dered` needs to be inserted is resolved at compile time, so there is no runtime penalty

## How `Deref` Coercion Interacts with Mutability

- similarly to the `Deref` trait to override `*` operator on immutable references, we can use the `DerefMut` trait to override `*` on mutable references
- Rust deref coercion when it finds types and trait implementations in 3 cases:

  - From `&T` to `&U` when `T: Deref<Target=U>`
  - From `&mut T` to `&mut U` when `T: DerefMut<Target=U>`
  - From `&mut T` to `&U` when `T Deref<Target=U>`

- third case: Rust can also coerce a mutable reference to an immutable one. but reverse it is not possible because the borrowing rules.

## Running Code on Cleanup with `Drop` Trait

- the smart pointer pattern is `Drop`, which lets you customize what happens when a value is about to go out of scope.
- In Rust, you can specify that a particular bit of code be run whenever a value goes out of scope, and the compiler will insert this code automatically. As a result, you don't need to be careful about placing cleanup code everywhere in a program that an instance of a particular type is finished with—you still won't leak resources!
- the `Drop` trait requires you to implement one method named `drop` that takes a mutable reference to `self`

```rust
struct CustomerSmartPointer {
  data: String,
}

impl Drop for CustomerSmartPointer {
  fn drop(&mut self) {
    println!("Droping CustomerSmartPointer with data `{}`!",self.data`);
  }
}

fn main() {
  let c = CustomerSmartPointer {
    data: String::from("some stuff"),
  };
  let d = CustomerSmartPointer {
    data: String::from("other stuff"),
  };

  println!("CustomerSmartPointer created");
}
```

- The Rust automatically called `drop` for us when our instances went out of scope, calling the code we specified. Variables are dropped in the reverse order of their creation so `d` was dropped before `c`

## Dropping a Value Early with `std::mem::drop`

- if we want to drop some values early (force) for example when using smart pointers that manage locks, so we can releases the lock so that other code in the same scope can acquire the lock. Rust does not let you call the `Drop` trait `drop` method manually. Instead we have to call the `std::mem::drop` function

```rust
fn main() {
  let c = CustomerSmartPointer {
    data: String::from("some data"),
  };
  println!("CustomerSmartPointer created");
  drop(c);
  println!("CustomerSmartPointer dropped the end of main.");
}
```

- the `drop` function is different from the `drop` method in the `Drop` trait.
- we can use code specified in the `Drop` trait implementation in many ways to make cleanup convenient and safe. For instance, we can use it to create our own memory allocator

## `Rc<T>`, the Reference Counted Smart Pointer

- in some cases we want to have single value owned by multiple owners. We need to explicitly use Rust type `Rc<T>` which is abbrevation for _reference counting_. The `Rc<T>` type keeps track of the number of references to a value to determine whether or not the value is still in use. If there is zero references to a value the calue can be cleaned up without any references becoming invalid
- we use the `Rc<T>` type when we want to allocate some data on the heap for multiple parts of our program to read and we can not determine at compile time which part will finish using the data last. If we know which part would finish last, we could just make that part the data's owner
- **NOTE**: the `Rc<T>` is only for use in single-threaded scenerios

## Using `Rc<T>` to Share Data

```rust
enum List {
  Cons(i32, Rc<List>),
  Nil,
}
use create::List::{Cons,Nil};
use std::rc::Rc;

fn main() {
  let a = Rc::new(Cons(5, Rc::new(Cons(10,Rc::new(Nil)))));
  let b = Cons(3, Rc::clone(&a));
  let c = Cons(4,Rc::clone(&a));
}
```

- we need to add a `use` statement to bring the `Rc<T>` into scope because it is not in prelude. We can use `a.clone()` but the Rust convention is to use `Rc::clone` in this case. The implementaion of `Rc::clone` does not make deep copy of all data like most types' implementation of `clone` do. The call `Rc::clone` only increments the reference count which does not take much time

## Cloning an `Rc<T>` Increases the Reference Count

```rust

fn main() {
  let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
  println!("count after creating a = {}", Rc::strong_count(&a));
  let b = Cons(3, Rc::clone(&a));
  println!("count after creating b = {}", Rc::strong_count(&a));
  {
    let c = Cons(4, Rc::clone(&a));
    println!("count after creating c = {}", Rc::strong_count(&a));
  }
  println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
```

```text
$ cargo run
   Compiling cons-list v0.1.0 (file:///projects/cons-list)
    Finished dev [unoptimized + debuginfo] target(s) in 0.45s
     Running `target/debug/cons-list`
count after creating a = 1
count after creating b = 2
count after creating c = 3
count after c goes out of scope = 2
```

## `RefCell<T>` and the Interior Mutability Pattern

- _Interior mutability_ is design pattern in Rust that allows you to mutate data even when there are immutable references to that data, the pattern uses `unsafe` code inside a data structure to bend Rust's usual rules that govern mutation and borrwing
- Unsafe code indicates to the compiler that we are checking the rules manually instead of relying on the compiler to check them for

## Enforcing Borrowing Rules at Runtime wiht `RefCell<T>`

- unlike `Rc<T>` the `RefCell<T>` type represents single ownership over the data it holds. What is the difference between `RefCell<T>` and `Box<T>`: recall from the Chapter 4:
  - At any given time you can have ether (bot not both) one mutable reference or any number of immutable references
  - References must always be valid
- With references and `Box<T>` the borrowing rules invariants are enforced at compile time. With `RefCell<T>` these invariants are enforced at runtime. If we break the rules our program with not get compile error but instead it will panic and exit.
- advantages to checking the borrowing rules at compile time are that errors will be caught sooner in the development process.
- advantages to checking the borrowing rules at runtime is that certain memory-safe scenarios are then allowed where they would have been disallowed by the compile-time checks. Static analysis like the Rust compiler is inherently conservative. Some properties of code are impossible to detect by analyzing the code: most famous is the Halting Problem.
- similar to `Rc<T>` the `RefCell<T>` is only use in single-threaded scenarios

## A Use Case for Interior Mutability: Mock Objects

- sometimes during testing a programmer will sue a type in place of another type, in order to observe particular behavior and assert it's implemented correctly. This placholder type is called a _test double_. _Mock objects_ are specific types of test doubles that record what happens during a test so you can assert that the correct actions took place. Rust does not have Mock object in standard library but we can create struct that will serve same purposes as mock object.

```rust
pub trait Messanger {
  fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messanger> {
  messanger: &'a T,
  value: usize,
  max: usize,
}

impl<'a, T> LimitTracker<'a,T>
where
  T: Messanger,
  {
    pub fn new(messanger: &'a T, max: usize) -> LimitTracker<'a,T> {
      LimitTracker {
        messanger,
        value: 0,
        max,
      }
    }

    pub fn set_value(&mut self,value: usize) {
      self.value = value;

      let percentage_of_max = self.value as f64 / self.max as f64;

      if percentage_of_max >= 1.0 {
        self.messanger.send("Error: You are over your qouta!");
      } else if percentage_of_max >= 0.9 {
        self.messanger.send("Urgent warning: you have used up over 90% of your qouta!");
      } else if percentage_of_max >= 0.75 {
        self.messanger.send("Warrning : you have used up over 75% of your qouta!");
      }
    }
  }
```

## Keeping Track of Borrows at Runtime with `RefCell<T>`

- when creating immutable and mutable references we use `&` and `&mut` syntax
- with `RefCell<T>` we use the `borrow` and `borrow_mut` methods which are part of safe API tha belnogs to `RefCell<T>`. The `borrow` method returns the smart pointer type `Ref<T>` and `borrow_mut` returns the smart pointer type `RefMut<T>`. Both types implement `Deref`
- The `RefCell<T>` keeps track od how many `Ref<T>` and `RefMut<T>` smart pointers are currently active. Each time we call `borrow` the count is increases how many immutable borrows are active. When a `Ref<T>` value goes out of scope the count of immutable borrows goes down by one. Just like compile time borrowing rules, `RefCell<T>` lets us have many immutable borrows or one mutable borrow at any point in time.
- if we violate these rules we will get panic at runtime (not compile error)

## Having Multiple Owners of Mutable Data by Combining `Rc<T>` and `RefCell<T>`

- common way to use `RefCell<T>` is in combination with `Rc<T>`. If we have `Rc<T>` that holds a `RefCell<T>` you can get value that can have multiple owneres and you can mutate

```rust
#[derive(Debug)]
enum List {
  Cons(Rc<RefCell<i32>>,Rc<List>)
}

use create::List::{Cons,Nil};
use std::cell::RefCell;
use std::rc::Rc:

fn main() {
  let value = Rc::new(RefCell::new(5));

  let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
  let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
  let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

  *value.borrow_mut() += 10;

   println!("a after = {:?}", a);
   println!("b after = {:?}", b);
   println!("c after = {:?}", c);
}
```

- This technique is pretty neat! By using `RefCell<T>`, we have an outwardly immutable List value. But we can use the methods on `RefCell<T>` that provide access to its interior mutability so we can modify our data when we need to.
- **Note** that `RefCell<T>` does not work for multithreaded code!
- `Mutex<T>` is the thread-safe version of `RefCell<T>`

## Reference Cycles Can Leak Memory

- We can see that Rust allows memory leaks by using `Rc<T>` and `RefCell<T>`: it’s possible to create references where items refer to each other in a cycle.
- this will create memory leaks, because count of each item in the cycle will never reach 0, and the values will never be dropped

## Creating a Reference Cycle

```rust
use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
  Cons(i32, RefCell<Rc<List>>),
  Nil,
}

impl List {
  fn tail(&self) -> Option<&RefCell<Rc<List>>> {
    match self {
      Cons(_, item) => Some(item),
      Nil => None,
    }
  }
}

fn main() {
  let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
  println!("a initial rc count = {}", Rc::strong_count(&a));
  println!("a next item = {:?}", a.tail());
  let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
  println!("a rc count after b creation = {}", Rc::strong_count(&a));
  println!("b initial rc count = {}", Rc::strong_count(&b));
  println!("b next item = {:?}", b.tail());
  if let Some(link) = a.tail() {
      *link.borrow_mut() = Rc::clone(&b);
  }
  println!("b rc count after changing a = {}", Rc::strong_count(&b));
  println!("a rc count after changing a = {}", Rc::strong_count(&a));
  // Uncomment the next line to see that we have a cycle;
  // it will overflow the stack
  // println!("a next item = {:?}", a.tail());
}
```

- If we uncomment the last println! and run the program, Rust will try to print this cycle with `a` pointing to `b` pointing to `a` and so forth until it overflows the stack.
- one of the solution for avoiding reference cycles is reorganizing data structure so that some reference express ownership and some reference don't. As result we can have cycles made up of some ownership relationships and some non-ownership relationships and only the ownership relationships affects whether or not a value can be dropped.

## Preventing Reference Cycles: Turning an `Rc<T>` into a `Weak<T>`

- Strong references are how we can share ownership of an `Rc<T>` instance. Weak references do not express an ownership relationship, and their count does not affect when an `Rc<T>` instance is cleaned up. They won't cause a reference cycle because any cycle involving some weak references will be broken once the strong reference count of values involed is 0
- because the value that `Weak<T>` refer might have been dropped, to do anything with the value that a `Weak<T>` is pointing to, we must make sure the value still exist. To do this we can call `upgrade`method on a `Weak<T>` instance, which will return an `Option<Rc<T>>`. We will get rseutl of `Some` if the `Rc<T>` value has not been dropped yet and result of `None` if it has been dropped

## Creating a Tree Data Structure: a Node with Child Nodes

```rust
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Node {
  value: i32,
  children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
  let leaf = Rc::new(Node {
    value: 3,
    children: RefCell::new(vec![]),
  });
  let branch = Rc::new(Node {
    value: 5,
    children: RefCell::new(vec![Rc::clone(&leaf)])
  });
}
```

- we clone the `Rc<Node>` in `leaf` and store that in branch meaning the `Node` in `leaf` now has two owneres: `leaf` and `branch`. We can get from `branch` to `leaf` through `branch.children` but there is no way to get from `leaf` to `branch`. The reason is that `leaf` has no reference to `branch` and does not know they are related.

## Adding a Reference from Child to Its Parent

- Thinking about the relationships another way, a parent node should own its children: if a parent node is dropped, its child nodes should be dropped as well. However, a child should not own its parent: if we drop a child node, the parent should still exist. This is a case for weak references!
- so instead of `Rc<T>` we will make the type of `parent` use `Weak<T>` specifically a `Ref<Weak<Node>>`

```rust
use std::cell::RefCell;
use std::rc::{Rc,Weak};

#[derive(Debug)]
struct Node {
  value: i32,
  parent: RefCell<Weak<Node>>,
  children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
  let leaf = Rc::new(Node {
    value: 3,
    parent: RefCell::new(Weak::new()),
    children: RefCell::new(vec![]),
  });

  println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

  let branch = Rc::new(Node {
      value: 5,
      parent: RefCell::new(Weak::new()),
      children: RefCell::new(vec![Rc::clone(&leaf)]),
  });

  *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

  println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}
```

## Visualizing Changes to `strong_count` and `weak_count`

```rust
fn main() {
  let leaf = Rc::new(Node {
    value: 3,
    parent: RefCell::new(Weak::new()),
    children: RefCell::new(vec![]),
  });

  println!(
    "leaf strong = {}, weak = {}",
    Rc::strong_count(&leaf),
    Rc::weak_count(&leaf),
  );

  {
    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
    println!(
        "branch strong = {}, weak = {}",
        Rc::strong_count(&branch),
        Rc::weak_count(&branch),
    );
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
  }

  println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
  println!(
      "leaf strong = {}, weak = {}",
      Rc::strong_count(&leaf),
      Rc::weak_count(&leaf),
  );
}
```
