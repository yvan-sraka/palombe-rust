# 🕊️ Palombe [![cargo version](https://img.shields.io/crates/v/palombe.svg)](https://crates.io/crates/palombe)

Palombe lets you send and receive messages synchronously through different processes using named pipes

## Quick example

### Thread A

```rust
extern create palombe;

fn main() {
    palombe.send("foo", "bar");
}
```

### Thread B

```rust
extern create palombe;

fn main() {
    println("{}", palombe.receive("foo")); // bar
}
```
