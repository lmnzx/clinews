use dotenv::dotenv;
use std::error::Error;

use newsapi::{get_articles, Articles};

fn render_articles(articles: &Articles) {
    for a in &articles.articles {
        println!("> {}", a.title);
        println!("- {}", a.url);
        println!();
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    dotenv()?;

    let api_key = std::env::var("API_KEY")?;

    let url = &format!(
        "https://newsapi.org/v2/top-headlines?country=us&apiKey={}",
        api_key
    );
    let articles = get_articles(url)?;
    render_articles(&articles);
    Ok(())
}
