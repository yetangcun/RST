use md5;
pub fn md5_hash(source:&String) -> String {

    let hsh = md5::compute(source.as_bytes()); // 返回一个md5::Digest类型

    // 将Digest转换为字符串
    let hex_str = hsh.iter().map(|byte| format!("{:02x}", byte)).collect();
    
    hex_str

}