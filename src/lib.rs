extern crate env_logger;
#[macro_use] extern crate error_chain;
extern crate log;

mod errors;
pub use errors::*;

pub fn init() -> Result<()> {
	env_logger::LogBuilder::new()
		.format(formatter).filter(None, log::LogLevelFilter::Info)
		.init()?;

	Ok(())
}

fn formatter(record: &log::LogRecord) -> String {
	format!("{} - {}", record.level(), record.args())
}
