use futures::stream::StreamExt;
use rdkafka::consumer::Consumer;
use rdkafka::consumer::StreamConsumer;
use rdkafka::{ClientConfig, Message};

#[tokio::main]
async fn main() {
    let consumer: StreamConsumer = ClientConfig::new()
        .set("bootstrap.servers", "localhost:9092")
        .set("group.id", "Hopin")
        .create()
        .expect("Producer creation error");

    consumer.subscribe(&["hopin-live"]).unwrap();
    let mut message_stream = consumer.start();
    loop {
        if let Some(Ok(message)) = message_stream.next().await {
            let message = message
                .detach()
                .payload()
                .map(|s| String::from_utf8(s.to_vec()).unwrap())
                .unwrap_or_default();
            println!("message obtained {:?}", message);
        }
    }
}
