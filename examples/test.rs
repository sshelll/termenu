use termenu::quit_now;

fn main() {
    let mut menu = termenu::Menu::new().unwrap_or_else(|e| {
        quit_now!("Error: {}", e);
    });
    menu.set_title("test selection")
        .add_item("first opt", 123)
        .add_item("second opt", 259)
        .add_item("third opt", 341)
        .run()
        .unwrap_or_else(|e| {
            quit_now!("Error: {}", e);
        });
    let selection = menu.get_selection();
    if let Some(selection) = selection {
        println!("You selected: {}", selection.value);
    } else {
        println!("You didn't select anything");
    }
}
