use std::time::Duration;
use chrono::Utc;
use rdkafka::message::{Header, OwnedHeaders};
use rdkafka::producer::{FutureProducer, FutureRecord};
use uuid::Uuid;

pub async fn produce_to_kafka(){
    let now=Utc::now().to_string();
    let future_producer:FutureProducer = rdkafka::ClientConfig::new()
        .set("bootstrap.servers",std::env::var("KAFKA_SERVER").unwrap())
        .create().expect("failed to create");
    let msg=r#"{ "Value": 151 }"#;

    let headers=Header{key:"date_created",value:Some(&now)};
    let owned_headers=OwnedHeaders::new().insert(headers);
    let key=Uuid::now_v7().to_string();
    let topic=std::env::var("KAFKA_TOPIC").unwrap();
    let record=FutureRecord::to(&topic)
        .payload(msg).key(&key).headers(owned_headers);

    let delivery_status= future_producer.send(
        record,
        rdkafka::util::Timeout::After(Duration::from_secs(3))
    ).await;

    match delivery_status {
        Ok(report)=> println!("{:?}",report),
        Err(err)=> println!("{:?}",err)
    }

}