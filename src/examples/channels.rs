use std::{sync::mpsc, thread};

pub fn test_channels() {
    println!("[Channels]");

    let (tx, rx) = mpsc::channel();

    let handle1 = thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).expect("Could not send value");
    });

    let handle2 = thread::spawn(move || {
        let received = rx.recv().expect("Could not receive value");
        println!("Got: {}", received);
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
}
