use bank_summary::{
    barclays::{
        filter::{
            BarclaysFilter,
            MoneyDirection
        },
        scraper::BarclaysScraper,
    }, filter::Filter, scraper::Scraper, transaction::Transaction
};

use thirtyfour::{
    prelude::*,
    ChromeCapabilities
};
use async_std::task::block_on;

async fn get_transactions<S: Scraper, F: Filter>(driver: &WebDriver, filter: Option<F>) -> WebDriverResult<Vec<Transaction>> {
    S::open_login(driver).await?;
    S::await_login(driver).await?;
    S::open_transactions(driver, filter).await?;
    S::transactions(driver).await
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let barclays_filter = BarclaysFilter::default()
        .with_search(String::from("money"))
        .with_direction(MoneyDirection::In);

    let capabilities = ChromeCapabilities::new();
    let webdriver = block_on(WebDriver::new("http://localhost:4444/wd/hub/session", capabilities))?;
    
    let transactions = block_on(get_transactions::<BarclaysScraper, _>(&webdriver, Some(barclays_filter)));

    println!("{:?}", transactions);

    Ok(())
}
