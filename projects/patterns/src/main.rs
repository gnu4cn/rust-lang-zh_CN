enum Color {
    Rgb(u32, u32, u32),
    Hsv(u32, u32, u32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

fn main() {
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println! ("将颜色改为红 {r}、绿 {g} 及蓝 {b}");
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println! ("将颜色改为色调 {h}、饱和度 {s} 及颜色值 {v}");
        }
        _ => (),
    }
}
