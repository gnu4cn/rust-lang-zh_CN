use std::time::Duration;

fn main() {
    let fut = async {
        let slow = async {
            trpl::sleep(Duration::from_secs(5)).await;
            "最终完成"
        };

        match timeout(slow, Duration::from_secs(2)).await {
            Ok(message) => println!("在 '{message}' 下成功"),
            Err(duration) => {
                println!("在 {} 秒后失败", duration.as_secs())
            }
        }
    };

    trpl::block_on(fut);
}
