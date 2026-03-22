use aggregator::{SocialPost, Summary, NewsArticle};

pub fn notify(item: &impl Summary) {
    println! ("突发新闻！{}", item.summarize());
}

fn return_summarizable() -> impl Summary {
    SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from(
            "当然，正如咱们或许已经知道的一样，朋友们"
        ),
        reply: false,
        retweet: false,
    }
}

fn main() {
    let post = SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from(
            "当然，跟大家已经知道的一样，朋友们",
        ),
        reply: false,
        retweet: false,
    };

    println! ("1 个新帖子：{}", post.summarize());


    let article = NewsArticle {
        headline: String::from("企鹅队赢得斯坦利杯锦标赛！"),
        location: String::from("美国，宾夕法尼亚州，匹兹堡"),
        author: String::from("Iceburgh"),
        content: String::from(
            "匹兹堡企鹅队再度成为美国曲棍球联盟 \
            NHL 中的最佳球队。"
        ),
    };

    println! ("有新文章可读！{}", article.summarize());
}
