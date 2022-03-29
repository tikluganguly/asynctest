use async_std::task::{block_on, sleep};
use futures::future::join;
use std::time;

async fn say_hello(){
    println!("Hello Again");
}

async fn with_result()->Result<String,()>{
    Ok(String::from("Hello with result"))
}

async fn connect_radis()->String{
    sleep(time::Duration::from_secs(1)).await;
    String::from("Radis Connected")
}

async fn connect_db()->String{
    sleep(time::Duration::from_secs(1)).await;
    String::from("Db Connected")
}

async fn main_executor(){
    say_hello().await;
    let (db,radis)= join(connect_db(),connect_radis()).await;
    println!("{}, {}",db,radis);
    println!("{}",with_result().await.unwrap());
}

fn main() {
    block_on(main_executor());
}
