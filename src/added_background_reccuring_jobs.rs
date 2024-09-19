use std::time::Duration;
use chrono::{Local};
use job_scheduler::Job;
use crate::consumer::create_consumer;

pub async  fn schedule_jobs(){
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


}