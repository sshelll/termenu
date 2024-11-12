# Termenu
> fzf-like terminal ui api for rust

## Demo
![demo](https://github.com/sshelll/assets/blob/master/termenu/termenu.jpg?raw=true)

## Install
Add this to your `Cargo.toml`:
```toml
[dependencies]
termenu = "2.1.0"
```

Or with `no-pipe` feature, check [Crate Features](#crate-features) for more information.:
```toml
[dependencies]
termenu = { version = "2.1.0", features = ["no-pipe"], default-features = false }
```

To install the binary, run:
```bash
# default
cargo install termenu@2.1.0
# with no-pipe feature
cargo install termenu@2.1.0 --features "no-pipe" --no-default-features
```

## Examples

check examples folder

```bash
# basic example
cargo run --example basic

# complex example
cargo run --example complex
```

## Use as a library

```rust
fn main() {
    let mut menu = termenu::Menu::new().unwrap();
    let mut item_list = Vec::new();
    for i in 1..=10 {
        item_list.push(termenu::Item::new(format!("{}th item", i).as_str(), i));
    }
    let selection = menu
        .set_title("test selection:")
        .add_list(item_list)
        .select()
        .unwrap();
    if let Some(selection) = selection {
        println!("You selected: {}", selection);
    }
}
```

## Crate Features

- no-pipe
This crate depends on `crossterm`, by default I enabled `use-dev-tty` feature on `crossterm` to support pipe input.  
If you don't need to use pipe input, or this feature causes some problems, you can disable it.  
In that way, `termenu` will only depend on `crossterm` without any features.

## NOTE

- Currently, termenu does not support window resizing.
