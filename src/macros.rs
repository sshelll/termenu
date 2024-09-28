macro_rules! term_print {
    ($content:expr) => {
        term_exec!(crossterm::style::Print($content));
    };
}
pub(crate) use term_print;

macro_rules! term_printf {
    ($content:expr, $($arg:tt)*) => {
        term_exec!(crossterm::style::Print(format!($content, $($arg)*)));
    };
}
pub(crate) use term_printf;

macro_rules! term_cursor_down {
    ($content:expr) => {
        term_exec!(crossterm::cursor::MoveToNextLine($content));
    };
}
pub(crate) use term_cursor_down;

macro_rules! term_cursor_col {
    ($content:expr) => {
        term_exec!(crossterm::cursor::MoveToColumn($content));
    };
}
pub(crate) use term_cursor_col;

macro_rules! term_cursor_move {
    ($row:expr, $col:expr) => {
        term_exec!(crossterm::cursor::MoveTo($col, $row));
    };
}
pub(crate) use term_cursor_move;

macro_rules! term_clear_from_cursor_down {
    () => {
        term_exec!(crossterm::terminal::Clear(
            crossterm::terminal::ClearType::FromCursorDown
        ));
    };
}
pub(crate) use term_clear_from_cursor_down;

macro_rules! term_exec {
    ($content:expr) => {
        crossterm::execute!(std::io::stdout(), $content)?;
    };
}
pub(crate) use term_exec;

macro_rules! ignore_io_error {
    ($body:expr) => {
        let _ = (|| -> std::io::Result<()> {
            $body;
            Ok(())
        })();
    };
}
pub(crate) use ignore_io_error;
