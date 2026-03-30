use std::thread;

fn main () {
    let mut list = vec! [1, 2, 3];
    println! ("定义闭包前：{list:?}");

    let list = thread::spawn(move || {
        println! ("线程中，压入新值前：{list:?}");
        list.push(7);
        println! ("线程中，压入新值后：{list:?}");
        list
    }).join().unwrap();

    println! ("线程结束后：{list:?}");
}
