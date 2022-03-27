use async_std::task::block_on;

async fn say_hello(){
    println!("Hello Again");
}

fn main() {
    block_on(say_hello());
}
