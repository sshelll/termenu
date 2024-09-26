use std::io;

use crossterm::{event, terminal};

pub struct Item<T> {
    display: String,
    value: T,
    cursor_pos: (u16, u16),
}

impl<T> Item<T> {
    pub fn new(display: &str, value: T) -> Item<T> {
        Item {
            display: display.to_string(),
            value,
            cursor_pos: (0, 0),
        }
    }
}

pub struct Menu<T> {
    item_list: Vec<Item<T>>,
}

impl<T> Menu<T> {
    pub fn new() -> Menu<T> {
        Menu {
            item_list: Vec::new(),
        }
    }

    pub fn add(&mut self, item: Item<T>) -> &mut Self {
        self.item_list.push(item);
        self
    }

    pub fn add_list(&mut self, items: Vec<Item<T>>) -> &mut Self {
        self.item_list.extend(items);
        self
    }

    pub fn add_item(&mut self, display: &str, value: T) -> &mut Self {
        self.item_list.push(Item::new(display, value));
        self
    }

    pub fn add_item_list(&mut self, items: Vec<(&str, T)>) -> &mut Self {
        self.item_list.extend(
            items
                .into_iter()
                .map(|(display, value)| Item::new(display, value)),
        );
        self
    }
}

impl<T> Menu<T> {
    pub fn run(&mut self) -> io::Result<()> {
        terminal::enable_raw_mode()?;
        loop {
            let evt = event::read()?;
            match evt {
                event::Event::Key(key) => {
                    eprint!("KeyPressed: ");
                    if key.modifiers != event::KeyModifiers::NONE {
                        eprint!("{}+", key.modifiers);
                    }
                    eprintln!("{}\r", key.code);
                    if key.code == event::KeyCode::Esc {
                        break;
                    }
                }
                _ => {}
            }
        }
        terminal::disable_raw_mode()?;
        Ok(())
    }
}
