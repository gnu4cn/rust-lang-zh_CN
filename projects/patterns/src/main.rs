fn main() {
    enum Message {
        Hello { id: u32 },
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello { id: id @ 3..=7 } => {
            println! ("找到位于范围内的 id: {id}")
        }
        Message::Hello { id: 10..=12 } => {
            println! ("找到位于另一范围内 id")
        }
        Message::Hello { id } => println! ("找到别的 id: {id}"),
    }
}
