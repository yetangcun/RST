use ftail::{ansi_escape::TextStyling, Config, Ftail};
use log::{Level, LevelFilter, Log};

pub fn init_logger() {
    let tz:ftail::Tz = "Asia/Shanghai".parse().unwrap();
    Ftail::new()
    .timezone(tz)
    // .console(LevelFilter::Error)
    // .filter_levels(vec![
    //     Level::Trace,
    //     Level::Debug,
    //     Level::Info,
    //     Level::Error,
    //     Level::Warn
    // ])
    .custom(
        |config: ftail::Config| Box::new(CustmLogger { config }) as Box<dyn Log + Send + Sync>,
        LevelFilter::Trace,
    )
    // .datetime_format("%Y-%m-%d %H:%M:%S%.3f")
    .daily_file("ftlogs", LevelFilter::Trace)
    .init()
    .unwrap();
}

// the custom logger implementation
struct CustmLogger {
    config: Config,
}

impl Log for CustmLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        // if self.config.level_filter == LevelFilter::Off {
        //     return true;
        // }

        metadata.level() <= self.config.level_filter
    }

    fn log(&self, record: &log::Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let time = chrono::Local::now()
            .format("%Y-%m-%d %H:%M:%S%.3f")  // &self.config.datetime_format
            .to_string();

        println!(
            "{} [{}] {}:{}: {}", 
            time.black(), 
            record.level().bold(), 
            record.file().unwrap(), 
            record.line().unwrap_or(0), 
            record.args()
        );
    }

    fn flush(&self) {}
}

pub fn info (info: &str) {
    log::info!("{}", info);
}

pub fn warn (info: &str) {
    log::warn!("{}", info);
}

pub fn err (info: &str) {
    log::error!("{}", info);
}

pub fn debug (info: &str) {
    log::debug!("{}", info);
}