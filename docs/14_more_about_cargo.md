# More About Cargo and Crates.io

- Customize your build through release profiles
- Publish libraries on [crates.io](https://crates.io/)
- Organize large projects with workspaces
- Install binaries from crates.io
- Extend Cargo using custom commands
- see [documentation](https://doc.rust-lang.org/cargo/)

## Customizing Builds with Release Profiles

- Cargo has two main profiles: `dev` and `release`.
- when you run : `cargo build` it will compile to dev and when you run `cargo build --release` it will compile into release profile
- Cargo has default settings for each of the profiles that apply when you haven't explicitly added any `[profile.*]` sections in the project’s Cargo.toml file. By adding `[profile.*]` sections for any profile you want to customize, you override any subset of the default settings. For example, here are the default values for the `opt-level` setting for the `dev` and `release` profiles:

```toml
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
```

- The opt-level setting controls the number of optimizations Rust will apply to your code, with a range of 0 to 3. Applying more optimizations extends compiling time, so if you’re in development and compiling your code often, you’ll want fewer optimizations to compile faster even if the resulting code runs slower.
- For the full list of configuration options and defaults for each profile, see Cargo’s [documentation](https://doc.rust-lang.org/cargo/reference/profiles.html)
