use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println! ("hi，生成的线程中的数字 {i} !");
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();

    for i in 1..5 {
        println! ("hi，主线程中的数字 {i} !");
        thread::sleep(Duration::from_millis(1));
    }
}
