use MQExtensionLib::mqs;
use std::io;


#[tokio::main]
async fn main() {

    mqs::rs_kfk::KfkConsumer::recv("192.168.30.111:9092", &["hyp_kfk_tpc"]).await;
    // mqs::rs_kfk::KfkConsumer::recv("192.168.30.111:9092", &["test_topic"]);

    println!("Hello, world!");

    // 等待输入
    println!("Press enter to exit...");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
}
