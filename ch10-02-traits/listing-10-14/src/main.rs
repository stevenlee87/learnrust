use aggregator::{self, NewsArticle, Summary};

fn main() {
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());
}

/*
cd ~/project/learnrust/ch10-02-traits/listing-10-14
cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.04s
     Running `target/debug/aggregator`
New article available! (Read more...)
 */