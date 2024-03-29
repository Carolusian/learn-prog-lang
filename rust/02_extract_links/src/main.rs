// source: https://rust-lang-nursery.github.io/rust-cookbook/web/scraping.html
use std::error::Error;
// use error_chain::error_chain; // opiniated error handling, Result<T> is from it
// use std::error::Error;
use reqwest::get as http_get; // http request library
use select::document::Document; // HTML document 
use select::predicate::Name; // XPath selector

// error_chain! {
//     // map to foreign errors
//     foreign_links {
//         ReqError(reqwest::Error);
//         IoError(std::io::Error);
//     }
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");
    // await is future that can be pattern matched
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
