pub fn vec_fn() {
    let v:Vec<i32> = Vec::new();
    let mut vec = vec![1,2,3];
    vec.push(4);

    let thrd: &i32 = &vec[0];
    let secd: Option<&i32> = vec.get(1);
    println!("the first vec element is: {thrd}");

    let mut vec2 = Vec::new();
    vec2.push(1);
}