#![allow(dead_code)]
mod menu;

mod keymap;

mod draw;

#[macro_use]
mod macros;

pub struct Item<T> {
    display: String,
    pub value: T,
}

enum Mode {
    Normal,
    Query,
}

pub struct Menu<T> {
    title: Option<String>,
    item_list: Vec<Item<T>>,
    mode: Mode,

    // cursor absolute position (row, col)
    cursor_abs_pos: (u16, u16),

    selection_idx: u16,
    selected: bool,
}
