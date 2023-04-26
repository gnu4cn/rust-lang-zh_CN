use std::thread;

fn main() {
    let list = vec! [1, 2, 3];
    println! ("在定义闭包之前的：{:?}", list);

    thread::spawn(move || println! ("从线程打印出的：{:?}", list))
        .join()
        .unwrap();
}
