use rdkafka::{Message,ClientConfig};
use rdkafka::consumer::BaseConsumer;
use std::time::Duration;
use rdkafka::consumer::Consumer;
#[tokio::main]
async fn main() {

    
    let consumer: BaseConsumer = ClientConfig::new()
    .set("bootstrap.servers", "localhost:9092")
    .set("group.id","Hopin")
    .create()
    .expect("Producer creation error");

    consumer.subscribe(&["hopin-live"]).unwrap();
    loop 
    {
        match consumer.poll(Duration::from_secs(1))
        {
            Some(message_type) =>
            {
                match message_type
                {
                    Ok(message) =>
                    {
                        let message = message.detach().payload().map(|s|String::from_utf8(s.to_vec()).unwrap()).unwrap_or_default();
                       println!("obtained{:?}",message);
                    },
                    Err(error_type) => println!("error obtainign message {:?}",error_type)
                }
            },
            None => println!("none obtained")
        }

    }
}
