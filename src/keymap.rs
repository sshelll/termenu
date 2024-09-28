use std::{cmp, io};

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{Menu, Mode};

impl<T> Menu<T> {
    /// Dispatch a key event.
    /// returns true if the event should cause the menu to exit.
    pub(crate) fn dispatch_key(&mut self, key: KeyEvent) -> io::Result<bool> {
        match key.modifiers {
            KeyModifiers::CONTROL => {
                if key.code == KeyCode::Char('c') {
                    return Ok(true);
                }
                Ok(false)
            }
            KeyModifiers::NONE => self.dispatch_code(key.code),
            _ => Ok(false),
        }
    }

    fn dispatch_code(&mut self, code: KeyCode) -> io::Result<bool> {
        match self.mode {
            Mode::Normal => self.dispatch_normal(code),
            Mode::Query => self.dispatch_query(code),
        }
    }

    fn dispatch_normal(&mut self, code: KeyCode) -> io::Result<bool> {
        match code {
            KeyCode::Up | KeyCode::Char('k') => self.key_up(),

            KeyCode::Down | KeyCode::Char('j') => self.key_down(),

            KeyCode::Esc => return self.key_esc(),

            KeyCode::Enter => {
                self.key_enter();
                return Ok(true);
            }

            KeyCode::Char('/') => {
                self.enter_query_mode()?;
            }
            _ => {}
        };
        Ok(false)
    }

    fn dispatch_query(&mut self, code: KeyCode) -> io::Result<bool> {
        match code {
            KeyCode::Esc => return self.key_esc(),

            KeyCode::Enter => {
                self.key_enter();
                return Ok(true);
            }
            KeyCode::Char(c) => {
                self.query.push(c);
                // a Chinese character takes 3 bytes, however, it only takes 2 cells in most terminals
                self.query_cursor_col += cmp::min(2, c.len_utf8() as u16);
            }
            KeyCode::Backspace => {
                let ch = self.query.pop();
                if let Some(c) = ch {
                    self.query_cursor_col =
                        self.query_cursor_col - cmp::min(2, c.len_utf8() as u16);
                }
            }
            _ => {}
        }
        Ok(false)
    }
}

impl<T> Menu<T> {
    fn key_up(&mut self) {
        self.selection_idx = self.selection_idx.saturating_sub(1);
    }

    fn key_down(&mut self) {
        self.selection_idx = cmp::min(self.selection_idx + 1, self.item_list.len() as u16 - 1);
    }

    fn key_esc(&mut self) -> io::Result<bool> {
        match self.mode {
            Mode::Normal => Ok(true),
            _ => {
                self.enter_normal_mode()?;
                Ok(false)
            }
        }
    }

    fn key_enter(&mut self) {
        self.selected = true;
    }
}
