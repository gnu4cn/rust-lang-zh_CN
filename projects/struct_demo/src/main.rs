#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("hector.peng@rust-lang.xfoss.com"),
        username: String::from("hector.peng"),
        ..user1
    };

    println! ("{:#?}", user1);
}
