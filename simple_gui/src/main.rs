#![allow(dead_code)]
#![allow(unused_variables)]

use simple_gui::Draw;

pub struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // 具体绘制复选框的代码
        println! ("这是一个大小为：{} 像素 x {} 像素，有着选项：{:?} 的复选框；", self.width, self.height, self.options);
    }
}

use simple_gui::Screen;

pub fn main() {
    let screen = Screen {
        components: vec! [Box::new(String::from("你好"))],
    };

    screen.run();
}
