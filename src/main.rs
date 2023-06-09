use std::time::Duration;

use rdkafka::{ClientConfig, consumer::{Consumer, StreamConsumer}, Message, producer::{FutureProducer, FutureRecord}, util::Timeout};

#[tokio::main]
async fn main() {
    let mut config = ClientConfig::new();
    config.set("bootstrap.servers", "localhost:9092");
    config.set("group.id", "rust-rdkafka-roundtrip-example");

    let recents_consumer: StreamConsumer = config
        .create()
        .expect("can create consumer from configuration");

    let recents_producer: FutureProducer = config
    .create()
    .expect("can create producer from configuration");

    recents_producer.send(
        FutureRecord::to("recents").key("hello").payload("goodbye"),
        Timeout::After(Duration::new(5, 0)),
    ).await.unwrap();

    recents_consumer
        .subscribe(&["recents"])
        .expect("can subscribe to recents topic");

    println!("after subscribe");

    let owned_message = recents_consumer.recv().await.expect("can receive message");
    let received_key = std::str::from_utf8(owned_message.key().unwrap()).unwrap();

    assert_eq!("hello", received_key);
}
