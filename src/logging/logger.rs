use super::{Log, LogLevel};
use env::Env;
use std::io::Write;

pub struct Logger<W: Write> {
    out: W,
}

impl<W: Write> Logger<W> {
    pub fn new(out: W) -> Self {
        Self { out }
    }
}

impl<W: Write> Log for Logger<W> {
    fn log(&mut self, log_level: &LogLevel, msg: &str) {
        if log_level.is_on() && log_level >= &Env::log_level() {
            write!(&mut self.out, "{}\n", msg).unwrap();
        }
    }
}
