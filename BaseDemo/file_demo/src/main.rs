use std::io;
use CfgExtensionLib::cfgutil::{CfgLoader, LoadCfg};
use serde::{Deserialize, Serialize};

use fs_mod::{cfg_toml, cfg_json, cfg_yaml};

fn main() {
    let contents = CfgExtensionLib::cfgutil::rd_relativ_file("tst.txt");
    println!("file contents: {}, demo!", contents);

    let toml_obj:cfg_toml = CfgLoader::load_cfg::<cfg_toml>("toml.toml"); // println!("cfg_toml: {:?}", cfg_toml);
    println!("api_addr: {}", toml_obj.basecfg.third_api_addr);

    let json_obj:cfg_json = CfgLoader::load_cfg::<cfg_json>("json.json");
    println!("db host: {}", json_obj.dbcfg.host);

    // 等待输入, 防止退出
    let mut inputs = String::new();
    io::stdin()
    .read_line(&mut inputs)
    .expect("Failed to read line");
}

mod fs_mod {
    // use serde::{Deserialize};
    use super::*;
    
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