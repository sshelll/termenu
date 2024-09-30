use std::io::{self};

use crossterm::{event, terminal};
use once_cell::sync::OnceCell;

use crate::{color::colorize, macros::*, ColorScheme, Item, Menu, Mode};

impl<T> Item<T> {
    pub fn new(display: &str, value: T) -> Item<T> {
        Item {
            alias: display.to_string(),
            value,
            score: None,
            matched_indices: None,
        }
    }
}

// constructor
impl<T> Menu<T> {
    /// Create a new menu instance
    ///
    /// # Example
    /// ```no_run
    /// let mut menu = termenu::Menu::new().unwrap();
    /// ```
    pub fn new() -> io::Result<Menu<T>> {
        let (col, row) = crossterm::cursor::position()?;
        let (_, rows) = crossterm::terminal::size()?;
        Ok(Menu {
            colorscheme: ColorScheme::default(),
            title: None,
            item_list: Vec::new(),
            mode: Mode::Normal,
            cursor_abs_pos: (row + 1, col),
            max_row: rows,
            selection_idx: 0,
            selected: false,
            fuzzy_matcher: OnceCell::new(),
            query: String::new(),
            insert_idx: 0,
            scroll_offset: 0,
            matched_item_indices: Vec::new(),
        })
    }

    /// Set the title of the menu, which will be displayed at the top of the menu
    pub fn set_title(&mut self, t: &str) -> &mut Self {
        self.title = Some(t.to_string());
        self
    }

    pub(crate) fn get_title(&self) -> &str {
        self.title.as_deref().unwrap_or("select")
    }

    pub fn add(&mut self, item: Item<T>) -> &mut Self {
        self.item_list.push(Some(item));
        self
    }

    pub fn add_list(&mut self, items: Vec<Item<T>>) -> &mut Self {
        self.item_list.extend(items.into_iter().map(Some));
        self
    }
}

// select api
impl<T> Menu<T> {
    /// Start the menu and return the selection
    /// if the user presses `esc` or `ctrl-c`, `None` will be returned
    /// otherwise, the selected item will be returned
    pub fn select(&mut self) -> io::Result<Option<Item<T>>> {
        if self.item_list.is_empty() {
            return Ok(None);
        }
        terminal::enable_raw_mode()?;
        self.scroll_to_fit()?;
        self.enter_normal_mode()?;
        self.draw()?;
        loop {
            let evt = event::read()?;
            if let event::Event::Key(key) = evt {
                let resp = self.dispatch_key(key)?;
                if resp.exit() {
                    break;
                }
                if resp.redraw() {
                    self.draw()?;
                }
            }
        }
        Ok(self.get_selection())
    }

    // when the cursor is at the bottom of the screen, scroll up to fit the menu
    fn scroll_to_fit(&mut self) -> io::Result<()> {
        // get the size of the terminal
        let (_, max_rows) = crossterm::terminal::size()?;
        let (row, _) = self.cursor_abs_pos;

        // check how many rows are left
        let left_rows = max_rows - row;

        // check how many items are there
        let item_cnt = self.item_list.len() as u16;

        // if there are more rows than items, no need to scroll
        if item_cnt < left_rows {
            return Ok(());
        }

        // if there are more items than rows, scroll up
        let diff = item_cnt - left_rows + 1;
        term_exec!(crossterm::terminal::ScrollUp(diff.min(max_rows - 1)));

        // we've alreay scrolled up, but the cursor is still at the bottom of the screen
        // just move the cursor up
        self.cursor_abs_pos = (row.saturating_sub(diff + 1), 0);
        Ok(())
    }

    fn get_selection(&mut self) -> Option<Item<T>> {
        ignore_io_error!(self.clear()?);

        if !self.selected {
            return None;
        }

        let item_idx = match self.mode {
            Mode::Normal => (self.selection_idx + self.scroll_offset) as usize,
            Mode::Query => {
                self.matched_item_indices[(self.selection_idx + self.scroll_offset) as usize]
            }
        };

        let item = self.item_list.get_mut(item_idx).unwrap().take().unwrap();

        // print the result to the terminal
        ignore_io_error!({
            term_printf!(
                "{} {}",
                colorize(self.get_title(), &self.colorscheme.title),
                item.alias
            );
            term_cursor_down!(1);
        });

        Some(item)
    }
}

impl<T> Drop for Menu<T> {
    fn drop(&mut self) {
        ignore_io_error!({
            term_cursor_col!(0);
            term_exec!(crossterm::cursor::Show);
        });
        terminal::disable_raw_mode().unwrap();
    }
}
