mod lib;
use lib::*;

fn main() {
    let tweet = Tweet::new();
    println!("1 new tweet: {}", tweet.summarize());

    let news_article = NewsArticle::new();
    notify(&news_article);
    notify_longer(&news_article);
}
