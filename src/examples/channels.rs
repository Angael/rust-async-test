use std::{sync::mpsc, thread};

pub fn test_channels() {
    println!("[Channels]");

    let (tx, rx) = mpsc::channel::<String>();

    let handle1 = thread::spawn(move || {
        let val = String::from("message from channel thread, sending via tx");

        // Comment to see the error
        tx.send(val).expect("Could not send value");
    });

    let handle2 = thread::spawn(move || {
        let received = rx
            .recv_timeout(std::time::Duration::from_secs(1))
            .expect("Could not receive value");

        println!("Got: {}", received);
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
}
