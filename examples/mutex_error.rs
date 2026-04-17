use std::sync::{Arc, Mutex};

async fn do_something_async() {
    println!("doing async work...");
}

async fn increment_and_do_stuff(mutex: &Mutex<i32>) {
    {
        let mut lock = mutex.lock().unwrap(); // MutexGuard

        *lock += 1;
    }

    do_something_async().await; // ❗跨 await 持有 lock
}

#[tokio::main]
async fn main() {
    let data = Arc::new(Mutex::new(0));

    let data2 = data.clone();

    tokio::spawn(async move {
        increment_and_do_stuff(&data2).await;
    })
    .await
    .unwrap();
}
