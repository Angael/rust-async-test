use std::{sync::mpsc, thread};

pub fn test_channels() {
    println!("[Channels]");

    let (tx, rx) = mpsc::channel();

    let handle1 = thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let handle2 = thread::spawn(move || {
        let received = rx.recv().unwrap();
        println!("Got: {}", received);
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
}
