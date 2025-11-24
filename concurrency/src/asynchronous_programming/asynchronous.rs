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
        let tx1 = tx.clone();

        let tx_future = async move {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];
            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let tx1_future = async move {
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];

            for val in vals {
                tx1.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let rx_future = async {
            while let Some(value) = rx.recv().await {
                println!("received `{value}`");
            }
        };

        trpl::join3(tx_future, tx1_future, rx_future).await;
    })
}
