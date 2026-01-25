#[macro_export]
macro_rules! log_err_recursive {
    ($err:expr, $($arg:tt)*) => {{
        use colored::Colorize;
        eprintln!("{}: {}", "Error".red().bold(), format!($($arg)*));

        let mut source = std::error::Error::source(&$err);
        while let Some(cause) = source {
            eprintln!("  {} {}", "╰─>".red(), cause);
            source = std::error::Error::source(cause);
        }
    }};
}
