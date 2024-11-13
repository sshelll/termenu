use clap::Parser;
use std::io::{self, BufRead};
use termenu::{FontShape, FontStyle, Item};

#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    /// name of this operation, displays at the beginning.
    #[clap(short, long)]
    name: Option<String>,

    /// max height of the menu, should be a percentage in range (0, 1], otherwise it will be
    /// ignored
    #[clap(short, long, value_parser=validate_max_height)]
    max_height: Option<f32>,

    /// disable escape input, by default \n, \t, etc will be displayed as \\n, \\t, etc in single
    /// line
    #[clap(short, long)]
    disable_escape: bool,
}

fn validate_max_height(input: &str) -> Result<f32, String> {
    match input.parse::<f32>() {
        Ok(n) if n > 0.0 && n <= 1.0 => Ok(n),
        _ => Err("max height should be a percentage in range (0, 1]".to_string()),
    }
}

macro_rules! quit_now {
    ($content:expr, $($arg:tt)*) => {{
        eprintln!($content, $($arg)*);
        std::process::exit(1);
    }};
}

fn main() {
    let args = Args::parse();
    let mut menu = termenu::Menu::new().unwrap_or_else(|e| quit_now!("Error: {}", e));
    args.name.map(|name| menu.set_title(&name));
    args.max_height.map(|percent| menu.set_max_height(percent));

    let mut colorscheme = termenu::ColorScheme::default();
    colorscheme
        .set_title_style(
            FontStyle::default()
                .set_shape(FontShape::Bold | FontShape::Underline)
                .build(),
        )
        .set_query_style(FontStyle::default().set_shape(FontShape::Italic).build())
        .set_more_tag_style(
            FontStyle::default()
                .set_fg_color(colored::Color::Magenta)
                .build(),
        );
    menu.set_colorscheme(colorscheme);
    menu.enable_print_result(false);

    let stdin = io::stdin();
    loop {
        let mut buf = String::new();
        match stdin.lock().read_line(&mut buf) {
            Err(e) => quit_now!("Error: {}", e),
            Ok(0) => break, //EOF
            _ => {
                buf = buf.trim_end_matches('\n').to_string();
                if !args.disable_escape {
                    buf = buf.escape_default().to_string();
                }
                menu.add(Item::new(&buf, ()))
            }
        };
    }

    let selection = menu
        .select_item()
        .unwrap_or_else(|e| quit_now!("Error: {}", e));
    if let Some(item) = selection {
        print!("{}", item.alias)
    }
}

