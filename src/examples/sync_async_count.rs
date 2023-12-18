use std::thread;
use std::time::Duration;

pub fn count() -> Vec<i32> {
    println!("[Count]");
    let mut vector: Vec<_> = Vec::new();

    let handle = thread::spawn(move || {
        for i in 0..10 {
            // println!("[spawned] {}", i);
            vector.push(i * 1);
            thread::sleep(Duration::from_millis(1));
        }
        vector
    });

    let mut vector2 = handle.join().unwrap();

    for i in 0..5 {
        // println!("[main] {}", i);
        vector2.push(i * 1000);
        thread::sleep(Duration::from_millis(1));
    }

    return vector2;
}
