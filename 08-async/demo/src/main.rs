// mpsc: multi-producer single-consumer
use std::{
    collections::HashMap,
    sync::{mpsc::channel, Arc, Mutex},
    thread::{self, sleep},
    time::Duration,
};

#[tokio::main]
async fn main() {
    mutable_counter();
    misleading_mutable_counter();
    // forbidden_mutable_counter_struct();

    let ip = get_ip().await.unwrap();
    println!("IP: {}", ip);
}

fn mutable_counter() {
    let counter = Arc::new(Mutex::new(0));
    let mut done_count = 0;
    let thread_count = 4;
    let (tx, rx) = channel();

    // ..= is inclusive range
    for i in 1..=thread_count {
        let (counter, tx) = (Arc::clone(&counter), tx.clone());

        thread::spawn(move || {
            let mut counter = counter.lock().unwrap();
            println!("safe: doing {} + {}", *counter, i);
            *counter += i;
            tx.send(())
            // the lock is unlocked here when `data` goes out of scope.
        });
    }

    while done_count < 4 {
        rx.recv().unwrap();
        done_count = done_count + 1;
        println!("safe: threads done: {}", done_count)
    }

    println!("safe: total: {}", counter.lock().unwrap());
}

// Rust won't allow this to compile!
// fn forbidden_mutable_counter_struct() {
//     struct Counter {
//         pub val: i32,
//     }

//     let thread_count = 4;
//     let mut counter = Counter { val: 0 };

//     for i in 1..=thread_count {
//         thread::spawn(move || {
//             println!("forbidden: Doing {} + {}", counter.val, i);
//             counter.val += i;
//         });
//     }

//     sleep(Duration::new(1, 0));
//     println!("forbidden: counter is {}", &counter.val);
// }

fn misleading_mutable_counter() {
    let thread_count = 4;
    let mut counter = 0;

    for i in 1..=thread_count {
        // counter i32 is copied, so this compiles!
        thread::spawn(move || {
            println!("misleading: doing {} + {}", counter, i);

            // 🤔 !!!Very suspicious!!! warning from rustc!
            // value assigned to `counter` is never read
            // maybe it is overwritten before being read?
            // `#[warn(unused_assignments)]` on by default - rustc
            counter += i;
        });
    }

    sleep(Duration::new(1, 0));
    println!("misleading: counter is {}", counter);
}

// Threads = parallel, async = maybe parallel
// An async runtime lets us use async/await
// Async usually comes with a thread pool

async fn get_ip() -> Result<String, Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    let res = format!("{:#?}", resp);
    Ok(res)
}
