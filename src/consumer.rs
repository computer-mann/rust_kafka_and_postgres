use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::error::KafkaError::ClientConfig;
use rdkafka::Message;
use rdkafka::message::Headers;

pub async  fn create_consumer(){

    let mut  base_consumer:StreamConsumer = rdkafka::ClientConfig::new()
        .set("group.id",std::env::var("CONSUMER_GROUP_ID").unwrap())
        .set("bootstrap.servers",std::env::var("KAFKA_SERVER").unwrap())
        .set("auto.offset.reset","earliest")
        .set("socket.timeout.ms","4000")
        .set("enable.auto.commit","false")
        //.set("retention.ms","900000")
        .create().expect("failed to create");


    base_consumer.subscribe(&[&*std::env::var("KAFKA_TOPIC").unwrap()]).expect("Cant subscribe");
    log::info!("about to start consuming from {}",std::env::var("KAFKA_TOPIC").unwrap());
    loop {
        match base_consumer.recv().await {
            Err(e)=> println!("{:?}",e),
            Ok(message)=> match message.payload_view::<str>() {
                None => println!("no message"),
                Some(Ok(payload )) => {
                    let mut my_headers:Vec<String>=vec![];
                    if let Some(existing_headers)=message.headers(){
                        existing_headers.iter().for_each(|x|{
                            my_headers.push(format!("{}",x.key))
                        });
                    }
                    log::info!("Partition= {:?} , {}",my_headers,payload);
                    log::info!("thread handling kafka consumer: {}",std::thread::current().name().unwrap());
                    tokio::time::sleep(std::time::Duration::from_millis(500)).await
                } ,
                Some(Err(err)) => println!("{:?}",err)
            }
        }
    }
}