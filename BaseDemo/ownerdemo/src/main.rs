fn main() {

    let _scope = "Hello rst";
    let mut scopes = String::from("hello");
    scopes.push_str(", world...");

    println!("{scopes}");

    // 所有权
    let str1 = String::from("hey");
    let _str2 = str1;
    println!("{}", _str2);

    let i1 = 9;
    let _i2 = i1;
    println!("{}", i1);
}
