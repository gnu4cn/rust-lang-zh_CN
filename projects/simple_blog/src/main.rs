#![allow(dead_code)]
#![allow(unused_variables)]

use simple_blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("今天午饭我吃了沙拉。");
    assert_eq! ("", post.content());

    post.request_review();
    assert_eq! ("", post.content());

    post.approve();
    assert_eq! ("今天午饭我吃了沙拉。", post.content());
}
