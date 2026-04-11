use std::{thread, time::Duration};

fn slow(name: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms));
    println!("'{name}' 运行了 {ms}ms");
}

fn main() {
    let fut = async {
        let a = async {
            println!("'a' 已启动。");
            slow("a", 30);
            trpl::yield_now().await;
            slow("a", 10);
            trpl::yield_now().await;
            slow("a", 20);
            trpl::yield_now().await;
            println!("'a' 已结束。");
        };

        let b = async {
            println!("'b' 已启动。");
            slow("b", 75);
            trpl::yield_now().await;
            slow("b", 10);
            trpl::yield_now().await;
            slow("b", 15);
            trpl::yield_now().await;
            slow("b", 350);
            trpl::yield_now().await;
            println!("'b' 已结束。");
        };

        trpl::select(a, b).await;
    };

    trpl::block_on(fut);
}
