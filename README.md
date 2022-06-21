# Rust

- new objective learn Rust
- you have one month :)

## Something about Rust

- designed and developed be former Mozzila employee **Graydon Hoare** in 2009
- but first stable release 1.0 was released on May 15 2015.

## Start Of project

- install Rust on WSL2

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

```shell
mkdir my_project
cargo init
```

## Cargo Commnads

```sh
cargo
# Rust's package manager
cargo -h
# Run the main.rs script with quiet mode
cargo run -q
# release build
cargo run -q --release
```

---

# Learning Links

## Awesome tools

- https://rust-analyzer.github.io/ (text editor / IDE support)

- https://github.com/rust-lang/rustfmt (autoformatter)
- https://github.com/rust-lang/rust-clippy (code improvement suggestions)

## Documentation

- https://doc.rust-lang.org/book/
- https://doc.rust-lang.org/stable/rust-by-example/
- https://doc.rust-lang.org/std/
- https://rust-lang-nursery.github.io/rust-cookbook/ (examples)
- https://learning-rust.github.io/
- https://cheats.rs/ (nice cheatsheet)

## More specific explanations

- Modules:
  - http://www.sheshbabu.com/posts/rust-module-system/
- Lifetimes:
  - https://medium.com/nearprotocol/understanding-rust-lifetimes-e813bcd405fa
  - https://github.com/pretzelhammer/rust-blog/blob/master/posts/common-rust-lifetime-misconceptions.md
- Strings:
  - https://blog.thoughtram.io/string-vs-str-in-rust/
  - https://www.ameyalokare.com/rust/2017/10/12/rust-str-vs-String.html

## Blogs

- https://fasterthanli.me/
- https://blog.burntsushi.net/
- https://manishearth.github.io/
- https://steveklabnik.com/writing
- https://ianjk.com/rust-gamejam/
- https://lucumr.pocoo.org/tags/rust/

## Exercises

- https://tourofrust.com/
- https://github.com/rust-lang/rustlings
- https://the6p4c.github.io/rust-fmt-game/
- Or try implementing stuff from https://rosettacode.org/wiki/Rosetta_Code

## Example projects

- https://github.com/mre/idiomatic-rust
- https://github.com/n8henrie/advent2018-rust
- https://github.com/yaahc/color-eyre
