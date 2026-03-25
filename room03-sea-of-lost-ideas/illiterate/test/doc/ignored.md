# Should not emit anything
``` {.rust #unbound-ref}
println!("Will never be emmited");
```

```rust
fn main() {
  println!("I should be ignored");
}
```

```rust
fn main() {
  <<refwithouttarget>>
}
```
