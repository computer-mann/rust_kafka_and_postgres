use sqlx::error::BoxDynError;
use sqlx::Row;

pub async fn query_postgres() -> Result<(),BoxDynError> {
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