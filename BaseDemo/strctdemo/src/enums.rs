pub fn enum_fn () {
    println!("---begin enum---");
    let msg = Message::Write(String::from("hello"));
    let msg1 = Message::Move {
        x: 1, 
        y: 2
    };
    msg.call();
    msg1.call();
    
    let abs_number = Some(1);
    let sm_char = Some('f');

    let y: Option<i32> = Some(5);
}


enum IpKind {
    V4,
    V6
}

struct IpAddr {
    kind: IpKind,
    address: String
}

enum Message {
    Quit,
    Move {x:i32,y:i32},
    Write(String),
    ChgColor(i32,i32,i32),
}

impl Message {
    fn call(&self) {
        // method body would be defined here
        // println!("{0}",self.Write.to_string());
    }
}
