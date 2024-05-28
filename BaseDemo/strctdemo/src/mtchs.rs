use crate::enums;

pub enum UsState {
    Alabama,
    Alaska,
    // --snip--
}
pub enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

// 特殊模式匹配 _
pub fn match_fn0() {
    let d_roll = 9;
    match d_roll {
        6=>println!("six"),
        7=>println!("seven"),
        _=>println!("others")
    }
}

// 通配模式 other
pub fn match_fn1(cn:Coin) -> u8 {
    match cn {
        Coin::Penny=>1,
        Coin::Nickel=>5, // Coin::Dime=>10,
        Coin::Quarter(ustate)=>25,
        other=>100,
    }
}

// 引用其他文件定义的枚举
pub fn match_fn2(msg:enums::Message) -> String  {
    match msg {
        enums::Message::Quit=> String::from("Quit"),
        enums::Message::Move{x,y}=> String::from("MOVE"),
        enums::Message::Write(s)=> s,
        enums::Message::ChgColor(r,g,b)=> String::from("EMPTY")
    }
}

// Option<T> 枚举使用
pub fn match_fn3(msg:Option<enums::IpAddr>) -> String  {
    match msg {
        Some(enums::IpAddr{kind,address})=> String::from(address),
        None=> String::from("None")
    }
}

pub fn match_fn4(cfg_max:Option<u8>) {
    // let cfg_max = Some(3u8);
    if let Some(max) = cfg_max {
        println!("The maximum is configured to be {}", max);
    }
    else {
        println!("The maximum is not configured");
    }
    // match cfg_max {
    //     Some(max) => println!("The maximum is configured to be {}", max),
    //     _ => (),
    // }
}
