# indt

Just for experience Rust library for writing with indentions.

# Usage

Clone repository

```git
git clone https://github.com/Ususucsus/indt-rs
```

and add dependency in your `Cargo.toml` file

```toml
indt = { path = "../indt-rs" }
```

Create `Indent` instance

```rust
let stdout = &mut std::io::stdout();
let indt = indt::Indent(stdout);
```

make indetion bigger 

```rust
indt.more()
```

 and write something

```rust
write!(indt, "Hello, {}", 42);
```

# Other

To see full documentation 

```
cargo doc --open
```

To test

```
cargo test
```

This is *not* meant to be used somewhere. 
Instead, use something well-tested like [indented](https://crates.io/crates/indenter/)