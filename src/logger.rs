//! Logger module with configurable log level, colored output, and file location display

use colored::Colorize;
use log::{Level, LevelFilter, SetLoggerError};
use std::io::{self, Write};

/// Initialize the logger with a specific log level and location display option
///
/// # Arguments
/// * `level` - The maximum log level to display (e.g., Level::Info)
/// * `show_location` - Whether to show file and line number information for each log message
///
/// # Returns
/// * `Result<(), SetLoggerError>` - Ok if successful, Err if logger is already initialized
pub fn init_logger(level: Level, show_location: bool) -> Result<(), SetLoggerError> {
    let level_filter = match level {
        Level::Error => LevelFilter::Error,
        Level::Warn => LevelFilter::Warn,
        Level::Info => LevelFilter::Info,
        Level::Debug => LevelFilter::Debug,
        Level::Trace => LevelFilter::Trace,
    };

    log::set_boxed_logger(Box::new(Logger { show_location }))?;
    log::set_max_level(level_filter);

    Ok(())
}

struct Logger {
    show_location: bool,
}

impl log::Log for Logger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::max_level()
    }

    fn log(&self, record: &log::Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let level = match record.level() {
            Level::Error => "ERROR".red(),
            Level::Warn => "WARN".yellow(),
            Level::Info => "INFO".green(),
            Level::Debug => "DEBUG".blue(),
            Level::Trace => "TRACE".magenta(),
        };

        let file_line = match (self.show_location, record.file(), record.line()) {
            (true, Some(file), Some(line)) => format!("{file}:{line}").dimmed(),
            _ => "".dimmed(),
        };

        let message = format!("[{}] {} {}", level, file_line, record.args());

        // Write to stderr to avoid interfering with stdout output
        let mut stderr = io::stderr();
        writeln!(stderr, "{message}").ok();
    }

    fn flush(&self) {
        // No flushing needed for this simple logger
    }
}
