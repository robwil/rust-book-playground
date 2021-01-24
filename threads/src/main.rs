use std::thread;
use std::time::Duration;

fn main() {
    // Thread basics: spawn, move, and join. https://doc.rust-lang.org/book/ch16-01-threads.html
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        for i in 1..10 {
            println!("hi number {} from the spawned thread! {:?}", i, v);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
    println!("after join");

    // Channels and message passing. https://doc.rust-lang.org/book/ch16-02-message-passing.html
    
}
