#[derive(PartialEq, PartialOrd)]
pub enum LogLevel {
    Off,
    Warn,
    Info,
    Trace,
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

        println!("{}", message);
    }
}
