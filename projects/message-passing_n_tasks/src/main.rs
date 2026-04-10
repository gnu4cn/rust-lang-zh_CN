use std::time::Duration;

fn main() {
    let fut = async {
        let (tx, mut rx) = trpl::channel();

        let vals = vec! [
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("future"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            trpl::sleep(Duration::from_millis(500)).await;
        }

        while let Some(value) = rx.recv().await {
            println!("收到 '{value}'");
        }
    };

    trpl::block_on(fut);

    println! ("非并发部分");
}
