# Chain of references

```{.rust name=base}
2
```

```{.rust name=uses-base}
40 + <<base>>
```

```{.rust name=uses-nested}
let y = <<uses-base>>;
println!("y = {}", y);
```

```{.rust file=src/main.rs}
fn main() {
    <<uses-nested>>
}
```
