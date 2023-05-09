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
        data: String::from("c-我的事情"),
    };
    let d = CustomSmartPointer {
        data: String::from("d-其他事情"),
    };
    println! ("已创建出一些 CustomSmartPointer 实例");
}
