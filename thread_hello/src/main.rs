use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("Hi number {} from main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    println!("Hello, world!");
    handle.join().unwrap();
}
