extern crate ansi_term;
extern crate env_logger;
#[macro_use] extern crate error_chain;
extern crate log;

mod errors;
pub use errors::*;

pub fn init() -> Result<()> {
    env_logger::LogBuilder::new()
        .format(formatter)
        .filter(None, log::LogLevelFilter::Trace)
        .init()?;

    Ok(())
}

fn formatter(record: &log::LogRecord) -> String {
    let level = match record.level() {
        log::LogLevel::Trace => ansi_term::Colour::Blue.paint("[TRACE]"),
        log::LogLevel::Debug => ansi_term::Colour::Cyan.paint("[DEBUG]"),
        log::LogLevel::Info => ansi_term::Colour::Green.paint("[INFO] "),
        log::LogLevel::Warn => ansi_term::Colour::Yellow.paint("[WARN] "),
        log::LogLevel::Error => ansi_term::Colour::Red.paint("[ERROR]"),
    };

    format!("{} {} - {}", level, record.target(), record.args())
}
