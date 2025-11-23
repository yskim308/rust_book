use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn run_basic_example() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();

    thread::spawn(move || {
        let values = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in values {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

    thread::spawn(move || {
        let values = vec![
            String::from("more"),
            String::from("messages"),
            String::from("from"),
            String::from("second"),
            String::from("thread"),
        ];
        for val in values {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(3));
        }
    });

    for message in rx {
        println!("Got: {message}");
    }
}
