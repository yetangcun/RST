use std::{io, fs, env};
use std::path::Path;
use serde::{Deserialize, Serialize};
use CfgExtensionLib::cfgutil::{CfgLoader, LoadCfg};

use CommonExtensionLib::{utils::{logutil}};

use fs_mod::{cfg_toml, cfg_json, cfg_yaml};

fn main() {
    let contents = CfgExtensionLib::cfgutil::rd_relativ_file("tst.txt");
    println!("file contents: {}, demo!", contents);

    let toml_obj:cfg_toml = CfgLoader::load_cfg::<cfg_toml>("toml.toml"); // println!("cfg_toml: {:?}", cfg_toml);
    println!("api_addr: {}", toml_obj.basecfg.third_api_addr);

    let json_obj:cfg_json = CfgLoader::load_cfg::<cfg_json>("json.json");
    println!("db host: {}, port: {}", json_obj.dbcfg.host, json_obj.dbcfg.port);

    // let rd_str = fs_mod::rd_relative_file_info("cfgs", "/cfg.json");

    let _ = logutil::init_logger();
    logutil::info("this is info log");
    logutil::warn("here is warn log");
    logutil::err("these are error log");
    logutil::debug("those are debug log");
    
    // 等待输入, 防止退出
    let mut inputs = String::new();
    io::stdin()
    .read_line(&mut inputs)
    .expect("Failed to read line");
}

mod fs_mod {
    use super::*; // use serde::{Deserialize};

    pub fn rd_relative_file_info (dir_path: &str,file_path: &str) -> String {
        let mut curr_dir = env::current_dir().unwrap();  // PathBuf
        let mut pre_path = curr_dir.display().to_string() + "/" + dir_path;
        // let mut pre_path = format!("{}/{}", curr_dir.display(), dir_path);
        println!("pre_path: {}", pre_path);
        
        
        // 判断目录是否存在, 不存在则创建
        match fs::metadata(&pre_path) {
            Ok(md) => {
                println!("{:?}", md);
            },
            Err(e) => {
                if e.kind() == io::ErrorKind::NotFound {
                    fs::create_dir(&pre_path).expect("create dir failed");
                    println!("create dir {} success!", pre_path);
                }
            }
        }

        pre_path.push_str(file_path); 
        
        println!("full file path: {}", pre_path);
        
        // 判断文件是否存在, 不存在则创建
        if fs::metadata(&pre_path).is_err() { 
            fs::File::create(&pre_path).expect("create file failed");
            println!("create file {} success!", pre_path);
        }
        

        let contents = CfgExtensionLib::cfgutil::rd_full_file(&pre_path);
        contents
    }
    
    
    pub fn rd_file_info (dir_path: &str,file_path: &str) -> String {
        // 判断目录是否存在, 不存在则创建
        match fs::metadata(dir_path) {
            Ok(md) => {
                println!("{:?}", md);
            },
            Err(e) => {
                if e.kind() == io::ErrorKind::NotFound {
                    fs::create_dir(file_path).expect("create dir failed");
                    println!("create dir {} success!", file_path);
                }
                println!("{:?}", e);
            }
        }

        // 判断文件是否存在, 不存在则创建
        if fs::metadata(file_path).is_err() { 
            fs::File::create(file_path).expect("create file failed");
            println!("create file {} success!", file_path);
        }
        

        let contents = CfgExtensionLib::cfgutil::rd_full_file(file_path);
        contents
    }
    
    
    #[derive(Debug, Deserialize)]
    pub struct basecfg {
        pub third_api_addr: String,
        pub time_out: i32
    }
    #[derive(Debug, Deserialize)]
    pub struct dbcfg {
        pub host: String,
        pub port: i32,
        pub user: String,
        pub pwd: String,
        pub db: String
    }
    #[derive(Debug, Deserialize)]
    pub struct cfg_toml {
        pub basecfg: basecfg,
        pub dbcfg: dbcfg
    }

    #[derive(Debug, Deserialize)]
    pub struct cfg_json {
        pub dbtype:String,
        pub dbcfg: dbcfg
    }

    #[derive(Debug, Deserialize)]
    pub struct cfg_yaml {
        pub dbtype:String,
        pub dbcfg: dbcfg
    }
}