use crate::{color::colorize, macros::*, Menu, Mode};
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
        let mut title = colorize(self.get_title(), &self.colorscheme.title);
        if let Mode::Query = self.mode {
            title = format!(
                "{} /{}",
                title,
                colorize(&self.query, &self.colorscheme.query)
            );
        }
        term_printf!("{}", title);
        Ok(())
    }

    pub(crate) fn print_cursor(&self) -> io::Result<()> {
        if let Mode::Query = self.mode {
            let (row, _) = self.cursor_abs_pos;
            term_cursor_move!(row, self.get_query_cursor_col());
        }
        Ok(())
    }

    pub(crate) fn print_options(&self) -> io::Result<()> {
        let (row, _) = self.cursor_abs_pos;
        let item_count = match self.mode {
            Mode::Normal => self.item_list.len(),
            Mode::Query => self.matched_item_indices.len(),
        } as u16;

        // print since the scroll offset
        let mut idx = self.scroll_offset;
        let mut i = 0;
        let mut has_more = false;
        loop {
            term_cursor_down!(1);

            // reached the end of the list
            if idx >= item_count {
                break;
            }

            // reach the end of the screen
            if row + i == self.max_row - 2 {
                has_more = true;
                term_print!(colorize("---more---", &self.colorscheme.more_tag));
                break;
            }

            // print
            let item_idx = match self.mode {
                Mode::Normal => idx as usize,
                Mode::Query => self.matched_item_indices[idx as usize],
            };
            let item = &self.item_list[item_idx];

            if idx == self.selection_idx + self.scroll_offset {
                term_printf!("> {}", colorize(&item.alias, &self.colorscheme.chosen_ln));
            } else {
                let text = match self.mode {
                    Mode::Normal => item.get_colored_alias_for_normal_mode(&self.colorscheme),
                    Mode::Query => item.get_colored_alias_for_query_mode(&self.colorscheme),
                };
                term_printf!("  {}", text);
            }

            idx += 1;
            i += 1;
        }

        if !has_more && self.show_end_tag {
            term_print!(colorize("---end---", &self.colorscheme.more_tag));
        }

        Ok(())
    }
}
