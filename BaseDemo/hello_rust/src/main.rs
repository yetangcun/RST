use std::io;

fn main() {
    // println!("Hello, rust!");
    println!("welcome, rust!");

    println!("please input:");
    let mut guess = String::new();

    io::stdin()
    .read_line(&mut guess)
    .expect("failed to read line");

    println!("you guessed: {guess}")
}
