use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println! ("创建 a 后的计数 = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println! ("创建 b 后的计数 = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println! ("创建 c 后的计数 = {}", Rc::strong_count(&a));
    }

    println! ("c 超出作用域后的计数 = {}", Rc::strong_count(&a));
}
