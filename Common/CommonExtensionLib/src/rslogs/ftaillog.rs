use ftail::{ansi_escape::TextStyling, Config, Ftail};
use log::{Level, LevelFilter, Log};

use std::{fs, env};
use std::io::{Write};
use std::fs::OpenOptions;

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
    // .daily_file("ftlogs", LevelFilter::Trace)
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

    fn log(&self, record: &log::Record) 
    {
        if !self.enabled(record.metadata()) {
            return;
        }

        // let time = chrono::Local::now()   // &self.config.datetime_format
        //     .format("%y-%m-%d %H:%M:%S%.3f")
        //     .to_string();
        
        // println!(
        //     "{} [{}] {}:{}: {}", 
        //     time.black(), 
        //     record.level().bold(), 
        //     record.file().unwrap(), 
        //     record.line().unwrap_or(0), 
        //     record.args()
        // );
        
        let time = chrono::Local::now(); // 获取当前时间
        let mut curr_dir = env::current_dir().unwrap().display().to_string(); // 获取当前所在目录
        curr_dir.push_str("/logs"); // 构造实际日志目录

        // 检查日志目录是否已存在
        if fs::metadata(&curr_dir).is_err() { 
            fs::create_dir(&curr_dir).expect("create file failed");
        }

        // 检查日志文件是否已存在
        let fl_pth = format!("{}/log{}.log", curr_dir, time.format("%Y%m%d")); // 每天一个文件
        if fs::metadata(&fl_pth).is_err() { 
            fs::File::create(&fl_pth).expect("create file failed");
        }
        
        let tm_str = time.format("%y-%m-%d %H:%M:%S%.3f"); // 日志内容时间部分的格式

        // 完整日志内容格式
        let info = format!(
            "{} [{}] {}:{}: {}", 
            tm_str, 
            record.level(),
            record.file().unwrap(), 
            record.line().unwrap_or(0), 
            record.args()
        );

        // 以追加的方式打开
        let mut fl = OpenOptions::new()
        .append(true)
        .open(&fl_pth)
        .expect("open file failed");

        let _ = fl.write(format!("{info}\n").as_bytes());  // 正式写入操作

        let _ = fl.flush(); // 也可以不刷新;
    }

    fn flush(&self) {}
}

// pub fn info (info: &str) {
//     log::info!("{}", info);
// }

// pub fn warn (info: &str) {
//     log::warn!("{}", info);
// }

// pub fn err (info: &str) {
//     log::error!("{}", info);
// }

// pub fn debug (info: &str) {
//     log::debug!("{}", info);
// }