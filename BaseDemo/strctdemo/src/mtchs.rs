mod enums;

pub enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter
}

pub fn match_fn1(cn:Coin) -> u8 {
    match cn {
        Coin::Penny=>1,
        Coin::Nickel=>5,
        Coin::Dime=>10,
        Coin::Quarter=>25
    }
}

pub fn match_fn2(msg:enums::Message) -> String  {
    match msg {
        enums::Message::Quit=>println!("Quit"),
        enums::Message::Move{x,y}=>println!("Move x:{0},y:{1}",x,y),
        enums::Message::Write(s)=>println!("Write:{0}",s),
        enums::Message::ChgColor(r,g,b)=>println!("ChgColor:{0},{1},{2}",r,g,b)
    }

    return "".to_string();
}