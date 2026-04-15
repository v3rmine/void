# Circular dependency example

```{.rust name=cycle/a}
<<cycle/b>>
```

```{.rust name=cycle/b}
<<cycle/a>>
```
