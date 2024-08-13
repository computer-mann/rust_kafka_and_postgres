mod consumer;
mod producer;
mod using_postgres;

use std::error::Error;
use std::fmt::format;
use std::ops::{Deref, DerefMut};
use std::thread;
use rdkafka::consumer::{BaseConsumer, Consumer, StreamConsumer};
use rdkafka::ClientConfig;
use rdkafka::Message;
use rdkafka::message::{BorrowedHeaders, Headers};
use sqlx::{Connection, Row};
use uuid::Uuid;
use crate::producer::produce_to_kafka;

#[tokio::main]
async fn main()->Result<(), Box<dyn Error>> {
    simple_logger::init_with_level(log::Level::Info).unwrap();
    let uuid=Uuid::now_v7();
    println!("{}",uuid);
    log::info!("program start: ");
    let con_string=std::env::var("DB_CONNECTION").unwrap();

    let conn=sqlx::postgres::PgPool::connect(&con_string).await?;
    log::info!("connection successful");
    let all_customers=sqlx::query(r#"Select * from public."Customers" "#)
        .fetch_all(&conn).await?;
    all_customers.iter().for_each(|x|{
        let username:String=x.get("Username");
        println!("{:?}",username)
    });

    Ok(())
}
