use trpl::{Either, Html};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let fut_result = async {
        let title_fut_1 = page_title(&args[1]);
        let title_fut_2 = page_title(&args[2]);

        let (url, maybe_title) =
            match trpl::select(title_fut_1, title_fut_2).await {
                Either::Left(left) => left,
                Either::Right(right) => right,
            };

        println! ("{url} 先返回");
        match maybe_title {
            Some(title) => println! ("其页面标题为： '{title}'"),
            None => println! ("他没有标题。"),
        }
    };

    trpl::block_on(fut_result)
}

async fn page_title(url: &str) -> (&str, Option<String>) {
    let response_text = trpl::get(url).await.text().await;
    let title = Html::parse(&response_text)
        .select_first("title")
        .map(|title| title.inner_html());
    (url, title)
}


