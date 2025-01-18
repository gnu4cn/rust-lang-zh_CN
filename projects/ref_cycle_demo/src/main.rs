use std::cell::RefCell;
use std::rc::Rc;
use crate::List::{Cons, Nil};

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
    let a = Rc::new(
        Cons(
            5,
            RefCell::new(Rc::new(Nil))
        )
    );

    println! ("a 的初始 rc 计数 = {}", Rc::strong_count(&a));
    println! ("a 的下一条目 = {:?}", a.tail());

    let b = Rc::new(
        Cons(
            10,
            RefCell::new(Rc::clone(&a))
        )
    );

    println! ("创建 b 后 a 的 rc 计数 = {}", Rc::strong_count(&a));
    println! ("b 的初始 rc 计数 = {}", Rc::strong_count(&b));
    println! ("b 的下一条目 = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println! ("在修改 a 之后 b 的 rc 计数 = {}", Rc::strong_count(&b));
    println! ("在修改 a 之后 a 的 rc 计数 = {}", Rc::strong_count(&a));

    // 取消下面这行注释，就可以看到这里有着循环引用；
    // 他将溢出堆栈（it will overflow the stack）
    // println! ("a 的下一条目 = {:?}", a.tail());
}
