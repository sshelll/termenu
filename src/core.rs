use std::io::{self};

use colored::Colorize;
use crossterm::{event, terminal};

use crate::{macros::*, Item, Menu, Mode};

impl<T> Item<T> {
    pub fn new(display: &str, value: T) -> Item<T> {
        Item {
            alias: display.to_string(),
            value,
            score: None,
            matched_indices: None,
        }
    }

    pub(crate) fn get_colored_alias(&self) -> String {
        if self.matched_indices.is_none() || self.matched_indices.as_ref().unwrap().is_empty() {
            return self.alias.to_string();
        }
        let mut display = String::new();
        if let Some(indices) = self.matched_indices.as_ref() {
            let mut last = 0;
            for idx in indices.iter() {
                display.push_str(&self.alias[last..*idx]);
                let ch = &self.alias[*idx..=*idx].yellow().to_string();
                display.push_str(ch);
                last = idx + 1;
            }
            display.push_str(&self.alias[last..]);
        } else {
            display.push_str(&self.alias);
        }
        display
    }
}

// constructor
impl<T> Menu<T> {
    pub fn new() -> io::Result<Menu<T>> {
        let (col, row) = crossterm::cursor::position()?;
        let (_, rows) = crossterm::terminal::size()?;
        Ok(Menu {
            title: None,
            item_list: Vec::new(),
            mode: Mode::Normal,
            cursor_abs_pos: (row, col),
            max_row: rows,
            selection_idx: 0,
            selected: false,
            query: String::new(),
            query_cursor_col: 0,
            insert_pos: 0,
            scroll_offset: 0,
            matched_item_indices: Vec::new(),
        })
    }

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
    pub fn select(&mut self) -> io::Result<Option<Item<T>>> {
        terminal::enable_raw_mode()?;
        self.scroll_to_fit()?;
        self.enter_normal_mode()?;
        self.draw()?;
        loop {
            let evt = event::read()?;
            if let event::Event::Key(key) = evt {
                // PERF: sometimes a key event dose not change the state of the menu,
                // in that case, we can skip the draw step.
                // for exmaple: keep pressing up key when the first item is selected.
                if self.dispatch_key(key)? {
                    break;
                }
                self.draw()?;
            }
        }
        Ok(self.get_selection())
    }

    // when the terminal is at the bottom of the screen, scroll up to fit the menu
    fn scroll_to_fit(&mut self) -> io::Result<()> {
        // get the size of the terminal
        let (_, max_rows) = crossterm::terminal::size()?;
        let (row, _) = self.cursor_abs_pos;

        // check how many rows are left
        let left_rows = max_rows - row;

        // check how many items are there
        let item_cnt = self.item_list.len() as u16;

        // if there are more rows than items, no need to scroll
        if item_cnt <= left_rows {
            return Ok(());
        }

        // if there are more items than rows, scroll up
        let diff = item_cnt - left_rows;
        // eprintln!(
        //     "diff: {}, cur row: {}, after row: {}",
        //     diff,
        //     row,
        //     row.saturating_sub(diff + 1)
        // );
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
            term_printf!("{} {}", self.get_title(), item.alias);
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
