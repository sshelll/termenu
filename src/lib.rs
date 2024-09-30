#![allow(dead_code)]

mod core;
mod draw;
mod keymap;
mod mode;
mod query;

#[macro_use]
mod macros;

pub struct Item<T> {
    alias: String,
    pub value: T,
    pub(crate) score: Option<i64>,
    pub(crate) matched_indices: Option<Vec<usize>>,
}

enum Mode {
    Normal,
    Query,
}

pub struct Menu<T> {
    title: Option<String>,

    // use option just for take ownership of item, it'll never be None
    item_list: Vec<Option<Item<T>>>,

    mode: Mode,

    // original cursor absolute position (row, col)
    cursor_abs_pos: (u16, u16),
    max_row: u16,

    selection_idx: u16,
    selected: bool,

    // query mode fields
    query: String,
    query_cursor_col: u16,
    matched_item_indices: Vec<usize>,
    insert_pos: usize,

    scroll_offset: u16,
}
