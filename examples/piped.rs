use crossterm::cursor::{MoveUp, RestorePosition, SavePosition};

fn main() {
    let mut tty = std::fs::OpenOptions::new()
        .write(true)
        .open("/dev/tty")
        .unwrap();
    crossterm::execute!(tty, SavePosition).unwrap();

    let input = handle_by_pipe();

    crossterm::execute!(tty, RestorePosition).unwrap();
    crossterm::execute!(tty, MoveUp(5)).unwrap();
    crossterm::execute!(tty, crossterm::style::Print(input.join("\n"))).unwrap();
}

fn handle_by_pipe() -> Vec<String> {
    use std::io::Read;
    let stdin = std::io::stdin();
    let mut buffer = String::new();
    let mut input = vec![];
    if stdin.lock().read_to_string(&mut buffer).is_ok() {
        input = buffer
            .lines()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
    }
    input
}
