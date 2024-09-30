use std::io;

use crate::{macros::*, Menu, Mode};

impl<T> Menu<T> {
    pub(crate) fn enter_normal_mode(&mut self) -> io::Result<()> {
        self.mode = Mode::Normal;
        self.matched_item_indices.clear();
        term_exec!(crossterm::cursor::Hide);
        Ok(())
    }

    pub(crate) fn enter_query_mode(&mut self) -> io::Result<()> {
        self.mode = Mode::Query;
        self.query = String::new();
        term_exec!(crossterm::cursor::Show);
        Ok(())
    }
}
