# Should not emit anything
``` {.rust name=unbound-ref}
println!("Will never be emmited");
```

```rust
fn main() {
  println!("I should be ignored");
}
```
