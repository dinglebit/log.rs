use chrono::Utc;
use dinglebit_terminal::style;
use hostname;
use log::{Level, LevelFilter, Metadata, Record};

pub struct ColorLogger {
    host: String,
    verbose: bool,
}

impl ColorLogger {
    pub fn init(filter: LevelFilter, verbose: bool) {
        let s = Self {
            host: hostname::get().unwrap().into_string().unwrap(),
            verbose: verbose,
        };
        log::set_boxed_logger(Box::new(s)).unwrap();
        log::set_max_level(filter);
    }
}

impl log::Log for ColorLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= log::max_level()
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let mut l = match self.verbose {
                true => style!(
                    "{:5} | {} | {} | {} | {}:{}:{} | {}",
                    record.level(),
                    Utc::now(),
                    self.host,
                    record.target(),
                    record.module_path().unwrap_or(""),
                    record.file().unwrap_or(""),
                    record.line().unwrap_or(0),
                    record.args()
                ),
                false => style!("{:5} | {} | {}", record.level(), Utc::now(), record.args()),
            };
            match record.metadata().level() {
                Level::Error => l.red(),
                Level::Warn => l.magenta(),
                Level::Info => l.white(),
                Level::Debug => l.cyan(),
                Level::Trace => l.gray(),
            };
            println!("{}", l);
        }
    }

    fn flush(&self) {
        println!("flush");
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
