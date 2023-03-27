#![allow(dead_code)]
#![allow(unused_variables)]

use std::thread;

fn main() {
    let v = vec! [1, 2, 3];

    let handle = thread::spawn(move || {
        println! ("这里有个矢量值：{:?}", &v);
    });

    handle.join().unwrap();
}
