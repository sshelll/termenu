use std::io;

use crate::{macros::*, Menu, Mode};

impl<T> Menu<T> {
    pub(crate) fn enter_normal_mode(&mut self) -> io::Result<()> {
        self.mode = Mode::Normal;
        term_exec!(crossterm::cursor::Hide);
        Ok(())
    }

    pub(crate) fn enter_query_mode(&mut self) -> io::Result<()> {
        self.mode = Mode::Query;
        self.query = String::new();
        self.query_cursor_col = self.get_title_line().len() as u16;
        term_exec!(crossterm::cursor::Show);
        Ok(())
    }
}
