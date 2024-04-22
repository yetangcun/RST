fn main() {

    let _scope = "Hello rst";
    let mut scopes = String::from("hello");
    scopes.push_str(", world...");

    println!("{scopes}");

    // 所有权 字符串类型 移动 旧变量赋值给新变量后被后一个新变量覆盖 旧变量就失效了
    let str1 = return_str();
    let _str2 = str1;  // 所有权转移
    let _restr = reference_str(&_str2);
    println!("{_str2},{_restr}");
    str_fn(_restr);  // 所有权转移

    let mut rstr = String::from("Hi");
    println!("{}--", rstr);
    refer_str(&mut rstr);
    println!("{}+++", rstr);

    // 标量类型 以及 由标量类型组成的复合类型都不一样 赋值给新变量之后旧变量依然有效可用
    let i1 = 9;
    let _i2 = i1;
    println!("{i1}, {_i2}");

    str()
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

fn refer_str(str:&mut String){
    str.push_str(", my friend");
}

fn str() {
    let s = String::from("test 007");
    let _rstr = str_refer(&s);
    println!("{_rstr}, {s}")
}

fn str_refer(sr:&String) -> String {
    let mut nsr = String::from(sr);
    nsr.push_str(",welcome");

    let nsr1 = "nsr1";
    // let nsr1 = &nsr; // 此路不通
    let nsr2 = &mut nsr;

    println!("{0},{1}", nsr1, nsr2);
    
    println!("{0}", nsr2);

    let nsr3 = &mut nsr;
    
    println!("{0}", nsr3);

    nsr
    // println!("{sr}, welcome");
}
