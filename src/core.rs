use std::io::{self, IsTerminal};

use crossterm::terminal;
use once_cell::sync::OnceCell;

use crate::{color::colorize, macros::*, term, ColorScheme, Item, Menu, Mode};

impl<T: Send + Sync> Item<T> {
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
impl<T: Send + Sync> Menu<T> {
    /// Create a new menu instance
    ///
    /// # Example
    /// ```no_run
    /// let mut menu = termenu::Menu::new().unwrap();
    /// ```
    pub fn new() -> io::Result<Menu<T>> {
        let is_pipe = !io::stdin().is_terminal();
        let (_, row) = term::get_cursor_position(is_pipe)?;
        let (_, rows) = crossterm::terminal::size()?;
        Ok(Menu {
            is_pipe,
            enable_print_result: true,
            colorscheme: ColorScheme::default(),
            title: None,
            item_list: Vec::new(),
            mode: Mode::Normal,
            cursor_abs_pos: (row, 0),
            max_row: rows,
            selection_idx: 0,
            selected: false,
            fuzzy_matcher: OnceCell::new(),
            query: String::new(),
            insert_idx: 0,
            scroll_offset: 0,
            max_height_percent: 1.0,
            matched_item_indices: Vec::new(),
            show_end_tag: true,
            rayon_pool: OnceCell::new(),
        })
    }

    /// Set if show the '---end---' tag at the end of the menu
    pub fn show_end_tag(&mut self, b: bool) -> &mut Self {
        self.show_end_tag = b;
        self
    }

    /// Set the max height of the menu, should be a percentage in range (0, 1], otherwise it will
    /// be ignored
    pub fn set_max_height(&mut self, percent: f32) -> &mut Self {
        if percent <= 0.0 || percent > 1.0 {
            return self;
        }
        let (row, _) = self.cursor_abs_pos;
        let display_cnt = (self.max_row as f32 * percent) as u16;
        self.max_height_percent = percent;
        self.max_row = self.max_row.min(display_cnt + row + 1);
        self
    }

    /// Set the title of the menu, which will be displayed at the top of the menu
    pub fn set_title(&mut self, t: &str) -> &mut Self {
        self.title = Some(t.to_string());
        self
    }

    /// Controls whether to print the result to stderr after selecting.
    ///
    /// For example, by default the result would be print like this in stderr:
    /// select: $selected_item
    ///
    /// You can disable this by calling this function with `false`.
    ///
    /// This is useful when you want to use termenu in a pipe like this:
    /// ```bash
    /// cat file.txt | termenu | grep 123
    /// ```
    pub fn enable_print_result(&mut self, b: bool) -> &mut Self {
        self.enable_print_result = b;
        self
    }

    pub(crate) fn get_title(&self) -> &str {
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

    /// reset the menu state and clear all the items that have been added,
    /// this will not reset the color scheme and other reusable settings
    pub fn reset(&mut self) -> io::Result<()> {
        self.item_list.clear();
        self.matched_item_indices.clear();
        self.selected = false;
        self.mode = Mode::Normal;

        // let (_, row) = crossterm::cursor::position()?;
        let (_, row) = term::get_cursor_position(self.is_pipe)?;
        let (_, rows) = crossterm::terminal::size()?;
        self.cursor_abs_pos = (row, 0);
        self.max_row = rows;
        self.query = String::new();
        self.insert_idx = 0;
        self.scroll_offset = 0;

        Ok(())
    }
}

// select api
impl<T: Send + Sync> Menu<T> {
    /// Start the menu and return the selection
    /// if the user presses `esc` or `ctrl-c`, `None` will be returned
    /// otherwise, the selected item will be returned
    pub fn select(&mut self) -> io::Result<Option<&T>> {
        match self.select0()? {
            Some(item) => Ok(Some(&item.value)),
            None => Ok(None),
        }
    }

    /// Same as [select](Self::select), but return the selected [Item] instead of the value
    pub fn select_item(&mut self) -> io::Result<Option<&Item<T>>> {
        self.select0()
    }

    fn select0(&mut self) -> io::Result<Option<&Item<T>>> {
        use crossterm::event;
        if self.item_list.is_empty() {
            return Ok(None);
        }
        terminal::enable_raw_mode()?;
        self.scroll_to_fit()?;
        self.enter_normal_mode()?;
        self.draw()?;
        loop {
            if let event::Event::Key(key) = event::read()? {
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
        let (row, _) = self.cursor_abs_pos;

        // check how many rows are left
        let (_, term_max_row) = crossterm::terminal::size()?;
        let left_rows = term_max_row - row;

        // check how many items are there
        let item_cnt =
            self.item_list
                .len()
                .min((self.max_row as f32 * self.max_height_percent) as usize) as u16;

        // if there are more rows than items, no need to scroll
        // plus 2 is for the title and the more tag
        if item_cnt + 2 < left_rows {
            return Ok(());
        }

        // if there are more items than rows, scroll up
        let diff = item_cnt + 2 - left_rows;
        term_exec!(crossterm::terminal::ScrollUp(diff.min(self.max_row - 1)));

        // we've alreay scrolled up, but the cursor is still at the bottom of the screen
        // just move the cursor up
        self.cursor_abs_pos = (row.saturating_sub(diff), 0);
        Ok(())
    }

    fn get_selection(&mut self) -> Option<&Item<T>> {
        ignore_io_error!(self.clear()?);

        if !self.selected {
            if self.enable_print_result {
            ignore_io_error!({
                term_printf!("{}", colorize(self.get_title(), &self.colorscheme.title),);
                term_cursor_down!(1);
            });
        }
            return None;
        }

        let item_idx = match self.mode {
            Mode::Normal => (self.selection_idx + self.scroll_offset) as usize,
            Mode::Query => {
                self.matched_item_indices[(self.selection_idx + self.scroll_offset) as usize]
            }
        };

        let item = self.item_list.get(item_idx).unwrap();

        // print the result to the terminal
        if self.enable_print_result {
            ignore_io_error!({
                term_printf!(
                    "{} {}",
                    colorize(self.get_title(), &self.colorscheme.title),
                    item.alias
                );
                term_cursor_down!(1);
            });
        }

        Some(item)
    }
}

// drop trait
impl<T: Send + Sync> Drop for Menu<T> {
    fn drop(&mut self) {
        ignore_io_error!({
            term_cursor_col!(0);
            term_exec!(crossterm::cursor::Show);
        });
        terminal::disable_raw_mode().unwrap();
    }
}
