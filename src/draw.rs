use crate::{
    term_clear_from_cursor_down, term_cursor_down, term_cursor_move, term_exec_stdout, term_printf,
    Menu,
};
use std::io;

impl<T> Menu<T> {
    pub(crate) fn draw(&mut self) -> io::Result<()> {
        self.clear()?;

        term_printf!("{}: ", self.title.as_ref().unwrap_or(&"select".to_string()));

        for (i, item) in self.item_list.iter().enumerate() {
            term_cursor_down!(1);
            if i == self.selection_idx as usize {
                term_printf!("> {}", item.display);
            } else {
                term_printf!("  {}", item.display);
            }
        }

        Ok(())
    }

    pub(crate) fn clear(&self) -> io::Result<()> {
        let (row, col) = self.cursor_abs_pos;
        term_cursor_move!(row, col);
        term_clear_from_cursor_down!();
        Ok(())
    }
}
