use std::io::Write;

/// Try this:
/// ```bash
/// echo $(echo '1st item\n2nd item\n3rd item' | cargo run --example pipe | grep item)
/// ```
fn main() {
    let input = get_pipe_input();
    let mut menu = termenu::Menu::new().unwrap();
    input.iter().for_each(|line| {
        menu.add(termenu::Item::new(line, ()));
    });
    menu.enable_print_result(false);
    let selection = menu.set_title("test selection:").select_item().unwrap();
    if let Some(selection) = selection {
        println!("{}", selection.alias);
        std::io::stdout().flush().unwrap();
    }
}

fn get_pipe_input() -> Vec<String> {
    use std::io::BufRead;
    let mut input = Vec::new();
    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        input.push(line.unwrap());
    }
    input
}
