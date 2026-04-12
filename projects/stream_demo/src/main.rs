use trpl::StreamExt;
use std::time::Duration;

fn main() {
    let fut = async {
        let values = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let iter = values.iter().map(|n| n * 2);
        let mut stream = trpl::stream_from_iter(iter);

        while let Some(value) = stream.next().await {
            println!("值为： {value}");
            trpl::sleep(Duration::from_secs(1)).await;
        }
    };

    trpl::block_on(fut)
}
