use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub(crate) fn get_cursor_position(is_pipe: bool) -> io::Result<(u16, u16)> {
    if !is_pipe {
        return crossterm::cursor::position();
    }

    let already_raw = crossterm::terminal::is_raw_mode_enabled()?;
    if !already_raw {
        crossterm::terminal::enable_raw_mode()?;
    }
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
    if !already_raw {
        crossterm::terminal::disable_raw_mode()?;
    }

    if let Some(caps) = response
        .strip_prefix("\x1b[")
        .and_then(|s| s.strip_suffix("R"))
    {
        let mut parts = caps.split(';');
        if let (Some(row), Some(col)) = (parts.next(), parts.next()) {
            let row = row.parse().unwrap_or(0);
            let col = col.parse().unwrap_or(0);
            return Ok((col, row - 1));
        }
    }
    Err(io::Error::new(
        io::ErrorKind::Other,
        "Failed to parse cursor position",
    ))
}

pub(crate) struct KeyListener {
    is_pipe: bool,
    tty: Option<File>,
}

impl KeyListener {
    pub(crate) fn new(is_pipe: bool) -> Self {
        Self {
            is_pipe,
            tty: {
                if !is_pipe {
                    None
                } else {
                    Some(
                        OpenOptions::new()
                            .read(true)
                            .write(true)
                            .open("/dev/tty")
                            .unwrap(),
                    )
                }
            },
        }
    }

    pub(crate) fn read_key_event(&mut self) -> io::Result<KeyEvent> {
        if !self.is_pipe {
            loop {
                let evt = crossterm::event::read()?;
                if let crossterm::event::Event::Key(evt) = evt {
                    return Ok(evt);
                }
            }
        }
        let buf = &mut [0; 1];
        let tty = self.tty.as_mut().unwrap();
        tty.read_exact(buf)?;

        // TODO: need to handle more situations
        match buf[0] {
            3 => Ok(KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL)),
            4 => Ok(KeyEvent::new(KeyCode::Char('d'), KeyModifiers::CONTROL)),
            26 => Ok(KeyEvent::new(KeyCode::Char('z'), KeyModifiers::CONTROL)),
            10 | 13 => Ok(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE)),
            _ => Ok(KeyEvent::new(
                KeyCode::Char(buf[0] as char),
                KeyModifiers::NONE,
            )),
        }
    }
}
