// use trpl::Html;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    let tx_clone = tx.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("from"),
            String::from("another thread"),
        ];

        for val in vals {
            tx_clone.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {received}");
    }
}

// async fn page_title(url: &str) -> (&str, Option<String>) {
//     let txt = trpl::get(url).await.text().await;
//     let title = Html::parse(&txt)
//         .select_first("title")
//         .map(|title_element| title_element.inner_html());
//     (url, title)
// }
