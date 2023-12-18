use std::{sync::mpsc, thread};

const NTHREADS: u8 = 10;

pub fn create_threads() {
    println!("[Many Threads]");
    let mut handles: Vec<thread::JoinHandle<()>> = Vec::new();
    let mut responses = Vec::new();

    let (tx, rx) = mpsc::channel::<u8>();

    for i in 0..NTHREADS {
        let tx = tx.clone();
        let handle = thread::spawn(move || {
            // println!("{}", i);
            tx.send(i).expect("Could not send value");
        });
        handles.push(handle);
    }

    for received in rx {
        responses.push(received);

        if responses.len() == NTHREADS as usize {
            break;
        }
    }

    println!("Responses: {:?}", responses);

    for handle in handles {
        handle.join().expect("Could not join thread");
    }
}
