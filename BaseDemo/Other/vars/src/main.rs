fn main() {
    let mut varia = 5;
    println!("you define a varia : {varia}");

    varia = 6;
    println!("you change the varia to : {varia}");

    let x = 6;
    println!("current x val is : {}",x);

    {
        let x = x+1;
        println!("current x value is : {}",x);
    }

    let x = x*2;
    println!("now x val become: {}",x);

    const XVAL:u32 = 3600*24*365;
    println!("the const XVAL IS: {}",XVAL);
}
