use termenu::*;

macro_rules! quit_now {
    ($content:expr, $($arg:tt)*) => {
        eprintln!($content, $($arg)*);
        std::process::exit(1);
    };
}

fn main() {
    let mut menu = Menu::new().unwrap_or_else(|e| {
        quit_now!("Error: {}", e);
    });

    // build item list
    let mut item_list = Vec::new();
    for i in 1..=100 {
        item_list.push(Item::new(format!("{}th item", i).as_str(), i));
    }

    // build colorscheme, you can skip this step to use the default colorscheme
    // NOTE: sometimes your terminal might not support all the colors, so you may need to adjust
    // the colorscheme
    let mut colorscheme = termenu::ColorScheme::default();
    colorscheme
        .set_title_style(
            *FontStyle::default()
                .set_shape(FontShape::Bold | FontShape::Underline)
                .set_fg_color(colored::Color::Green),
        )
        .set_query_style(*FontStyle::default().set_shape(FontShape::Italic))
        .set_chosen_ln_style(
            *FontStyle::default()
                .set_shape(FontShape::Underline)
                .set_fg_color(colored::Color::Black)
                .set_bg_color_256((215, 255, 0)),
        )
        .set_more_tag_style(*FontStyle::default().set_fg_color(colored::Color::Magenta));

    // run
    let selection = menu
        .set_title("test selection:")
        .add_list(item_list)
        .set_colorscheme(colorscheme)
        .select() // this is the menu entry
        .unwrap_or_else(|e| {
            quit_now!("Error: {}", e);
        });

    // handle selection
    if let Some(selection) = selection {
        println!("You selected: {}", selection.value);
    } else {
        println!("You didn't select anything");
    }
}
