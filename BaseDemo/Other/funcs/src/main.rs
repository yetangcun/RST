fn main() {
    println!("Hello, world!");

    ano_func(888);

    let val = func_1();
    println!("the result is {val}")
}


fn ano_func(params:i32) {
    println!("Here is a extension fn, and params is: {params}")
}

fn func_1() -> i32 {
    let const_v = 88888;
    return const_v;
}