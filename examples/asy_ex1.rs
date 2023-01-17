use futures::executor::block_on;

async fn say_hi() {
    println!("hi",);
}

async fn hello_world() {
    println!("hello world");
}

async fn async_main() {
    say_hi().await;
}
fn main() {
    let future = hello_world();
    block_on(async_main());
    block_on(future);
}
