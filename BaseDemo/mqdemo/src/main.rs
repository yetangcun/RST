use MQExtensionLib::mqs::rs_kfk;
use CommonExtensionLib::utils::cache_util;
use MQExtensionLib::mqs::rs_kfk::{kfk_msg, msg_mdl};
use std::io;


#[tokio::main]
async fn main() {

    // println!("Waiting msg from kafka...");
    // mqs::rs_kfk::KfkConsumer::recv("192.168.3.101:9092", &["hyp_kfk_tpc"]).await;
    // rs_kfk::KfkConsumer::recv("192.168.30.111:9092", &["hyp_kfk_tpc"]).await;

    // 等待输入, 递归生产消息
    // prd_msg().await;
    
    // cache_util::set_str_cache("ky1".to_string(), "val1".to_string()); // 写入本地缓存

    let mut cache_val = "".to_string();
    match cache_util::get_str_cache("ky1") {  // 获取本地缓存
        Some(val) => {
            cache_val = val;
        },
        None => {
            println!("cache not found");
        }
    };

    println!("get local cache val: {:?}", cache_val);
}

async fn prd_msg() {
    println!("Waiting input...");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    if input.trim() == "exit" {
        return;
    }

    let lens = input.len();
    if lens > 0 {
        let msg = input.trim();
        let _msg:kfk_msg<msg_mdl> = kfk_msg::<msg_mdl> {
            partition: 0,
            topic: "hyp_kfk_tpc".to_string(),
            msg: msg_mdl {
                msg_tp: 1,
                msg_id: 111111,
                msg: msg.to_string()
            }
        };
        let _msg = rs_kfk::KfkProducer::send::<msg_mdl>(&_msg).await;
    }
    
    let _res = Box::pin(prd_msg());  // 继续等待输入
    _res.await;
}
