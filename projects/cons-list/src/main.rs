use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}


fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println! ("a 的初始 rc 计数 = {}", Rc::strong_count(&a));
    println! ("a 的下一条目 = {:?}", a.tail().unwrap());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println! ("b 创建后 a 的 rc 计数 = {}", Rc::strong_count(&a));
    println! ("b 的初始 rc 计数 = {}", Rc::strong_count(&b));
    println! ("b 的下一条目 = {:?}", b.tail().unwrap());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println! ("修改 a 之后 b 的 rc 计数 = {}", Rc::strong_count(&b));
    println! ("修改 a 之后 a 的 rc 计数 = {}", Rc::strong_count(&a));

    // 取消注释下一行，就可以看到我们有个循环；
    // 他将导致栈溢出。
    println! ("a 的下一条目 = {:?}", a.tail());
}
