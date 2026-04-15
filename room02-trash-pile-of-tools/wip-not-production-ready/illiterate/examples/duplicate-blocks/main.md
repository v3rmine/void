# Duplicate named blocks

```{.rust name=test}
fn hello() {
    println!("Hello");
}
```

Some text in between.

```{.rust name=test}
fn world() {
    println!("World");
}
```

And a reference to the test block:

```{.rust file=src/main.rs}
<<test>>
```
