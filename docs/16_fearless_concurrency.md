# Fearless Concurrency

_Concurrent programming_, where different parts of a program execute independently, and _parallel programming_, where different parts of a program execute at the same time, are becoming increasingly important as more computers take advantage of their multiple processors.

- ownership and type system are a powerful set of tools to help manage memory safty and cocurrency problems. By leveraging ownership and type checking many concurrency errors are compile-time errors in Rust rather than runtime errors. This aspect of Rust has been nicknamed _fearless concurrency_. Fearless concurrency allows us to write code that is free of subtle bugs and is easy to refactor without introducing new bugs

## Using Threads to Run Code Simultaneosly

- in most current operating systems, an executed program's code is run in a _process_, and the oprerating systems will manage multiple processes at once. Within program, we can have also independent parts that run sumultaneosly.
- The features that run these independent parts are called _threads_
- e.g.: webserver can have multiple threads so that it could respond to more than one request at same time.
- Simultaneosly run programs can lead to problems such as:
  - Race conditions, where threads are accessing data or resources in inconsistend
  - Deadlocks, where two threads are waiting for each other, preventing both threads from continuing
  - Bugs that happen only in certain situations and are hard to reproduce and fix reliably
- The Rust standard library uses a 1:1 model of thread implementation, whereby a program uses one operating system thread per one language thread

## Creating a New Thread with `spawn`

```rust
use std::thread;
use std::time::Duration;

fn main() {
  thread::spawn(|| {
    for i in 1..10 {
      println!("hi number {} from the spawned thread!", i);
      thread::sleep(Duration::from_millis(1));
    }
  });

  for i in 1..5 {
    println!("hi number {} from main thread!", i);
    thread::sleep(Duration::from_millis(1));
  }
}
```

## Waiting for All Threads to Finish Using `join` Handles

- the above code only stops the spawned thread prematurely most of the time due to the manin thread ending, but because there is no guarantee on the order in which threads run, we also can not guarantee that the spawned thread will get to run at all

```rust
use std::thread;
use std::time::Duration;

fn main() {
  let handle = thread::spawn(|| {
    for i in 1..10 {
      println!("hi number {} form the spawned thread!",i);
      thread::sleep(Duration::from_millis(1));
    }
  });

  for in in 1..5 {
    println!("hi number {} from main thread!",i);
    thread::sleep(Duration::from_millis(1));
  }
  handle.join().unwrap();
}
```

- calling `join` on the handle blocks the thread currently running until the thread represented by the handle terminates. _Blocking_ a thread means that thread is prevented from perfoming work or exiting. Because we put the call `join` after the main thread's for loop there will be alternating when the main and spawned thread is running. But until the end the main thread is waiting because the call to `handle.join()` does not end until the spawned thread finished.
- If we put the `handle.join()` after the for loop and not the the end of main thread -> the main thread will wait for the spawned thread to finish and then run its `for` loop the output won't be interleved anymore.

## Using `move` Closures with Threads

- often it is use `move` keyword with closures passed to `thread::spawn` because the closure will then take ownership of that value it uses from the enviroment, thus transfering ownership of those values from thread to another. (see Chapter 13).

```rust
use std::thread;

fn main() {
  let v = vec![1,3,4];

  let handle = thread::spawn(move|| {
    println!("Here is a vector: {:?}", v);
  })

  handle.join().unwrap();
}
```

- we need to capture the `v` becouse the `println!` is running in closure. By adding the `move` keyword we force the the closure to take ownership of the values

## Using Message Passing to Transfer Data Between Threads

- one increasingly popular approach to ensuring safe concurrency is _message passing_ where threads or actors communicate by sending each other messages containing data. The idea from the Go language: "Do not communicate by sharing memory, instead share memory by communicating."
- to accomplish message-sending concurrency, Rust standard library provides an implementation of _channels_. A channel is a general programming concept by which data is sent from one thread to another
- a channel has tow halves: a transmiter and a receiver. The transmiter half is the upstream location where we put data and the receiver half is where the data ends up downstream. A channel is said to be _closed_ if eiher the transmiter or receiver half is dropped.

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
  let (tx,rx) = mpsc::channel(); // creating a channel
  // transmiter
  thread::spawn(move || {
    let val = String::from("hi");
    tx.send(val).unwrap();
  });
  // receiver
  let received = rx.recv().unwrap();
  println!("Got : {received}");
}
```

- `mpsc` stands for _multiple producer,single comsumer_ . The `mpsc::channel` function returns a tuple. We use `let` statment to destructures the tuples.

## Channels and Ownership Transference

**Compile Error**:

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
  let (tx, rx) = mpsc::channel();

  thread::spawn(move || {
    let val = String::from("hi");
    tx.send(val).unwrap();
    println!("val is {}", val);
  });

  let received = rx.recv().unwrap();
  println!("Got: {}", received);
}
```

```text
cargo run
   Compiling message-passing v0.1.0 (file:///projects/message-passing)
error[E0382]: borrow of moved value: `val`
  --> src/main.rs:10:31
let val = String::from("hi");
   |             --- move occurs because `val` has type `String`, which does not implement the `Copy` trait
9  |         tx.send(val).unwrap();
   |                 --- value moved here
10 |         println!("val is {}", val);
   |                               ^^^ value borrowed here after move
```

- The `send` function takes ownership of its parameter, and when the value is moved, the receiver takes ownership of it. This stops us from accidentally using the value again after sending it; the ownership system checks that everything is okay.

## Sending Multiple Values and Seeing the Receiver Waiting

```rust
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
  let (tx, rx) = mpsc::channel();

  thread::spawn(move || {
    let vals = vec![
      String::from("hi"),
      String::from("from"),
      String::from("the"),
      String::from("thread"),
    ];

    for val in vals {
      tx.send(val).unwrap();
      thread::sleep(Duration::from_secs(1));
      }
  });

  for received in rx {
    println!("Got: {}", received);
  }
}
```

- Because we don't have any code that pauses or delays in the `for` loop in the main thread, we can tell that the main thread is waiting to receive values from the spawned thread.

## Creating Multiple Producers by Cloning the Transmitter

```rust
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
  let (tx, rx) = mpsc::channel();
  let tx1 = tx.clone();

  thread::spawn(move || {
    let vals = vec![
      String::from("hi"),
      String::from("from"),
      String::from("the"),
      String::from("thread"),
    ];

    for val in vals {
      tx1.send(val).unwrap();
      thread::sleep(Duration::from_secs(1));
      }
  });

  thread::spawn(move || {
    let vals = vec![
      String::from("more"),
      String::from("messages"),
      String::from("for"),
      String::from("you"),
    ];

    for val in vals {
      tx.send(val).unwrap();
      thread::sleep(Duration::from_secs(1));
      }
  });

  for received in rx {
    println!("Got: {}", received);
  }
}
```

- before we create the first spawned thread, we call `clone` on the transmitter. This will give us a new transmitter we can pass to the first spawned thread.

## Shared-State Concurrency

- another method is for multiple threads to access the same shared data. From Go language: "do not communicate by sharing memory".
- in a way, channels in any programming language are similar to single ownership, because once you transfer a value down a channel you should no longer use that value. Shared memory concurrency is like multiple ownership: multiple threads can access the same memory lacation at same time. Multiple ownership can add complexity because these different owners need managing

## Using Mutexes to Allow Access to Data from One Thread at a Time

- _Mutex_ is abbreviation for _mutual exclusion_ as in a mutex allows only one thread to access some data at any given time. To access the data in a mutex a thread must first signal that it wants access by asking to acquire the mutex's lock. The lock is data structure that is part of mutex that keeps track of who currently has exclusive access to the data. Therefor the mutex is described as _guarding_ the data it holds via the locking system.
- Mutexes have reputation for being difficult to use because you have to remember two rules:
  1. You must attempt to acquire the lock before using the data.
  2. When you are done with data that the mutex guards, you must unlock the data so other threads can acquire the lock

### The API of `Mutex<T>`

- mutex in single threaded context:

```rust
use std::sync::Mutex;

fn main() {
  let m = Mutex::new(5);
  {
    let mut num = m.lock().unwrap();
  }
  println!("m = {:?}", m);
}
```

- To access the data inside the mutex we use the `lock` method to acquire the lock. This will block the current thread so it can not do any work until it is our turn to have the lock.
- the call `lock` would fail if another thread holding the lock panicked. In that case , no one would ever be able to get the lock so we have chosen to `unwrap` and have this thread panic if we are in that situation.
- after we acquire the lock we can treat the return value, named `num` in this case as a mutable reference to data inside
- `Mutex<T>` is smart pointer. More accurately the call `lock` returns a smart pointer called `MutexGuard` wrapped in a `LockResult` that we handled with the call to `unwrap`.
- `MutexGuard` implement the `Deref` to point at our inner data. The smart pointer also has `Drop` implementation that releases the lock automatically when a `MutexGuard` goes out of scope.

### Sharing a `Mutex<T>` Between Multiple Threads

- let see sharing a value between multiple threads:

**THIS WILL not COMPILE:**

```rust
use std::sync::Mutex;
use std::thread;

fn main() {
  let counter = Mutex::new(0);
  let mut handles = vec![];

  for _ in 0..10 {
    let handle = thread::spawn(move || {
      let mut num - counter.lock().unwrap();

      *num += 1;
    });
    handle.push(handle);
  }

  for handle in handles {
    handle.join().unwrap();
  }

  println!("Result: {}", *counter.lock().unwrap());
}
```

Error:

```text
let counter = Mutex::new(0);
   |         ------- move occurs because `counter` has type `Mutex<i32>`, which does not implement the `Copy` trait
...
9  |         let handle = thread::spawn(move || {
   |                                    ^^^^^^^ value moved into closure here, in previous iteration of loop
10 |             let mut num = counter.lock().unwrap();
   |                           ------- use occurs due to use in closure
```

- the error states that the `counter` value was moved in the previous iteration of the loop. We can't move the ownership of the lock `counter` into multiple threads.

### Multiple Ownership with Multiple Threads

- we can try to use the `Rc<T>` smart pointer to create reference counted value.

**THIS WILL not COMPILE:**

```rust
use std::sync::Mutex;
use std::rc:Rc;
use std::thread;

fn main() {
  let counter = Rc::new(Mutex::new(0)); // add Rc::new
  let mut handles = vec![];

  for _ in 0..10 {
    let counter = Rc::clone(&counter); // add counter
    let handle = thread::spawn(move || {
      let mut num - counter.lock().unwrap();

      *num += 1;
    });
    handle.push(handle);
  }

  for handle in handles {
    handle.join().unwrap();
  }

  println!("Result: {}", *counter.lock().unwrap());
}
```

- error will states that: `Rc<Mutex<i32>>` cannot be sent between threads safely. The compiler is also telling us the reason why: the trait `Send` is not implemented for `Rc<Mutex<i32>>`
- unfortunatly `Rc<T>` is not safe to share across threads. When `Rc<T>` managed the reference count, it adds to the count for each call to `clone` and substract from the count when each clone is dropped. But it does not use any concurrency primitives to make sure that changes to the count can not be interrupted by another thread. This could lead to wrong counts that could lead to memory leaks or a value being dropped before we are done with it.

### Atomic Reference Counting with `Arc<T>`

- `Arc<T>` is a type like `Rc<T>` that is safe to use in concurrnt situation. The _A_ stands for _atomic_, meaning it is an atomically reference counted type. See the atomic standard library -> [link](https://doc.rust-lang.org/std/sync/atomic/index.html)
- The reason why the all primitive types are not atomic is that thread safty comes with a performance penalty that you only want to pay when you really need to.

```rust
use std::sync::{Arc,Mutex};
use std::thread;

fn main() {
  let counter = Arc::new(Mutex::new(0)); // add Rc::new
  let mut handles = vec![];

  for _ in 0..10 {
    let counter = Arc::clone(&counter); // add counter
    let handle = thread::spawn(move || {
      let mut num - counter.lock().unwrap();

      *num += 1;
    });
    handle.push(handle);
  }

  for handle in handles {
    handle.join().unwrap();
  }

  println!("Result: {}", *counter.lock().unwrap());
}
```

## Similarities Between `RefCell<T>/Rc<T>` and `Mutex<T>/Arc<T>`

- We can noticed that counter is immutable but we could get a mutable reference to the value inside it; this means `Mutex<T>` provides interior mutability, as the `Cell` family does.
- same as way we used `RefCell<T>` to allow us to mutate contents inside an `Rc<T>` we use `Mutex<T>` to mutate contents inside an `Arc<T>`
- `Mutex<T>` comes with the risk of creating _deadlocks_. These occurs when an operation needs to lock two resources and two thread have each acquired on of the locks, causing them to wait for each other forever.

## Extensible Concurrency with the `Sync` and `Send` Traits

- two concurrency concepts are embedded in the language: the `std::marker` traits `Sync` and `Send`

## Allowing Transference od Ownership Between Threads with `Send`

- the `Sync` marker trait indicates that it is safe for the type implementing `Sync` to be referenced from multiple threads. In other words , any type `T` is `Sync` if `&T` is `Send` meaning the reference can be sent safely to anoter thread.
- The smart pointer `Rc<T>` is also not `Sync` for the same reason that it is not `Send`. The `RefCell<T>` type and the family of related `Cell<T>` types are not `Sync`. The implementation of borrow checking that `RedCell<T>` does at runtime is not thread-safe.
