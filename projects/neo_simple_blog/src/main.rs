#![allow(dead_code)]
#![allow(unused_variables)]

use neo_simple_blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("这是一个博客帖子。");

    let post = post.request_review();
    let post = post.approve();

    assert_eq! ("这是一个博客帖子。", post.content());
}
