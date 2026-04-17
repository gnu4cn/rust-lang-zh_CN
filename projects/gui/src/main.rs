use gui::Screen;

pub fn main() {
    let screen = Screen {
        components: vec! [Box::new(String::from("你好"))],
    };

    screen.run();
}
