use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;
use std::sync::mpsc;

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
    let (tx, rx) = mpsc::channel();
    let tx1 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(200));
        }
    });
    
    thread::spawn(move || {
        let vals = vec![
            String::from("hi2"),
            String::from("from2"),
            String::from("the2"),
            String::from("thread2"),
        ];
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(200));
        }
    });

    for received in rx {
        println!("Received: {}", received);
    }

    // Shared state concurrency: https://doc.rust-lang.org/book/ch16-03-shared-state.html
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for i in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            println!("thread {} incrementing num: {}", i, num);
            *num += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result: {}", *counter.lock().unwrap());

    // Creating deadlock is still possible!
    println!("About to create deadlock...");
    let counter1 = Arc::new(Mutex::new(0));
    let counter2 = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 1..3 {
        let counter1c = Arc::clone(&counter1);
        let counter2c = Arc::clone(&counter2);
        let handle = thread::spawn(move || {
            let mut num1 = counter1c.lock().unwrap();
            thread::sleep(Duration::from_millis(20));
            let mut num2 = counter2c.lock().unwrap();
            *num1 += 1;
            *num2 += 1;
        });
        handles.push(handle);
        let counter1c = Arc::clone(&counter1);
        let counter2c = Arc::clone(&counter2);
        let handle = thread::spawn(move || {
            let mut num2 = counter2c.lock().unwrap();
            thread::sleep(Duration::from_millis(20));
            let mut num1 = counter1c.lock().unwrap();
            *num1 += 1;
            *num2 += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result: {} {}", *counter1.lock().unwrap(), *counter2.lock().unwrap());
}
