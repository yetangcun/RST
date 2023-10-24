fn main() {
    let integer = 2i32;
    println!("the integer val is {}", integer);

    let integer = 222u8;
    println!("the new integer val is {}", integer);

    let floatval32:f64 = 16.8;
    println!("the f val is: {floatval32}");

    let floatval = 16.88f64;
    println!("the f32 val is: {}", floatval);

    let result:f64 = floatval-floatval32;
    println!("the result is: {}", result);
}
