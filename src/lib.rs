#![allow(dead_code)]
mod core;
mod draw;
mod keymap;
mod mode;

#[macro_use]
mod macros;

pub struct Item<T> {
    alias: String,
    pub value: T,
}

enum Mode {
    Normal,
    Query,
}

pub struct Menu<T> {
    title: Option<String>,
    item_list: Vec<Option<Item<T>>>,
    mode: Mode,

    // original cursor absolute position (row, col)
    cursor_abs_pos: (u16, u16),

    selection_idx: u16,
    selected: bool,

    query: String,
    query_cursor_col: u16,
}
