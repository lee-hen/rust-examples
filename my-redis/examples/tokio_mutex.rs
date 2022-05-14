use tokio::sync::Mutex; // note! This uses the Tokio mutex
use std::sync::{Arc, Mutex as StdMutex, MutexGuard};

// This compiles!
// (but restructuring the code would be better in this case)
async fn increment_and_do_stuff(mutex: &Mutex<i32>) {
    let mut lock = mutex.lock().await;
    *lock += 1;

    do_something_async().await;
} // lock goes out of scope here


// This works!
async fn increment_and_do_stuff_std_mutex(mutex: &StdMutex<i32>) {
    {
        let mut lock: MutexGuard<i32> = mutex.lock().unwrap();
        *lock += 1;
    } // lock goes out of scope here

    do_something_async().await;
}


fn main() {}
