use trpl::Html;

fn main() {
    
}
 async fn page_title(url: &str) -> Option<String> {
    let response_text = trpl::get(url).await.text().await;
    Html::parse(&response_text)
    .select_first("title")
    .map(|title| title.inner_html())
}

