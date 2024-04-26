fn main() {
    let mut usr1 = User {
        addr: String::from("yetangcun"),
        name: String::from("xiaoxiao"),
        age: 23
    };

    usr1.addr = String::from("yetangcun.");
    println!("Hello, {0}", usr1.name);
    println!("you're from {0}", usr1.addr);

    let usr2 = User {
        age: 24,
        ..usr1
    };
    println!("you're from the {0}", usr2.addr);
}


// 结构体定义
struct User {
    name: String,
    age:i32,
    addr: String
}