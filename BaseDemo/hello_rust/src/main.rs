use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() { // println!("Hello, rust!");
    println!("welcome to rust!");
    
    let _sec_num = rand::thread_rng().gen_range(100..=1000);
    println!("the real number is ：{_sec_num}");

    loop {
        println!("please input:");

        let mut guess = String::new(); // mut使变量是可变变量

        io::stdin().read_line(&mut guess).expect("failed to read line");

        let guess:u32 = match guess.trim().parse() {  // 数据类型转换 并进行数据校验
            Ok(num) => num,
            Err(_) => {
                println!("输入的内容必须是数字");
                continue;
            }
        };

        println!("you guessed: {guess}");

        match guess.cmp(&_sec_num) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
