fn main() {
    let mut menu = termenu::Menu::new();
    menu.add_item("first opt", 1)
        .add_item("second opt", 2)
        .add_item("third opt", 3);
    menu.run().unwrap();
}
