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

pub struct TomlCfgLoader;
pub struct JsonCfgLoader;
pub struct YamlCfgLoader;

impl<T> crate::LoadCfg<T> for TomlCfgLoader
where T:serde::de::DeserializeOwned,
{
    fn load_cfg(file_path: &str) -> T {
        let _str = fs::read_to_string(file_path)
            .expect("Something went wrong reading the file");

        toml::from_str(&_str).unwrap()
    }

    fn load_full_cfg<P: AsRef<Path>>(path: P) -> Result<T, Box<dyn std::error::Error>> {
        let _str = fs::read_to_string(path).expect("Something went wrong reading the file");
        let res: T = from_str(&_str)?;
        Ok(res)
    }
}

impl<T> crate::LoadCfg<T> for JsonCfgLoader
where T:serde::de::DeserializeOwned,
{
    fn load_cfg(file_path: &str) -> T {
        let _str = fs::read_to_string(file_path)
            .expect("Something went wrong reading the file");

        json_from_str(&_str).unwrap()
    }
    
    fn load_full_cfg<P: AsRef<Path>>(path: P) -> Result<T, Box<dyn std::error::Error>> {
        let _str = fs::read_to_string(path).expect("Something went wrong reading the file");
        let res: T = json_from_str(&_str)?;
        Ok(res)
    }
}

impl<T> crate::LoadCfg<T> for YamlCfgLoader 
where T:serde::de::DeserializeOwned,
{
    fn load_cfg(file_path: &str) -> T {
        let _str = fs::read_to_string(file_path)
            .expect("Something went wrong reading the file");

        yaml_from_str(&_str).unwrap()
    }

    fn load_full_cfg<P: AsRef<Path>>(path: P) -> Result<T, Box<dyn std::error::Error>> {
        let _str = fs::read_to_string(path).expect("Something went wrong reading the file");
        let res: T = yaml_from_str(&_str)?;
        Ok(res)
    }
}

// pub fn rd_file<T>(file_path: &str) -> &T {
//     let contents = fs::read_to_string(file_path)
//         .expect("Something went wrong reading the file");
//     &contents
// }

