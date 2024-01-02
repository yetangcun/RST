fn main() {

    let _scope = "Hello rst";
    let mut scopes = String::from("hello");
    scopes.push_str(", world...");

    println!("{scopes}");

    // 所有权 字符串类型 移动 旧变量赋值给新变量后被后一个新变量覆盖 旧变量就失效了
    let str1 = return_str();
    let _str2 = str1;
    let _restr = reference_str(&_str2);
    println!("{_str2},{_restr}");
    str_fn(_restr);  // 所有权转移

    // 标量类型 以及 由标量类型组成的复合类型都不一样 赋值给新变量之后旧变量依然有效可用
    let i1 = 9;
    let _i2 = i1;
    println!("{i1}, {_i2}");
}


fn return_str() -> String {
    let str = String::from("hey");
    str
}

fn str_fn(str:String) {
    println!("recv val: {}", str);
}

fn reference_str(str:&String) -> String {
    let mut str1 = String::from(str);
    str1.push_str(", my friend");
    str1
}