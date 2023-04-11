use aggregator::{Summary, Tweet, NewsArticle, Pair};

pub fn notify<T: Summary>(item: &T) {
    println! ("突发新闻！{}", item.summarize());
}

fn return_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "当然，如同你或许已经知道的一样，朋友们"
        ),
        reply: false,
        retweet: false,
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "当然，跟大家已经清楚的一样了，朋友们",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 条新推文: {}", tweet.summarize());
    notify(&tweet);


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
    notify(&article);

    println! ("1 条旧推文: {}", return_summarizable().summarize());

    let pair = Pair::new(5, 10);
    pair.cmp_display();

    let pair = Pair::new("这是一个测试", "This is a test.");
    pair.cmp_display();

    println! ("{}", 3.to_string());
}
