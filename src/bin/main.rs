use dinglebit_log::ColorLogger;
use log;
use log::{debug, error, info, trace, warn, LevelFilter};

fn main() {
    ColorLogger::init(LevelFilter::Trace, true);

    trace!("trace message");
    debug!("debug message");
    info!("info message");
    warn!("warn message");
    error!("error message");
}
