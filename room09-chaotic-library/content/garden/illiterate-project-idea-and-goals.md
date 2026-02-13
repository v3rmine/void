---
title: Illiterate project idea and goals
date: 2026-01-30
description: >
  The ideas and goals for "Illiterate", a literate programming tool in markdown which needs to be as flexible and accessible as possible
taxonomies:
  tags:
    - project
extra:
  serie: project-illiterate
  serie_index: 1
  guid: f49496b1-2f7d-485d-bb20-e93ba5d7f64b
  toc: true
---

# What?
A literate programming tool in markdown which is as flexible and accessible as possible.

# Why
Because [Org-babel](https://orgmode.org/worg/org-contrib/babel/how-to-use-Org-Babel-for-R.html) is excellent, but it requires writing documentation in Org mode, and most of the tools I use support Markdown well but Org mode support is minimal.

And I found that [entangled.py](https://github.com/entangled/entangled.py) works but isn't very flexible on the error recovery (like [moved files](https://github.com/entangled/entangled.py/issues/88)), I wanted something more closer to the usage of Org, just in Markdown.

I think that every small project could benefit from literate programming. But a relaxed one, it doesn't need to be professional or with a perfect grammar. I think even a "bad" literate documentation is still better than a project without documentation.

# Goals
## v1.0
For the `v1.0` I want to have the basic tool with as features:
- the commands:
  - `tangle`: to generate the code from the markdown source
  - `watch`: to watch and regenerate the code automatically
  - `check`: only check for missing references or broken internal links without emitting anything
- it should throw warnings when a reference is missing
- handle source file rename/move/deletion
- have an optional config file / parameters

## Next
The goals afterwards *(for the moment)* should be in the order of priority: 
1. a basic lsp server that allow to navigate to references/usages
2. extending the lsp server to wraps other lsp for the emitted files so you can have diagnostics or go to the emitted file
3. add support for defining attributes using the `quarto_attributes` [style](https://entangled.github.io/quarto_attributes/) 

# Syntax
## Emitting a file
````md
``` {.rust file=src/bin.rs}
fn main() {
  println!("Hello World!");
}
```
````

## Creating a reference
````md
``` {.rust name=ref}
"Hello World!"
```
````

## Linking to a reference
````md
``` {.rust file=src/bin.rs}
fn main() {
  println!(<<ref>>);
}
```
````

## Linking ref inside other refs
````md
``` {.rust name=ref2}
println!(<<ref>>)
```
````

## Customizing the ref regex
> [!NOTE]
> The `ref_reg` if customized need the `ref` capturing group

````md
``` {.rust ref_reg="^\s*\[\[(?<ref>[a-z0-9]+)\]\]\s*$"}
fn main() {
  [[ref2]]
}
```
````

## Disabling reference replacement
````md
``` {.rust plain file=src/bin.rs}
# ref2 will never be replaced
fn main() {
  <<ref2>>
}
```
````

## Including previous content block as comment
> [!NOTE]
> Only works with the last markdown block in the file

````md
The following code is a basic
Rust code printing `Hello World`

``` {.rust comments}
fn main() {
  println!("Hello World!");
}
```
````

This will output 
```rust
// The following code is a basic
// Rust code printing `Hello World`
fn main() {
  println!("Hello World!");
}
```

## Changing the comment style
### Single line comment
````md
The following code is a basic
Rust code printing `Hello World`

``` {.rust comments open="///"}
fn main() {
  println!("Hello World!");
}
```
````

This will output 
```rust
/// The following code is a basic
/// Rust code printing `Hello World`
fn main() {
  println!("Hello World!");
}
```

### Multi line comment
````md
The following code is a basic
Rust code printing `Hello World`

``` {.rust comments multiline open="/*\n" newline=" *" close="\n */"}
fn main() {
  println!("Hello World!");
}
```
````

This will output 
```rust
/* 
 * The following code is a basic
 * Rust code printing `Hello World`
 */
fn main() {
  println!("Hello World!");
}
```

## Variables
> [!NOTE]
> Nothing is dynamic here only static variables

````md
``` {.rust name=print_msg var_log_str='"Hello World!"'}
println!(print_msg);
```

```{.rust file=src/bin.rs}
fn main() {
  // Outputs: println!("Hello World");
  <<ref_with_var>>
  // Outputs: println!("Hello You");
  <<ref_with_var(log_str='"Hello You!"')>> 
}
```
````

## Customizing the variables regex
> [!NOTE]
> The `ref_reg` if customized need the `ref` and the `vars` capturing groups (vars only if variables are used)
> And the `var_reg` needs the `name` and `value` capturing groups
> You can omit `ref_reg` if the default works for you

````md
``` {.rust ref_reg="^\s*\[\[(?<ref>[a-z_]+)(?:\((?<vars>(?:(?:[a-z_]+)=(?:'[^']*'),? ?)+)\))?\]\]\s*$" var_reg="(?<name>[a-z_]+)='(?<value>[^']+)'"}
fn main() {
  [[ref_with_var(log_str='"Hello You!"')]]
}
```
````
