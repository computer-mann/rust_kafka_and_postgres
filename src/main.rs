mod consumer;
mod producer;
mod using_postgres;
mod redis_caching;
mod added_background_reccuring_jobs;
mod boyer_moore_maximum_voting_algoritm;

use rdkafka::consumer::Consumer;
use rdkafka::message::Headers;
use rdkafka::Message;
use sqlx::{Connection, Row};
use std::error::Error;
use std::ops::{Deref, DerefMut};
use uuid::Uuid;

#[tokio::main]
async fn main()->Result<(), Box<dyn Error>> {
    simple_logger::init_with_level(log::Level::Info).unwrap();
    let uuid=Uuid::now_v7();
    println!("{}",uuid);
    redis_caching::use_redis();
    Ok(())

}
