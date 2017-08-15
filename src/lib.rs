extern crate ansi_term;
extern crate chrono;
extern crate env_logger;
#[macro_use] extern crate error_chain;
extern crate isatty;
extern crate log;

use std::env;

use ansi_term::Colour;

mod errors;
pub use errors::*;

const BARE_TRACE: &'static str = "[TRACE]";
const BARE_DEBUG: &'static str = "[DEBUG]";
const BARE_INFO: &'static str = "[INFO ]";
const BARE_WARN: &'static str = "[WARN ]";
const BARE_ERROR: &'static str = "[ERROR]";

pub struct LogLog {
    time_format: String,
    show_time: bool,

    show_target: bool,

    show_colour: bool,
    colour_trace: String,
    colour_debug: String,
    colour_info: String,
    colour_warn: String,
    colour_error: String
}

pub fn init() -> Result<()> {
    build().init()
}

pub fn init_with(selector: &str) -> Result<()> {
    build().init_with(selector)
}

pub fn build() -> LogLog {
    LogLog {
        time_format: "%T ".to_string(),
        show_time: true,

        show_target: false,

        show_colour: true,
        colour_trace: Colour::Blue.bold().paint(BARE_TRACE).to_string(),
        colour_debug: Colour::Cyan.bold().paint(BARE_DEBUG).to_string(),
        colour_info: Colour::Green.bold().paint(BARE_INFO).to_string(),
        colour_warn: Colour::Yellow.bold().paint(BARE_WARN).to_string(),
        colour_error: Colour::Red.bold().paint(BARE_ERROR).to_string()
    }
}

impl LogLog {
    fn init_(mut self, selector: Option<&str>) -> Result<()> {
        if !isatty::stderr_isatty() {
            self.show_colour = false;
        }

        let rust_log: String = selector
            .map(|s| s.to_string())
            .or_else(|| env::var("RUST_LOG").ok())
            .unwrap_or_else(|| "debug".to_string());

        env_logger::LogBuilder::new()
            .format(move |record| self.formatter(record))
            .parse(&rust_log)
            .init()?;

        Ok(())
    }

    pub fn init(self) -> Result<()> {
        self.init_(None)
    }

    pub fn init_with(self, selector: &str) -> Result<()> {
        self.init_(Some(selector))
    }

    pub fn time(mut self, time_format: Option<&str>) -> Self {
        match time_format {
            Some(format) => {
                self.time_format = format!("{} ", format);
                self.show_time = true;
            },
            None => self.show_time = false
        }

        self
    }

    pub fn target(mut self, show: bool) -> Self {
        self.show_target = show;
        self
    }

    pub fn colour(mut self, show: bool) -> Self {
        self.show_colour = show;
        self
    }

    fn format_time(&self) -> String {
        if self.show_time {
            let ts = chrono::Local::now().format(&self.time_format).to_string();

            if self.show_colour {
                Colour::Purple.bold().paint(ts).to_string()
            } else {
                ts
            }
        } else {
            "".to_string()
        }
    }

    fn format_level(&self, level: log::LogLevel) -> &str {
        if self.show_colour {
            match level {
                log::LogLevel::Trace => &self.colour_trace,
                log::LogLevel::Debug => &self.colour_debug,
                log::LogLevel::Info => &self.colour_info,
                log::LogLevel::Warn => &self.colour_warn,
                log::LogLevel::Error => &self.colour_error
            }
        } else {
            match level {
                log::LogLevel::Trace => BARE_TRACE,
                log::LogLevel::Debug => BARE_DEBUG,
                log::LogLevel::Info => BARE_INFO,
                log::LogLevel::Warn => BARE_WARN,
                log::LogLevel::Error => BARE_ERROR
            }
        }
    }

    fn format_target(&self, target: &str) -> String {
        if self.show_target {
            let ts = format!(" ({})", target);

            if self.show_colour {
                Colour::Purple.bold().paint(ts).to_string()
            } else {
                ts
            }
        } else {
            "".to_string()
        }
    }

    fn formatter(&self, record: &log::LogRecord) -> String {
        format!(
            "{}{}{} {}",
            self.format_time(),
            self.format_level(record.level()),
            self.format_target(record.target()),
            record.args()
        )
    }
}
