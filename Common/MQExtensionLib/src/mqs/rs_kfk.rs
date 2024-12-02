use lazy_static::lazy_static;
use tokio::sync::Mutex;
use std::time::Duration;
use serde::{Deserialize, Serialize};

use rdkafka::error::KafkaResult;
use rdkafka::client::ClientContext;
use rdkafka::message::{Header,Message};
use rdkafka::config::{ClientConfig, RDKafkaLogLevel};
use rdkafka::topic_partition_list::TopicPartitionList;

use rdkafka::producer::{FutureProducer,FutureRecord};

use rdkafka::consumer::stream_consumer::StreamConsumer;
use rdkafka::consumer::{Consumer,BaseConsumer,CommitMode,ConsumerContext,Rebalance};

lazy_static! {
    pub static ref KFK_PRD: Mutex<FutureProducer> = Mutex::new(KfkProducer::_init("192.168.30.111:9092"));
}

pub struct KfkProducer {
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

struct CustomContext;

impl ClientContext for CustomContext {}

impl ConsumerContext for CustomContext {
    fn pre_rebalance(&self, _: &BaseConsumer<Self>, rebalance: &Rebalance) {
        // info!("Pre rebalance {:?}", rebalance);
    }

    fn post_rebalance(&self, _: &BaseConsumer<Self>, rebalance: &Rebalance) {
        // info!("Post rebalance {:?}", rebalance);
    }

    fn commit_callback(&self, result: KafkaResult<()>, _offsets: &TopicPartitionList) {
        // info!("Committing offsets: {:?}", result);
    }
}
type LoggingConsumer = StreamConsumer<CustomContext>;
pub struct KfkConsumer {
}
impl KfkConsumer {
    pub async fn recv(brokers: &str, topic: &[&str]) {
        let context = CustomContext;

        let consumer: LoggingConsumer = ClientConfig::new()
        .set("group.id", "console-consumer-80757")
        .set("bootstrap.servers", brokers) // "192.168.30.111:9092"
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "true")
        //.set("auto.offset.reset", "earliest")
        .set_log_level(RDKafkaLogLevel::Debug)
        .create_with_context(context)
        .expect("Consumer creation failed");

        consumer.
        subscribe(&topic.to_vec()).
        expect("Can not subscribe to specified topic");

        println!("Kfk consumer waiting for messages...");
        loop {
            match consumer.recv().await {
                Err(e) => println!("Kafka error: {}", e),
                Ok(m) => {
                    // println!("Recvd Messages:");
                    println!("key: {:?}, payload: {:?}, topic: {:?}, partition: {:?}, offset: {:?}, timestamp: {:?}",
                        m.key(),
                        m.payload_view::<str>().unwrap(),
                        m.topic(),
                        m.partition(),
                        m.offset(),
                        m.timestamp());
                    
                    consumer.commit_message(&m, CommitMode::Async).unwrap();
                }
            }
        }
    }
}

pub struct kfk_msg<T> 
where T: 
for<'a> Deserialize<'a> 
+ Serialize 
{
    pub partition: i32,
    pub topic: String,
    // pub msg_id: i64, // 消息ID
    // pub msg_tp: i32, // 消息类型
    pub msg: T
}