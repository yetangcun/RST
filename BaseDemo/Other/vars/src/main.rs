fn main() {
    let varia1 = 1;   // 不可变变量
    let mut varia = 5;  // 可变变量
    println!("you define a varia : {varia}");

    varia = 6;
    println!("you change the varia to : {varia}");

    const CONSTVAL:u32 = 888888;  // 常量
    println!("Hi: {CONSTVAL}");

    let var0 = 111;
    let var0 = var0 + 111;
    println!("var0: {var0}");
    {
        let var0 = var0*3;
        println!("var0:{var0}");
    }

    println!("last var0: {var0}");
}
