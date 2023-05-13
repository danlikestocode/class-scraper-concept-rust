use simple_logger::SimpleLogger;

pub fn init_logger() {
    SimpleLogger::new().init().unwrap();
    log::debug!("Initialized logger")
}
