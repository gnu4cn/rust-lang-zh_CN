use std::time::Duration;
use trpl::Either;

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

async fn timeout<F: Future>(
    future_to_try: F,
    max_time: Duration,
) -> Result<F::Output, Duration> {
    match trpl::select(future_to_try, trpl::sleep(max_time)).await {
        Either::Left(output) => Ok(output),
        Either::Right(_) => Err(max_time),
    }
}
