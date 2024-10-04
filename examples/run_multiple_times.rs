fn main() {
    let mut menu = termenu::Menu::new().unwrap();
    run_once(&mut menu);
    run_once(&mut menu);
    run_once(&mut menu);
}

fn run_once(menu: &mut termenu::Menu<i32>) {
    println!("[start running once]");
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
    } else {
        println!("You didn't select anything");
    }
    menu.reset().unwrap();
}
