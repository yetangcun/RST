pub mod cfgutil;
use std::io;
use std::path::Path;


pub trait LoadCfg<T> {
    fn load_cfg(cfg_path:&str) -> T;
    fn load_full_cfg<P: AsRef<Path>>(path: P) -> Result<T, Box<dyn std::error::Error>>;
}


pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
