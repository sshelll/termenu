//! A fzf-like library for terminal applications
//!
//! You can use it to build a menu for selecting items from a list in terminal with ui.
//! It supports both normal mode and query mode, and you can customize the colorscheme.
//!
//! ## Demo
//! ![demo](https://github.com/sshelll/assets/blob/master/termenu/termenu.jpg?raw=true)
//!
//! ## Key Mapping
//! - `j`/`k` or `down`/`up` to move the cursor
//! - `enter` to select the item
//! - `/` to enter query mode just like vim
//! - `ctrl-n`/`ctrl-p` to move the cursor in query mode
//! - `esc` to exit query mode or the menu
//! - `ctrl-c` to exit the menu
//!
//! ## Examples
//!
//! ```no_run
//! let mut menu = termenu::Menu::new().unwrap();
//! let mut item_list = Vec::new();
//! for i in 1..=10 {
//!    item_list.push(Item::new(format!("{}th item", i).as_str(), i));
//! }
//! let selection = menu
//!     .set_title("test selection:")
//!     .add_list(item_list)
//!     .select()
//!     .unwrap();
//! if let Some(selection) = selection {
//!    println!("You selected: {}", selection);
//! }
//! ```
//!
//! Check the examples folder for more details.

#![allow(dead_code)]
use colored::Color;
use fuzzy_matcher::skim::SkimMatcherV2;
use once_cell::sync::OnceCell;
mod color;
mod core;
mod draw;
mod keymap;
mod mode;
mod query;

#[allow(unused_macros)]
#[macro_use]
mod macros;

/// item in the menu
///
/// You can only store the same type of value in the menu
pub struct Item<T>
where
    T: Send + Sync,
{
    alias: String,
    pub value: T,
    pub(crate) score: Option<i64>,
    pub(crate) matched_indices: Option<Vec<usize>>,
}

enum Mode {
    Normal,
    Query,
}

/// the menu itself
pub struct Menu<T>
where
    T: Send + Sync,
{
    colorscheme: ColorScheme,

    title: Option<String>,

    // use option just for take ownership of item, it'll never be None
    item_list: Vec<Item<T>>,

    mode: Mode,

    // original cursor absolute position (row, col)
    cursor_abs_pos: (u16, u16),
    max_row: u16,

    selection_idx: u16,
    selected: bool,

    // query mode fields
    fuzzy_matcher: OnceCell<SkimMatcherV2>,
    query: String,
    matched_item_indices: Vec<usize>,
    insert_idx: usize,

    scroll_offset: u16,
    max_height_percent: f32,

    show_end_tag: bool,

    rayon_pool: OnceCell<rayon::ThreadPool>,
}

#[derive(Clone, Copy)]
/// Italic, Bold, Underline
pub enum FontShape {
    Italic = 1,
    Bold = 1 << 1,
    Underline = 1 << 2,
}

#[derive(Default, Clone, Copy)]
/// style of font
///
/// Examples:
/// 1. set more than one shape at the same time
/// ```no_run
/// let mut style = FontStyle::default();
/// style.set_shape(FontShape::Bold | FontShape::Italic);
/// ```
///
/// 2. set the fg/bg color by name
/// ```no_run
/// let mut style = FontStyle::default();
/// style.set_fg_color(Color::Red);
/// style.set_bg_color(Color::Blue);
/// ```
///
/// 3. set the fg/bg color by 256-color
/// ```no_run
/// let mut style = FontStyle::default();
/// style.set_fg_color_256((255, 0, 0));
/// style.set_bg_color_256((0, 0, 255));
/// ```
///
/// **Note: if you set the fg/bg color by name, the 256-color will be ignored**
pub struct FontStyle {
    shape: Option<FontShape>,
    fg_color: Option<Color>,
    bg_color: Option<Color>,
    fg_color_256: Option<(u8, u8, u8)>,
    bg_color_256: Option<(u8, u8, u8)>,
}

#[derive(Clone, Copy)]
/// colorscheme of the menu
///
/// combination of [`FontStyle`]
pub struct ColorScheme {
    title: FontStyle,
    query: FontStyle,
    items: FontStyle,
    matched: FontStyle,
    chosen_ln: FontStyle,
    more_tag: FontStyle,
}
