use trpl::Html;

async fn main() {
    let args: Vec<String> = std::env::args().collect();
    let url = &args[1];
    match page_title(url).await {
        Some(title) => println!("{url} 的标题是 {title}"),
        None => println!("{url} 没有标题"),
    }
}

async fn page_title(url: &str) -> Option<String> {
    let response = trpl::get(url).await.text().await;
    Html::parse(&response)
        .select_first("title")
        .map(|title| title.inner_html())
}


