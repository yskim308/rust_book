use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn run_basic_example() {
    let (tx, rx) = mpsc::channel();

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

    for message in rx {
        println!("Got: {message}");
    }
}
