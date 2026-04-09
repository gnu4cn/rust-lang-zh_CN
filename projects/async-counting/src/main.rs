use std::time::Duration;

fn main() {
    let fut_result = async {
        let fut1 = async {
            for i in 1..10 {
                println!("hi 来自第一个任务的数字 {i} !");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let fut2 = async {
            for i in 1..5 {
                println!("hi 来自第二个任务的数字 {i} !");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        trpl::join(fut1, fut2).await;
    };

    trpl::block_on(fut_result);
}
