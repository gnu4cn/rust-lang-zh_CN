struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };


    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    println! ("{}", user2.email);
}
