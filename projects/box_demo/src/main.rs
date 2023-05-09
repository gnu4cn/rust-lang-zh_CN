use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    let boxed_string = MyBox::new(String::from("装箱的字符串"));

    let x = 5;
    let y = &x;
    println! ("{}, {}", *boxed_string, *y);
}
