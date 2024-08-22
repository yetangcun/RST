use std::io;
mod vectorutil;
mod strutil;
mod hashmaputil;
mod cllectutil;
mod traitutil;

use traitutil::{larger_num, largest_num1};

fn main() {

    largest_num1();

    let vecs = vec![1,2,3,4,5];
    cllectutil::getMax(&vecs);
    cllectutil::getMax0(&vecs);

    println!("--集合调用开始--");
    vectorutil::vec_fn();
    strutil::str_fn();
    hashmaputil::has_map_fn();
    println!("--集合调用结束--");

    // io::stdout().flush().unwrap(); // 确保“按回车键退出程序...”被立即打印
    io::stdin().read_line(&mut String::new()).unwrap();

}
