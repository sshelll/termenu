use std::fs::OpenOptions;
use std::io::{self, Read, Write};

use crossterm::tty::IsTty;

// This file has nothing to do with termenu,
// I wrote this file to show how to get the cursor position in the terminal using tty.
// Currently, crossterm v0.28 doesn't support piped input.
fn get_cursor_position() -> io::Result<(u16, u16)> {
    // open tty
    let mut tty = OpenOptions::new().read(true).write(true).open("/dev/tty")?;

    // ask for cursor position
    write!(tty, "\x1b[6n")?;
    tty.flush()?;

    // read resp
    // resp fmt: \x1b[{row};{col}R
    let mut response = String::new();
    let mut buffer = [0; 1];
    while tty.read(&mut buffer)? == 1 {
        response.push(buffer[0] as char);
        if buffer[0] == b'R' {
            break;
        }
    }

    if let Some(caps) = response
        .strip_prefix("\x1b[")
        .and_then(|s| s.strip_suffix("R"))
    {
        let mut parts = caps.split(';');
        if let (Some(row), Some(col)) = (parts.next(), parts.next()) {
            let row = row.parse().unwrap_or(0);
            let col = col.parse().unwrap_or(0);
            return Ok((row, col));
        }
    }
    Err(io::Error::new(
        io::ErrorKind::Other,
        "Failed to parse cursor position",
    ))
}

fn main() {
    println!("is tty? {}", io::stdout().is_tty());
    crossterm::terminal::enable_raw_mode().unwrap();
    match get_cursor_position() {
        Ok((row, col)) => println!("Cursor position: row {}, col {}", row, col),
        Err(e) => eprintln!("Error: {}", e),
    }
    crossterm::terminal::disable_raw_mode().unwrap();
}
