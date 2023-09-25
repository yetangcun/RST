use std::io;

fn main() {
    // println!("Hello, rust!");
    println!("welcome to rust!");

    println!("please input:");

    let mut guess = String::new(); // mut使变量是可变变量

    io::stdin()
    .read_line(&mut guess)
    .expect("failed to read line");

    println!("you guessed: {guess}");

    let test = 666;
    println!("the test value is {}", test);
}
