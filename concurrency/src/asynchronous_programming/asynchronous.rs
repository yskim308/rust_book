use std::time::Duration;

pub fn run_basic_example() {
    trpl::run(async {
        let handle = trpl::spawn_task(async {
            for i in 1..10 {
                println!("hi number {i} from first task");
                trpl::sleep(Duration::from_millis(50)).await;
            }
        });

        for i in 1..5 {
            println!("hi number {i} from second task");
            trpl::sleep(Duration::from_millis(50)).await;
        }
        handle.await.unwrap();
    });
}

pub fn run_channel_example() {
    trpl::run(async {
        let (tx, mut rx) = trpl::channel();

        let val = String::from("hi");
        tx.send(val).unwrap();

        let receieved = rx.recv().await.unwrap();
        println!("receieved `{receieved}`");
    })
}
