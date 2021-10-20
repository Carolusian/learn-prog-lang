// source: https://rust-lang-nursery.github.io/rust-cookbook/web/scraping.html
use error_chain::error_chain; // opiniated error handling, Result<T> is from it
use reqwest::get as http_get; // http request library
use select::document::Document; // HTML document 
use select::predicate::Name; // XPath selector


error_chain! {
    // map to foreign errors
    foreign_links {
        ReqError(reqwest::Error);
        IoError(std::io::Error);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("Hello, world!");
    let res = http_get("https://www.rust-lang.org/en-US/")
        .await?
        .text()
        .await?;

    Document::from(res.as_str())
        .find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .for_each(|x| println!("{}", x));

    Ok(())
}
