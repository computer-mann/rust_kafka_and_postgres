mod consumer;
mod producer;
mod using_postgres;
mod redis_caching;

use std::error::Error;
use std::fmt::format;
use std::ops::{Deref, DerefMut};
use std::thread;
use std::time::Duration;
use chrono::{Local, Utc};
use job_scheduler::Job;
use rdkafka::consumer::{BaseConsumer, Consumer, StreamConsumer};
use rdkafka::ClientConfig;
use rdkafka::Message;
use rdkafka::message::{BorrowedHeaders, Headers};
use sqlx::{Connection, Row};
use uuid::Uuid;
use crate::consumer::create_consumer;
use crate::producer::produce_to_kafka;

#[tokio::main]
async fn main()->Result<(), Box<dyn Error>> {
    simple_logger::init_with_level(log::Level::Info).unwrap();
    let uuid=Uuid::now_v7();
    println!("{}",uuid);
    let mut scheduler=job_scheduler::JobScheduler::new();
    log::info!("program start: ");
    scheduler.add(Job::new("1/5 * * * * *".parse().unwrap(),|| {
        log::info!("thread handling 5 seconds job: {}",std::thread::current().name().unwrap());
        log::info!("every 5 seconds {}",Local::now())
    }));
    // scheduler.add(Job::new("1/10 * * * * *".parse().unwrap(),|| println!("every 9 seconds")));
    // scheduler.add(Job::new("1/10 * * * * *".parse().unwrap(),|| println!("every 11 seconds")));

    tokio::spawn(async {
        create_consumer().await
    });


            loop {
                scheduler.tick();
                tokio::time::sleep(Duration::from_millis(200)).await
            }


    Ok(())

}
