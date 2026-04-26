use std::{future, time::Duration};
use trpl::{Either, Html, StreamExt};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    trpl::block_on(async {
        // let title_fut_1 = page_title(&args[1]);
        // let title_fut_2 = page_title(&args[2]);

        // let (url, maybe_title) = match trpl::select(title_fut_1, title_fut_2).await {
        //     Either::Left(left) => left,
        //     Either::Right(right) => right,
        // };

        // println!("{url} returned first");
        // match maybe_title {
        //     Some(title) => println!("Its page title was: '{title}'"),
        //     None => println!("It had no title."),
        // }

        // let (tx, mut rx) = trpl::channel();

        // let vals = vec![
        //     String::from("hi"),
        //     String::from("from"),
        //     String::from("the"),
        //     String::from("future"),
        // ];

        // let send_fut = timeout(
        //     async move {
        //         for val in vals {
        //             trpl::sleep(Duration::from_millis(500)).await;
        //             tx.send(val).unwrap();
        //         }
        //     },
        //     Duration::from_millis(400),
        // );
        // trpl::spawn_task(send_fut);

        // while let Some(value) = rx.recv().await {
        //     println!("received '{value}'");
        // }

        let values = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let iter = values.iter().map(|n| n * 2);
        let mut stream = trpl::stream_from_iter(iter);

        while let Some(value) = stream.next().await {
            println!("The value was: {value}");
        }
    })
}
async fn page_title(url: &str) -> (&str, Option<String>) {
    let response_text = trpl::get(url).await.text().await;
    let title = Html::parse(&response_text)
        .select_first("title")
        .map(|title| title.inner_html());
    (url, title)
}

async fn timeout<F: Future>(future_to_try: F, max_time: Duration) -> Result<F::Output, Duration> {
    match trpl::select(future_to_try, trpl::sleep(max_time)).await {
        Either::Left(output) => Ok(output),
        Either::Right(_) => Err(max_time),
    }
}
