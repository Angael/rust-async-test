use std::{sync::mpsc, thread};

const NTHREADS: u16 = 64;

pub fn create_threads() {
    println!("[Many Threads]");
    let mut handles: Vec<thread::JoinHandle<()>> = Vec::new();
    let mut responses = Vec::new();

    let (tx, rx) = mpsc::channel();

    for i in 0..NTHREADS {
        let tx = tx.clone();
        handles.push(thread::spawn(move || {
            // println!("{}", i);
            tx.send(i).expect("Could not send value");
        }));
    }
    drop(tx);

    // because all tx are dropped, rx will stop receiving
    for received in rx {
        responses.push(received);
    }

    println!("Responses: {:?}", responses);

    for handle in handles {
        handle.join().expect("Could not join thread");
    }
}
