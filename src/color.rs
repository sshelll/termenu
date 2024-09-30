use std::ops;

use crate::{ColorScheme, FontShape, FontStyle, Item, Menu};
use colored::*;

pub(crate) fn colorize(s: &str, style: &FontStyle) -> String {
    let mut colored = s.to_string();

    // shape
    if let Some(shape) = style.shape {
        if shape & FontShape::Bold != 0 {
            colored = colored.bold().to_string();
        }
        if shape & FontShape::Italic != 0 {
            colored = colored.italic().to_string();
        }
        if shape & FontShape::Underline != 0 {
            colored = colored.underline().to_string();
        }
    }

    // fg
    if let Some(fg_color) = style.fg_color {
        colored = colored.color(fg_color).to_string();
    } else if let Some(fg_color_256) = style.fg_color_256 {
        colored = colored
            .truecolor(fg_color_256.0, fg_color_256.1, fg_color_256.2)
            .to_string();
    }

    // bg
    if let Some(bg_color) = style.bg_color {
        colored = colored.on_color(bg_color).to_string();
    } else if let Some(bg_color_256) = style.bg_color_256 {
        colored = colored
            .on_truecolor(bg_color_256.0, bg_color_256.1, bg_color_256.2)
            .to_string();
    }

    colored
}

impl<T> Menu<T> {
    pub fn set_colorscheme(&mut self, cs: ColorScheme) -> &mut Self {
        self.colorscheme = cs;
        self
    }
}

impl<T> Item<T> {
    pub(crate) fn get_colored_alias_for_normal_mode(&self, colorscheme: &ColorScheme) -> String {
        colorize(&self.alias, &colorscheme.items)
    }

    pub(crate) fn get_colored_alias_for_query_mode(&self, colorscheme: &ColorScheme) -> String {
        if self.matched_indices.is_none() || self.matched_indices.as_ref().unwrap().is_empty() {
            return colorize(&self.alias, &colorscheme.items);
        }
        let mut display = String::new();
        if let Some(indices) = self.matched_indices.as_ref() {
            let mut last = 0;
            for idx in indices.iter() {
                display.push_str(&self.alias[last..*idx]);
                let ch = colorize(&self.alias[*idx..=*idx], &colorscheme.matched);
                display.push_str(&ch);
                last = idx + 1;
            }
            display.push_str(&self.alias[last..]);
        } else {
            display.push_str(&self.alias);
        }
        display
    }
}

impl ops::BitOr for FontShape {
    type Output = Self; // 使用 u8 表示组合的位标志

    fn bitor(self, rhs: Self) -> Self {
        unsafe { std::mem::transmute((self as u8) | (rhs as u8)) }
    }
}

impl ops::BitAnd for FontShape {
    type Output = u8;

    fn bitand(self, rhs: FontShape) -> u8 {
        (self as u8) & (rhs as u8)
    }
}

impl FontStyle {
    fn default_matched() -> Self {
        Self {
            shape: None,
            fg_color: Some(Color::Yellow),
            bg_color: None,
            fg_color_256: None,
            bg_color_256: None,
        }
    }

    pub fn set_shape(&mut self, shape: FontShape) -> &mut Self {
        self.shape = Some(shape);
        self
    }

    pub fn set_fg_color(&mut self, color: Color) -> &mut Self {
        self.fg_color = Some(color);
        self
    }

    pub fn set_bg_color(&mut self, color: Color) -> &mut Self {
        self.bg_color = Some(color);
        self
    }

    pub fn set_fg_color_256(&mut self, color: (u8, u8, u8)) -> &mut Self {
        self.fg_color_256 = Some(color);
        self
    }

    pub fn set_bg_color_256(&mut self, color: (u8, u8, u8)) -> &mut Self {
        self.bg_color_256 = Some(color);
        self
    }
}

impl ColorScheme {
    pub fn new() -> Self {
        Self::default()
    }

    /// set the style of the title
    pub fn set_title_style(&mut self, style: FontStyle) -> &mut Self {
        self.title = style;
        self
    }

    /// set the style of the query, which is the input of the user in query mode
    pub fn set_query_style(&mut self, style: FontStyle) -> &mut Self {
        self.query = style;
        self
    }

    /// set the style of the items, which is the list of items which are displayed in the menu and
    /// not chosen yet
    pub fn set_items_style(&mut self, style: FontStyle) -> &mut Self {
        self.items = style;
        self
    }

    /// set the style of the chosen line, which is the line that the cursor is pointing to
    pub fn set_chosen_ln_style(&mut self, style: FontStyle) -> &mut Self {
        self.chosen_ln = style;
        self
    }

    /// set the style of the matched part of the items in query mode
    pub fn set_matched_style(&mut self, style: FontStyle) -> &mut Self {
        self.matched = style;
        self
    }

    /// set the style of the '---more---' tag, which is displayed at the bottom of the menu when there are
    /// more items than the screen can display
    pub fn set_more_tag_style(&mut self, style: FontStyle) -> &mut Self {
        self.more_tag = style;
        self
    }
}

impl Default for ColorScheme {
    fn default() -> Self {
        Self {
            title: FontStyle::default(),
            query: FontStyle::default(),
            items: FontStyle::default(),
            matched: FontStyle::default_matched(),
            chosen_ln: FontStyle::default_matched(),
            more_tag: FontStyle::default(),
        }
    }
}
