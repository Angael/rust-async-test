use std::{
    sync::mpsc,
    sync::{Arc, Mutex},
    thread,
};

const NTHREADS: u16 = 16;

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

    for handle in handles {
        handle.join().expect("Could not join thread");
    }

    println!("Responses: {:?}", responses);
}

pub fn create_threads_arc() {
    println!("[Many Threads Arc]");
    let mut handles: Vec<thread::JoinHandle<()>> = Vec::new();
    let responses = Arc::new(Mutex::new(vec![]));

    for i in 0..NTHREADS {
        let responses = Arc::clone(&responses);
        handles.push(thread::spawn(move || responses.lock().unwrap().push(i)));
    }

    for handle in handles {
        handle.join().expect("Could not join thread");
    }

    println!("Responses: {:?}", responses.lock().unwrap());
}
