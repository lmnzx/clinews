use console::style;
use dotenv::dotenv;
use std::error::Error;

use newsapi::{Article, Country, Endpoint, NewsApi};

fn render_articles(articles: &Vec<Article>) {
    println!("{}", style("Top Headlines\n\n").green().bold().underlined());
    for a in articles {
        println!("{}", style(&a.title()).bold().cyan());
        println!("{}", style(&a.url()).yellow());
        println!();
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv()?;

    let api_key = std::env::var("API_KEY")?;

    let mut newsapi = NewsApi::new(&api_key);
    newsapi
        .endpoint(Endpoint::TopHeadlines)
        .country(Country::Us);

    let newsapi_response = newsapi.fetch_async().await?;

    render_articles(&newsapi_response.articles());

    Ok(())
}
