use std::fmt;

#[derive(PartialEq, PartialOrd)]
pub enum LogLevel {
    Off,
    Warn,
    Info,
    Trace,
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let log_level = match *self {
            Self::Warn => "WARNING",
            Self::Info => "INFO",
            Self::Trace => "TRACE",
            _ => "",
        };
        write!(f, "{}", log_level)
    }
}

pub struct Logger {
    log_level: LogLevel,
}

impl Logger {
    pub fn init(log_level: LogLevel) -> Logger {
        Logger { log_level }
    }

    pub fn warn(&self, message: &str) {
        self.log(message, LogLevel::Warn)
    }

    pub fn info(&self, message: &str) {
        self.log(message, LogLevel::Info)
    }

    pub fn trace(&self, message: &str) {
        self.log(message, LogLevel::Trace)
    }

    fn log(&self, message: &str, log_level: LogLevel) {
        if self.log_level < log_level {
            return;
        };

        println!("{}: {}", log_level, message);
    }
}
