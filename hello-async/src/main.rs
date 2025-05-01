use trpl::Html;

fn main() {
    println!("Hello, world!");
}

async fn page_title(url: &str) -> Option<String> {
    let resp = trpl::get(url).await;
    let resp_txt = resp.text().await;

    Html::parse(&resp_txt)
        .select_first("title")
        .map(|title_element| title_element.inner_html())
}
