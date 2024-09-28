use crate::{
    term_clear_from_cursor_down, term_cursor_down, term_cursor_move, term_exec, term_print,
    term_printf, Menu, Mode,
};
use std::io;

impl<T> Menu<T> {
    pub(crate) fn draw(&self) -> io::Result<()> {
        self.clear()?;

        self.print_title()?;

        self.print_options()?;

        // NOTE: print cursor last because it may move the cursor down during the above calls
        self.print_cursor()
    }

    pub(crate) fn clear(&self) -> io::Result<()> {
        self.reset_cursor()?;
        term_clear_from_cursor_down!();
        Ok(())
    }

    pub(crate) fn reset_cursor(&self) -> io::Result<()> {
        let (row, col) = self.cursor_abs_pos;
        term_cursor_move!(row, col);
        Ok(())
    }

    pub(crate) fn print_title(&self) -> io::Result<()> {
        term_printf!("{}", self.get_title_line());
        Ok(())
    }

    pub(crate) fn print_cursor(&self) -> io::Result<()> {
        if let Mode::Query = self.mode {
            let (row, _) = self.cursor_abs_pos;
            term_cursor_move!(row, self.query_cursor_col);
        }
        Ok(())
    }

    pub(crate) fn print_options(&self) -> io::Result<()> {
        let (row, _) = self.cursor_abs_pos;
        let item_count = self.item_list.len() as u16;

        // print since the scroll offset
        let mut idx = self.scroll_offset;
        let mut i = 0;
        loop {
            term_cursor_down!(1);
            if idx >= item_count {
                break;
            }
            if row + i == self.max_row - 2 {
                term_print!("---more---");
                break;
            }
            let item = &self.item_list[idx as usize];
            if let Some(item) = item {
                if idx == self.selection_idx + self.scroll_offset {
                    term_printf!("> {}", item.alias);
                } else {
                    term_printf!("  {}", item.alias);
                }
            }
            idx += 1;
            i += 1;
        }

        // for (i, item) in self.item_list.iter().enumerate() {
        //     term_cursor_down!(1);
        //     if row + i as u16 == self.max_row - 2 {
        //         if i < item_count as usize - 1 {
        //             term_print!("---more---");
        //         }
        //         break;
        //     }
        //     let alias = &item.as_ref().unwrap().alias;
        //     if i == self.selection_idx as usize {
        //         term_printf!("> {}", alias);
        //     } else {
        //         term_printf!("  {}", alias);
        //     }
        // }

        Ok(())
    }

    pub(crate) fn get_title_line(&self) -> String {
        let title = self.get_title();
        if let Mode::Query = self.mode {
            format!("{} /{}", title, self.query)
        } else {
            title.to_string()
        }
    }
}
