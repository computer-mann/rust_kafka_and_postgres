use redis::Commands;

pub fn use_redis() {
    let client = redis::Client::open("redis://127.0.0.1/3").expect("conn string");
    let mut con = client.get_connection().unwrap();
    // throw away the result, just make sure it does not fail
    //let _: () = con.set("my_key", 42).unwrap();
    let val:String=con.get("playlist:1AjROZATv3oNKLcPYQONMB").unwrap();
    println!("{:?}",val)
}