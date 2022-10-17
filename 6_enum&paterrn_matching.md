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
