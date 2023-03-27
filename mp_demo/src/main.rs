#![allow(dead_code)]
#![allow(unused_variables)]

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec! [
            String::from("你好"),
            String::from("自"),
            String::from("此"),
            String::from("线程"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(500));
        }
    });

    thread::spawn(move || {
        let vals = vec! [
            String::from("给"),
            String::from("你"),
            String::from("一些别的"),
            String::from("消息"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(500));
        }
    });

    for received in rx {
        println! ("收到：{}", received);
    }
}
