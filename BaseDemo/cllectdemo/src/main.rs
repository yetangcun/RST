use std::io;
mod vectorutil;
mod strutil;
mod hashmaputil;

fn main() {
    println!("--集合调用开始--");
    vectorutil::vec_fn();
    strutil::str_fn();
    hashmaputil::has_map_fn();
    println!("--集合调用结束--");

    // io::stdout().flush().unwrap(); // 确保“按回车键退出程序...”被立即打印
    io::stdin().read_line(&mut String::new()).unwrap();
}
