struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println! ("正在以数据 `{}` 弃用 CustomSmartPointer！", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("我的事情"),
    };
    println! ("CustomSmartPointer 实例已创建");
    drop(c);
    println! ("CustomSmartPointer 在 main 结束前被弃用");
}
