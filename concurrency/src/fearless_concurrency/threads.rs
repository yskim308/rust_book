use std::thread;
use std::time::Duration;

pub fn run_basic_example() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from spawned thread");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {i} from main thread");
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}

pub fn run_ownership_example() {
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("heres a vector: {v:?}");
    });

    handle.join().unwrap();
}
