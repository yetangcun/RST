use std::io;
use CfgExtensionLib::{cfgutil::{TomlCfgLoader, JsonCfgLoader, YamlCfgLoader, LoadCfg}};
use serde::{Deserialize, Serialize};

use fs_mod::{cfg_toml, cfg_json, cfg_yaml};

fn main() {
    let contents = CfgExtensionLib::cfgutil::rd_relativ_file("tst.txt");
    println!("file contents: {}, demo!", contents);

    let toml_cfg = TomlCfgLoader{};
    let cfg_toml:cfg_toml = TomlCfgLoader::load_cfg::<cfg_toml>("cfg.toml");
    println!("cfg_toml: {:?}", cfg_toml);

    // 等待输入, 防止退出
    let mut inputs = String::new();
    io::stdin()
    .read_line(&mut inputs)
    .expect("Failed to read line");
}

mod fs_mod {
    use serde::{Deserialize};
    
    #[derive(Debug, Deserialize)]
    pub struct basecfg {
        third_api_addr: String,
        time_out: i32
    }
    #[derive(Debug, Deserialize)]
    pub struct dbcfg {
        pub host: String,
        pub port: String,
        pub user: String,
        pub pwd: String,
        pub db: String
    }
    #[derive(Debug, Deserialize)]
    pub struct cfg_toml {
        basecfg: basecfg,
        dbcfg: dbcfg
    }

    #[derive(Debug, Deserialize)]
    pub struct cfg_json {
        dbtype:String,
        dbcfg: dbcfg
    }

    #[derive(Debug, Deserialize)]
    pub struct cfg_yaml {
        dbtype:String,
        dbcfg: dbcfg
    }
}