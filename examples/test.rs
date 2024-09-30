use termenu::Item;

macro_rules! quit_now {
    ($content:expr, $($arg:tt)*) => {
        eprintln!($content, $($arg)*);
        std::process::exit(1);
    };
}

fn main() {
    let mut menu = termenu::Menu::new().unwrap_or_else(|e| {
        quit_now!("Error: {}", e);
    });
    let mut item_list = Vec::new();
    for i in 1..=20 {
        item_list.push(Item::new(format!("{}th item", i).as_str(), i));
    }
    let selection = menu
        .set_title("test selection:")
        .add_list(item_list)
        .select()
        .unwrap_or_else(|e| {
            quit_now!("Error: {}", e);
        });
    if let Some(selection) = selection {
        println!("You selected: {}", selection.value);
    } else {
        println!("You didn't select anything");
    }
}
