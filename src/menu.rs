use std::io::{self};

use crossterm::{event, terminal};

use crate::{
    ignore_io_error, term_clear_from_cursor_down, term_cursor_down, term_cursor_move,
    term_exec_stdout, term_printf, Item, Menu, Mode,
};

impl<T> Item<T> {
    pub fn new(display: &str, value: T) -> Item<T> {
        Item {
            display: display.to_string(),
            value,
        }
    }
}

impl<T> Default for Menu<T> {
    fn default() -> Self {
        Self::new().unwrap()
    }
}

impl<T> Menu<T> {
    pub fn new() -> io::Result<Menu<T>> {
        let (col, row) = crossterm::cursor::position()?;
        Ok(Menu {
            title: None,
            item_list: Vec::new(),
            mode: Mode::Normal,
            cursor_abs_pos: (row, col),
            selection_idx: 0,
            selected: false,
        })
    }

    pub fn set_title(&mut self, t: &str) -> &mut Self {
        self.title = Some(t.to_string());
        self
    }

    fn get_title(&self) -> &str {
        self.title.as_deref().unwrap_or("select")
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

    pub fn run(&mut self) -> io::Result<()> {
        terminal::enable_raw_mode()?;
        self.normal_mode()?;
        self.draw()?;
        loop {
            let evt = event::read()?;
            if let event::Event::Key(key) = evt {
                if self.dispatch_key(key)? {
                    return Ok(());
                }
                self.draw()?;
            }
            // if let event::Event::Key(key) = evt {
            //     eprint!("KeyPressed-{:?}: ", key.kind);
            //     if key.modifiers != event::KeyModifiers::NONE {
            //         eprint!("{}+", key.modifiers);
            //     }
            //     eprintln!("{}\r", key.code);
            //     if key.code == event::KeyCode::Esc {
            //         break;
            //     }
            // }
        }
    }

    pub fn get_selection(&self) -> Option<&Item<T>> {
        ignore_io_error!({
            let (row, col) = self.cursor_abs_pos;
            term_cursor_move!(row, col);
            term_clear_from_cursor_down!();
        });

        if !self.selected {
            return None;
        }

        let item = &self.item_list[self.selection_idx as usize];
        ignore_io_error!({
            term_printf!("{}: {}\n", self.get_title(), item.display);
            term_cursor_down!(1);
        });
        Some(item)
    }
}

impl<T> Drop for Menu<T> {
    fn drop(&mut self) {
        ignore_io_error!({
            term_cursor_down!(1);
            term_exec_stdout!(crossterm::cursor::Show);
        });
        terminal::disable_raw_mode().unwrap();
    }
}
