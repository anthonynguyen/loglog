extern crate ansi_term;
extern crate chrono;
extern crate env_logger;
#[macro_use] extern crate error_chain;
extern crate log;

mod errors;
pub use errors::*;

pub fn init() -> Result<()> {
    let loglog = LogLog::new();
    env_logger::LogBuilder::new()
        .format(move |record| loglog.formatter(record))
        .filter(None, log::LogLevelFilter::Trace)
        .init()?;

    Ok(())
}

struct LogLog {
    pub trace_prefix: String,
    pub debug_prefix: String,
    pub info_prefix: String,
    pub warn_prefix: String,
    pub error_prefix: String
}

impl LogLog {
    pub fn new() -> Self {
        LogLog {
            trace_prefix: ansi_term::Colour::Blue.paint("[TRACE]").to_string(),
            debug_prefix: ansi_term::Colour::Cyan.paint("[DEBUG]").to_string(),
            info_prefix: ansi_term::Colour::Green.paint("[INFO ]").to_string(),
            warn_prefix: ansi_term::Colour::Yellow.paint("[WARN ]").to_string(),
            error_prefix: ansi_term::Colour::Red.paint("[ERROR]").to_string()
        }
    }

    pub fn formatter(&self, record: &log::LogRecord) -> String {
        let level = match record.level() {
            log::LogLevel::Trace => &self.trace_prefix,
            log::LogLevel::Debug => &self.debug_prefix,
            log::LogLevel::Info => &self.info_prefix,
            log::LogLevel::Warn => &self.warn_prefix,
            log::LogLevel::Error => &self.error_prefix,
        };

        let now = chrono::Local::now();

        format!("{} {} {} - {}", now, level, record.target(), record.args())
    }
}
