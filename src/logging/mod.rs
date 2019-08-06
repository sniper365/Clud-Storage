mod log_level;
mod logger;
mod logger_option;

pub use self::log_level::LogLevel;
pub use self::logger::Logger;
pub use self::logger_option::LoggerOption;

use env::Env;
use lazy_static::lazy_static;
use std::convert::TryFrom;
use std::sync::Mutex;

lazy_static! {
    pub static ref LOGGER: Mutex<LoggerOption> = { Mutex::new(Env::logger()) };
}

pub fn log(level: &str, msg: &str) {
    let level = LogLevel::try_from(level.to_string()).unwrap();

    let mut logger = LOGGER.lock().unwrap();

    logger.log(&level, msg)
}

macro_rules! log {
    ($level:expr, $msg:expr) => {
        log($level, $msg)
    };
}

trait Log {
    fn log(&mut self, level: &LogLevel, msg: &str);
}
