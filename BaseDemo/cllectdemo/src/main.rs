mod vectorutil;
mod strutil;
mod hashmaputil;

fn main() {
    println!("--集合调用开始--");
    vectorutil::vec_fn();
    strutil::str_fn();
    hashmaputil::has_map_fn();
    println!("--集合调用结束--");
}
