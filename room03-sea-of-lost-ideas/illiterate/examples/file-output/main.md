# Emits files to disk

```{.rust name=config}
pub const APP_NAME: &str = "my-app";
```

```{.rust file=src/lib.rs}
mod config;
use config::APP_NAME;

pub fn run() {
    println!("Running {}", APP_NAME);
}
```

```{.rust file=src/main.rs}
fn main() {
    my_app::run();
}
```
