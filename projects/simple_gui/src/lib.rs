#![allow(dead_code)]
#![allow(unused_variables)]

pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // 具体绘制按钮的代码
        println! ("这是一个大小：{} 像素 x {} 像素，有着 “{}” 标签的按钮；", self.width, self.height, self.label);
    }
}
