use ftail::{Config,Ftail};
use log::{Level, LevelFilter, Log};
use ftail::ansi_escape::TextStyling;

pub fn init_logger() {
    Ftail::new()
    .timezone(ftail::Tz::UTC)
    .datetime_format("%Y-%d-%m %H:%M:%S%.3f")
    .console(LevelFilter::Off)
    .filter_levels(vec![
        Level::Trace,
        Level::Debug,
        Level::Info,
        Level::Error,
    ])
    .custom(
        |config: ftail::Config| Box::new(CustmLogger { config }) as Box<dyn Log + Send + Sync>,
        LevelFilter::Trace,
    )
    // .filter_targets(vec!["foo"])
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
        if self.config.level_filter == LevelFilter::Off {
            return true;
        }

        metadata.level() <= self.config.level_filter
    }

    fn log(&self, record: &log::Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let time = chrono::Local::now()
            .format(&self.config.datetime_format)
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