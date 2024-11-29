use lazy_static::lazy_static;
use tokio::sync::Mutex;
use std::time::Duration;
use serde::{Deserialize, Serialize};

use rdkafka::error::KafkaResult;
use rdkafka::config::ClientConfig;
use rdkafka::message::{Header,Message};

use rdkafka::producer::{FutureProducer,FutureRecord};

use rdkafka::consumer::stream_consumer::StreamConsumer;
use rdkafka::consumer::{Consumer,BaseConsumer,CommitMode,ConsumerContext,Rebalance};

lazy_static! {
    pub static ref KFK_PRD: Mutex<FutureProducer> = Mutex::new(KfkProducer::_init("192.168.30.111:9092"));
}

pub struct KfkProducer {
    pub producer: FutureProducer,
}
impl KfkProducer {
    pub fn _init(kfk_server:&str) -> FutureProducer {

        let prd:FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", kfk_server)
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Producer creation error");

        prd
    }

    pub async fn send<T>(msg:&kfk_msg<T>) where T: 
    for<'a> Deserialize<'a> 
    + Serialize 
    {
        let _prd = KFK_PRD.lock().await;
        let topic = msg.topic.clone();
        let msg_json = serde_json::to_string(&msg.msg).unwrap();
        _prd.send(
            FutureRecord::<(), String>::to(&topic).
            payload(&msg_json).
            partition(msg.partition), 
            Duration::from_secs(0)).
            await.
            unwrap();
    }
}

pub struct KfkConsumer {
}
impl KfkConsumer {
    pub fn new() -> Self {
        KfkConsumer { }
    }

    pub fn recv(&self, topic: &str) -> String {
        String::from("")
    }
}

pub struct kfk_msg<T> 
where T: 
for<'a> Deserialize<'a> 
+ Serialize 
{
    pub partition: i32,
    pub topic: String,
    pub msg_id: i64,
    pub msg_type: i32,
    pub msg: T
}