use std::{cmp, io};

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{term_exec_stdout, Menu, Mode};

impl<T> Menu<T> {
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
        match code {
            KeyCode::Up | KeyCode::Char('k') => self.key_up(),

            KeyCode::Down | KeyCode::Char('j') => self.key_down(),

            KeyCode::Esc => return self.key_esc(),

            KeyCode::Enter => {
                self.selected = true;
                return Ok(true);
            }

            KeyCode::Char('/') => {
                unimplemented!()
            }
            _ => {}
        };
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
                self.normal_mode()?;
                Ok(false)
            }
        }
    }

    pub(crate) fn normal_mode(&mut self) -> io::Result<()> {
        self.mode = Mode::Normal;
        term_exec_stdout!(crossterm::cursor::Hide);
        Ok(())
    }
}
