# Termenu
> fzf-like terminal ui api for rust

## Demo
![demo](https://github.com/sshelll/assets/blob/master/termenu/termenu.jpg?raw=true)

## Examples

check examples folder

```bash
# basic example
cargo run --example basic

# complex example
cargo run --example complex
```

## Basic Usage

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
        println!("You selected: {}", selection.value);
    }
}
```
