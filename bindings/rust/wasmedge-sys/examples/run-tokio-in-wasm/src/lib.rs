use tokio::{
    runtime,
    time::{sleep, Duration},
};

async fn say_hello() {
    println!("sleep 2 secs");
    sleep(Duration::from_secs(2)).await;
    println!("Hello, rust!");
}

#[no_mangle]
pub fn hello() {
    let rt = runtime::Builder::new_current_thread().build().unwrap();
    rt.block_on(say_hello());
}
