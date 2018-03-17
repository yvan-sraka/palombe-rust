# Palombe

Palombe lets you send and receive messages synchronously through different processes using named pipes

## Quick example

```rust
extern create palombe;

fn main() {
    palombe.send("foo", "bar");
    println("{}", palombe.receive("foo")); // bar
}
```
