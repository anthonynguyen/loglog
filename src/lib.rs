//! # `loglog`
//!
//! loglog aims to be a simple and ergonomic logger that you can drop straight
//! into your code. It uses `env_logger` behind the scenes, so you can use your
//! familiar `log` crate macros.
//!
//! ## Usage
//!
//! Add `loglog` to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! log = "0.3"
//! loglog = "0.2"
//! ```
//!
//! After initializing the logger, the `log` crate macros will be available.
//!
//! ## Example
//!
//! ```rust
//! #[macro_use] extern crate log;
//! extern crate loglog;
//!
//! fn main() {
//!     loglog::build()
//!         .time(Some("%H:%M"))
//!         .init()
//!         .unwrap();
//!
//!     info!("This is an informational {}", "message");
//!     error!("This is not good!");
//! }
//! ```

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

/// The main logger builder.
pub struct LogLogBuilder {
    time_format: String,
    show_time: bool,

    show_target: bool,

    show_colour: bool,
    colour_trace: String,
    colour_debug: String,
    colour_info: String,
    colour_warn: String,
    colour_error: String,

    selector: Option<String>,

    use_stdout: bool
}

/// Create the logger builder with some default values.
pub fn build() -> LogLogBuilder {
    LogLogBuilder {
        time_format: "%T ".to_string(),
        show_time: true,

        show_target: false,

        show_colour: true,
        colour_trace: Colour::Blue.bold().paint(BARE_TRACE).to_string(),
        colour_debug: Colour::Cyan.bold().paint(BARE_DEBUG).to_string(),
        colour_info: Colour::Green.bold().paint(BARE_INFO).to_string(),
        colour_warn: Colour::Yellow.bold().paint(BARE_WARN).to_string(),
        colour_error: Colour::Red.bold().paint(BARE_ERROR).to_string(),

        selector: None,

        use_stdout: false
    }
}

/// Quickly create the builder and start the logger.
pub fn init() -> Result<()> {
    build().init()
}

impl LogLogBuilder {
    /// Start the logger with the constructed settings.
    pub fn init(mut self) -> Result<()> {
        if self.use_stdout && !isatty::stdout_isatty() ||
           !self.use_stdout && !isatty::stderr_isatty() {
            self.show_colour = false;
        }

        let target = if self.use_stdout {
            env_logger::LogTarget::Stdout
        } else {
            env_logger::LogTarget::Stderr
        };

        let rust_log = self.selector.clone()
            .or_else(|| env::var("RUST_LOG").ok())
            .unwrap_or_else(|| "debug".to_string());

        env_logger::LogBuilder::new()
            .format(move |record| self.formatter(record))
            .target(target)
            .parse(&rust_log)
            .init()?;

        Ok(())
    }

    /// Set the timestamp format to be used. If None is given, the timestamp is
    /// disabled.
    ///
    /// By default, the timestamp is displayed, and the format is `%H:%M:%S`.
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

    /// Enable or disable the output of the "log target". For example, for a
    /// module `zmod` in a crate `zcrate`, this would be `zcrate::zmod`.
    ///
    /// By default, the displaying the target is *not* enabled.
    pub fn target(mut self, show: bool) -> Self {
        self.show_target = show;
        self
    }

    /// Enable or disable coloured output. Note that colours are automatically
    /// disabled if the terminal doesn't support them.
    ///
    /// By default, colours are enabled.
    pub fn colour(mut self, show: bool) -> Self {
        self.show_colour = show;
        self
    }

    /// Enable or disable output to `stdout`.
    ///
    /// By default, logs are sent to `stderr`.
    pub fn stdout(mut self, enable: bool) -> Self {
        self.use_stdout = enable;
        self
    }

    /// Specify the `env_logger` selector.
    ///
    /// By default, this will be pulled from the environment variable `RUST_LOG`
    /// , and if that variable doesn't exist, `debug` will be used.
    pub fn select(mut self, selector: &str) -> Self {
        self.selector = Some(selector.to_string());
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
