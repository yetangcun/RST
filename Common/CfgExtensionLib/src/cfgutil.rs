use std::fs;
use std::io;
use std::env;
use std::path::Path;

// use std::io::{self, Read};
// use serde::{Deserialize, Serialize};

use toml::from_str;
use serde_json::from_str as json_from_str;
use serde_yaml::from_str as yaml_from_str;

pub fn rd_relativ_file(file_path: &str) -> String {
    let mut full_path = env::current_dir().unwrap(); // 获取当前的所在的目录路径
    full_path.push(file_path);
    let contents = fs::read_to_string(full_path)
        .expect("Something went wrong reading the file");

    contents
}

pub fn rd_full_file(full_file_path: &str) -> String {
    let contents = fs::read_to_string(full_file_path)
        .expect(&format!("Something went wrong reading the file: {}", full_file_path));

    contents
}

pub trait LoadCfg {
    fn load_cfg<T:serde::de::DeserializeOwned>(cfg_path:&str) -> T;
    // fn load_full_cfg<P: AsRef<Path>>(path: P) -> Result<T, Box<dyn std::error::Error>>;
}

pub struct CfgLoader;

impl LoadCfg for CfgLoader
{
    fn load_cfg<T:serde::de::DeserializeOwned>(file_path: &str) -> T {
        let _str = fs::read_to_string(file_path)
            .expect("Something went wrong reading the file");

        if file_path.ends_with(".toml") {
            from_str(&_str).unwrap()
        } else if file_path.ends_with(".json") {
            json_from_str(&_str).unwrap()
        } else if file_path.ends_with(".yaml") {
            yaml_from_str(&_str).unwrap()
        } else {
            panic!("Unsupported file type")
        }
    }
    
    // fn load_full_cfg<P: AsRef<Path>>(path: P) -> Result<T, Box<dyn std::error::Error>> {
    //     let _str = fs::read_to_string(path).expect("Something went wrong reading the file");
    //     let res: T = json_from_str(&_str)?;
    //     Ok(res)
    // }
}


