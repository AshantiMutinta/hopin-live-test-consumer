use rdkafka::{Message,ClientConfig};
use rdkafka::consumer::BaseConsumer;
use std::time::Duration;
#[tokio::main]
async fn main() {
    let consumer: BaseConsumer = ClientConfig::new()
    .set("bootstrap.servers", "kafka:9092")
    .create()
    .expect("Producer creation error");

    loop 
    {
        let message = consumer.poll(Duration::from_secs(1)).unwrap().unwrap();
        let message = message.detach().payload().map(|s|String::from_utf8(s.to_vec()).unwrap()).unwrap_or_default();
        println!("obtained{:?}",message);
    }
}
