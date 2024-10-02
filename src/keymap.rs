use std::{cmp, io};

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{macros::*, Menu, Mode};

pub(crate) struct KeyResponse(bool, bool);

impl KeyResponse {
    fn new(exit: bool, redraw: bool) -> Self {
        KeyResponse(exit, redraw)
    }

    pub(crate) fn exit(&self) -> bool {
        self.0
    }

    pub(crate) fn redraw(&self) -> bool {
        self.1
    }
}

impl<T> Menu<T> {
    pub(crate) fn dispatch_key(&mut self, key: KeyEvent) -> io::Result<KeyResponse> {
        match key.modifiers {
            KeyModifiers::NONE => self.dispatch_code(key.code),

            KeyModifiers::CONTROL => {
                if key.code == KeyCode::Char('c') {
                    return Ok(KeyResponse::new(true, false));
                }
                if let Mode::Query = self.mode {
                    match key.code {
                        KeyCode::Char('n') => return self.key_down(),
                        KeyCode::Char('p') => return self.key_up(),
                        _ => {}
                    }
                }
                Ok(KeyResponse::new(false, false))
            }

            _ => Ok(KeyResponse::new(false, false)),
        }
    }

    fn dispatch_code(&mut self, code: KeyCode) -> io::Result<KeyResponse> {
        match self.mode {
            Mode::Normal => self.dispatch_normal(code),
            Mode::Query => self.dispatch_query(code),
        }
    }

    fn dispatch_normal(&mut self, code: KeyCode) -> io::Result<KeyResponse> {
        match code {
            KeyCode::Up | KeyCode::Char('k') => return self.key_up(),

            KeyCode::Down | KeyCode::Char('j') => return self.key_down(),

            KeyCode::Esc => return self.key_esc(),

            KeyCode::Enter => return self.key_enter(),

            KeyCode::Char('/') => {
                self.enter_query_mode()?;
            }
            _ => {}
        };
        Ok(KeyResponse::new(false, true))
    }

    fn dispatch_query(&mut self, code: KeyCode) -> io::Result<KeyResponse> {
        match code {
            KeyCode::Esc => return self.key_esc(),

            KeyCode::Up => return self.key_up(),

            KeyCode::Down => return self.key_down(),

            KeyCode::Left => {
                self.insert_idx = self.insert_idx.saturating_sub(1);
            }

            KeyCode::Right => {
                self.insert_idx = cmp::min(self.insert_idx + 1, self.query.len());
            }

            KeyCode::Enter => return self.key_enter(),

            KeyCode::Char(c) => {
                let insert_pos = get_insert_pos!(&self.query, self.insert_idx);
                self.query.insert(insert_pos, c);
                self.insert_idx += 1;
                self.fuzzy_match();
            }

            KeyCode::Backspace => {
                if !self.query.is_empty() {
                    self.insert_idx = self.insert_idx.saturating_sub(1);
                    let pos = get_insert_pos!(&self.query, self.insert_idx);
                    self.query.remove(pos);
                    self.fuzzy_match();
                }
            }
            _ => {}
        }
        Ok(KeyResponse(false, true))
    }
}

impl<T> Menu<T> {
    fn key_up(&mut self) -> io::Result<KeyResponse> {
        if self.selection_idx == 0 {
            if self.scroll_offset == 0 {
                return Ok(KeyResponse(false, false));
            }
            self.scroll_offset -= 1;
            return Ok(KeyResponse(false, true));
        }
        self.selection_idx -= 1;
        Ok(KeyResponse(false, true))
    }

    fn key_down(&mut self) -> io::Result<KeyResponse> {
        let item_cnt = match self.mode {
            Mode::Normal => self.item_list.len(),
            Mode::Query => self.matched_item_indices.len(),
        } as u16;
        if self.selection_idx + self.scroll_offset == item_cnt - 1 {
            return Ok(KeyResponse(false, false));
        }
        let (row, _) = self.cursor_abs_pos;
        if self.selection_idx + row == self.max_row - 3 {
            self.scroll_offset += 1;
            return Ok(KeyResponse(false, true));
        }
        self.selection_idx += 1;
        Ok(KeyResponse(false, true))
    }

    fn key_esc(&mut self) -> io::Result<KeyResponse> {
        match self.mode {
            Mode::Normal => Ok(KeyResponse(true, false)),
            _ => {
                self.enter_normal_mode()?;
                Ok(KeyResponse(false, true))
            }
        }
    }

    fn key_enter(&mut self) -> io::Result<KeyResponse> {
        match self.mode {
            Mode::Normal => {
                self.selected = true;
            }
            Mode::Query => {
                if !self.matched_item_indices.is_empty() {
                    self.selected = true;
                }
            }
        }
        Ok(KeyResponse(true, false))
    }
}

impl<T> Menu<T> {
    pub(crate) fn get_query_cursor_col(&self) -> u16 {
        let mut col = 0;

        // calculate the prefix cells
        let prefix = format!("{} /", self.get_title());
        col += prefix
            .chars()
            // a Chinese character takes 3 bytes, however, it only takes 2 cells in terminals
            .fold(0, |acc, c| acc + cmp::min(2, c.len_utf8() as u16));

        // calculate the query cells from 0 to self.insert_idx
        col += self
            .query
            .chars()
            .take(self.insert_idx)
            .fold(0, |acc, c| acc + cmp::min(2, c.len_utf8() as u16));

        col
    }
}
