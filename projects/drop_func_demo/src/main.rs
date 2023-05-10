struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println! ("正在使用数据 `{}` 弃用 CustomSmartPointer！", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("我的事情"),
    };

    println! ("已创建出一个 CustomSmartPointer 实例。");
    drop(c);
    println! ("在 main 结束之前这个 CustomSmartPointer 已被弃用。")
}
