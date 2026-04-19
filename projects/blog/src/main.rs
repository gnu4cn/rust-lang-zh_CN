use blog::Post;
use std::any::type_name;

fn main() {
    let mut post = Post::new();
    println! ("{}", type_name_of_val(&post));

    post.add_text("今天午饭我吃了沙拉。");

    let post = post.request_review();
    println! ("{}", type_name_of_val(&post));

    let post = post.approve();
    println! ("{}", type_name_of_val(&post));

    assert_eq! ("今天午饭我吃了沙拉。", post.content());
}

fn type_name_of_val<T>(_: &T) -> &'static str {
    type_name::<T>()
}
