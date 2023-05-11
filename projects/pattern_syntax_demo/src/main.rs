fn main() {
    enum Message {
        Hello { id: u32 },
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => println! ("找到位于范围内的一个 id: {}", id_variable),
        Message::Hello { id: 10..=12 } => {
            println! ("找到位于另一范围的一个 id");
        },
        Message::Hello { id } => println! ("找到别的一个 id: {}", id),
    }

    let level = 0.8;

    match level {
        l if l >= 1.0 => println! ("超出额度"),
        l if l >= 0.9 => println! ("超出 90% 额度"),
        l if l >= 0.75 => println! ("超出 75% 额度"),
        _ => (),
    }
}
