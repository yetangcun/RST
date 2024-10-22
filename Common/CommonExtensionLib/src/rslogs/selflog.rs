use std::{io, fs, env};
use std::io::Write;
use std::fs::OpenOptions;
use chrono::{Local, DateTime, Utc};
// use log::{Record};

pub fn log_info(msg: &str) -> Result<bool, std::io::Error> {
    let mut curr_dir = env::current_dir().unwrap().display().to_string(); // 获取当前所在目录
    curr_dir.push_str("/logs");
    if fs::metadata(&curr_dir).is_err() { 
        fs::create_dir(&curr_dir).expect("create file failed");
    }

    let now_tm = Local::now();
    let file_path = format!("{}/log{}.log", curr_dir, now_tm.format("%Y%m%d")); // 每天一个文件
    
    if fs::metadata(&file_path).is_err() { 
        fs::File::create(&file_path).expect("create file failed");
    }
    
    // 把日志内容写入到文件中 并换行
    let now_str = now_tm.format("%y-%m-%d %H:%M:%S").to_string(); // 补充时间内容
    let contents = format!("{now_str} [INFO] {msg}");
    let mut fl = OpenOptions::new()
        .write(true)
        .append(true)
        .open(file_path)
        .expect("open file failed");
    writeln!(fl, "{contents}").expect("write file failed");

    // fs::write(file_path, contents).expect("write file failed"); // 这个是覆盖,不是追加
    
    // 刷新fl
    let _ = fl.flush(); // 也可以不刷新;
    
    Ok(true)
}


pub fn log_err(err: &str) -> Result<bool, std::io::Error> {
    let mut curr_dir = env::current_dir().unwrap().display().to_string(); // 获取当前所在目录
    curr_dir.push_str("/logs");
    if fs::metadata(&curr_dir).is_err() { 
        fs::create_dir(&curr_dir).expect("create file failed");
    }

    let now_tm = Local::now();
    let file_path = format!("{}/log{}.log", curr_dir, now_tm.format("%Y%m%d"));
    
    if fs::metadata(&file_path).is_err() { 
        fs::File::create(&file_path).expect("create file failed");
    }
    
    // 把日志内容写入到文件中 并换行
    // let rd = Record::builder()
    // .level(log::Level::Info)
    // .target("selflog.rs")
    // .build();
    // let log_fl = format!("{}:{}", rd.file().unwrap_or(""), rd.line().unwrap_or(0));

    let tm_str = now_tm.format("%y-%m-%d %H:%M:%S").to_string();
    let contents = format!("{tm_str} [ERR] {err}");

    // let mut fl = OpenOptions::new()
    //     .write(true)
    //     .append(true)
    //     .open(&file_path)
    //     .expect("open file failed");
    // writeln!(fl, "{contents}").expect("write file failed");

    let mut fl = OpenOptions::new()
        .append(true)
        .open(&file_path)
        .expect("open file failed");
    let _ = fl.write(format!("{contents}\n").as_bytes());

    // fs::write(file_path, contents).expect("write file failed"); // 这个是覆盖,不是追加
    
    // 刷新
    let _ = fl.flush(); // 也可以不刷新;
    
    Ok(true)
}