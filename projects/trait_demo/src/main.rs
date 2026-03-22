use aggregator::{SocialPost, Summary};

fn main() {
    let post = SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from(
            "当然，跟大家已经知道的一样，朋友们",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 个新帖子: {}", post.summarize());
}
