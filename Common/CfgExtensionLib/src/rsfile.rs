use std::fs;
use std::env;
use std::path::Path;
use std::io::{self, Read};

pub fn rd_relativ_file(file_path: &str) -> String {
    let mut full_path = env::current_dir().unwrap(); // 获取当前的所在的目录路径
    full_path.push(file_path);
    let contents = fs::read_to_string(full_path)
        .expect("Something went wrong reading the file");

    contents
}


pub fn rd_full_file(file_path: &str) -> String {
    let contents = fs::read_to_string(file_path)
        .expect(&format!("Something went wrong reading the file: {}", file_path));

    contents
}