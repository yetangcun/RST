use std::io;
mod strutil;
mod vectorutil;
mod hashmaputil;
mod cllectutil;
mod traitutil;
use std::fmt::Display;

use traitutil::{larger_num, largest_num1};
use traitutil::traitimpl::{NewsArticle,Tweet,Summary,trait_func,trait_func2,trait_fn5,trait_fn7};
use traitutil::lifeutil::{lft_fn,lft_fn1,lft_fn2,lft_fn3,lft_fn4};

fn main() {
    lft_fn2();
    lft_fn3();
    lft_fn4();

    let na_obj = NewsArticle {
        headline:String::from("News Article"),
        location:String::from("China"),
        author:String::from("Tom"),
        content:String::from("Hello World")
    };
    let twt_obj = Tweet {
        username:String::from("Tom"),
        content:String::from("Hello World"),
        reply:false,
        retweet:false
    };
    let na_res = na_obj.summarize();
    let na_tst = na_obj.tst_fn();
    println!("the na_res is: {na_res}, {na_tst}"); 
    // trait_func(&na_obj); trait_func(&twt_obj); trait_func2(&na_obj); trait_func2(&twt_obj);
    trait_fn5(&na_obj); trait_fn7(na_obj, twt_obj);

    largest_num1();

    let vecs = vec![1,2,3,4,5];
    cllectutil::getMax(&vecs);
    cllectutil::getMax0(&vecs);

    println!("--集合调用开始--");
    vectorutil::vec_fn();
    strutil::str_fn();
    hashmaputil::has_map_fn();
    println!("--集合调用结束--");

    // io::stdout().flush().unwrap(); // 确保“按回车键退出程序...”被立即打印
    io::stdin().read_line(&mut String::new()).unwrap();

}
