use std::io;
mod strutil;
mod vectorutil;
mod hashmaputil;
mod cllectutil;
mod traitutil;
use std::future::Future;
use std::fmt::Display;

// use tokio::runtime::Runtime;
use tokio::runtime::Runtime;

use traitutil::{larger_num, largest_num1};
use traitutil::traitimpl::{NewsArticle,Tweet,Summary,trait_func,trait_func2,trait_fn5,trait_fn7};
use traitutil::lifeutil::{lft_fn,lft_fn1,lft_fn2,lft_fn3,lft_fn4,lft_fn6};
use CacheExtensionLib::{rscaches::rdscache};

#[tokio::main]
async fn main() {

    let ky = String::from("uid:99999999");
    let vl = String::from("666666");
    // let rds_conn = rdscache::RdsCache::set_str(&ky, vl);
    // let rs_vl = rdscache::RdsCache::get_str(&ky);
    // println!("rs_rds_vl is : {rs_vl}");

    // 创建一个 Tokio 运行时
    // let runtime = Runtime::new().unwrap();
    // let rds_pool_set = runtime.block_on(rdscache::RdsPool::set_str(&ky, vl));
    // let rds_pool_get = runtime.block_on(rdscache::RdsPool::get_str(&ky));
    // println!("rds_pool_get is : {rds_pool_get}");
    
    let rds_pool_set = rdscache::RdsPool::set_str(&ky, vl).await;
    let rds_pool_get = rdscache::RdsPool::get_str(&ky).await;
    println!("rds_pool_get is : {rds_pool_get}");

    lft_fn2();
    lft_fn3();
    lft_fn4();

    let na_obj = NewsArticle {
        headline:String::from("News Article"),
        location:String::from("China"),
        author:String::from("Tom"),
        content:String::from("Hello good friends!")
    };
    let twt_obj = Tweet {
        username:String::from("Tom"),
        content:String::from("Hello my friends"),
        reply:false,
        retweet:false
    };

    let str1 = String::from("Hello, world!");
    let str2 = String::from("welcome, my friends!");
    lft_fn6(&str1, &str2, &na_obj);

    let na_res = na_obj.summarize();
    let na_tst = na_obj.tst_fn();
    println!("the na_res is: {na_res}, {na_tst}");  // trait_func(&na_obj); trait_func(&twt_obj); trait_func2(&na_obj); trait_func2(&twt_obj);
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
