#[macro_export]
macro_rules! term_print {
    ($content:expr) => {
        term_exec_stdout!(Print($content));
    };
}

#[macro_export]
macro_rules! term_printf {
    ($content:expr, $($arg:tt)*) => {
        term_exec_stdout!(crossterm::style::Print(format!($content, $($arg)*)));
    };
}

#[macro_export]
macro_rules! term_cursor_down {
    ($content:expr) => {
        term_exec_stdout!(crossterm::cursor::MoveToNextLine($content));
    };
}

#[macro_export]
macro_rules! term_cursor_move {
    ($row:expr, $col:expr) => {
        term_exec_stdout!(crossterm::cursor::MoveTo($col, $row));
    };
}

#[macro_export]
macro_rules! term_clear_from_cursor_down {
    () => {
        term_exec_stdout!(crossterm::terminal::Clear(
            crossterm::terminal::ClearType::FromCursorDown
        ));
    };
}

#[macro_export]
macro_rules! term_exec_stdout {
    ($content:expr) => {
        crossterm::execute!(std::io::stdout(), $content)?;
    };
}

#[macro_export]
macro_rules! quit_now {
    ($content:expr, $($arg:tt)*) => {
        eprintln!($content, $($arg)*);
        std::process::exit(1);
    };
}

#[macro_export()]
macro_rules! ignore_io_error {
    ($body:expr) => {
        let _ = (|| -> std::io::Result<()> {
            $body
            Ok(())
        })();
    };
}
