use flexi_logger::{
    Logger,
    FileSpec,
    WriteMode,
    Record,
    Duplicate,
    Cleanup,
    Criterion,
    DeferredNow,
    Naming,
    Age
};
use log::*;
use std::io::{self, Write, Error};


pub fn init_logger() -> Result<(), Error>{
    let logger = Logger::try_with_str("trace")
    .unwrap()
    .log_to_file(
        FileSpec::default()
            .directory("logs")
            .basename("log")
            .suffix("txt")
    )
    .append()  // 日志信息追加到一个文件里面了
    .rotate(
        Criterion::AgeOrSize(Age::Day, 1024*1024*6), // 日志文件交替周期 每天|大小超过10
        // Criterion::Size(1024*1024*100), // 100MB 按大小切割
        // Naming::Timestamps,   // 文件命名
        Naming::TimestampsCustomFormat { 
            current_infix: Some("curr"),
            format: "%Y%m%d"   // %Y%m%d%H%M%S
        },
        Cleanup::KeepLogFiles(30), // 保留30个文件 Cleanup::Never 保留所有文件
    )
    .append()
    .format_for_files(custom_format) // .format(custom_format)
    // .format_for_files(flexi_logger::detailed_format)
    // .duplicate_to_stdout(Duplicate::Trace)
    .write_mode(WriteMode::Direct)  // BufferAndFlush
    .duplicate_to_stderr(Duplicate::Trace) // Error Warn Info Debug Trace None, All
    .start()
    .unwrap();

    Ok(())
}

fn custom_format(
    w: &mut dyn Write,
    now: &mut DeferredNow,
    record: &Record
) -> std::io::Result<()> {
    write!(
        w, 
        "[{}] {} {}:{}: {}", 
        now.now().format("%Y-%m-%d %H:%M:%S%.3f"), 
        record.level(), 
        // record.module_path().unwrap_or("<unnamed>"),
        // record.file().unwrap_or("<unnamed>"),
        record.target(),
        record.line().unwrap_or(0),
        record.args()
    )
}


pub fn err(err: &str) {
    error!("{}", err);
}

pub fn warn(warn: &str) {
    warn!("{}", warn);
}

pub fn info(info: &str) {
    info!("{}", info);
}

pub fn debug(debug: &str) {
    debug!("{}", debug);
}