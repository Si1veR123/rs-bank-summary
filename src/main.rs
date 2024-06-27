use bank_summary::{
    barclays::{
        filter::{
            BarclaysFilter,
            MoneyDirection
        },
        scraper::BarclaysScraper,
    }, scraper::Scraper, transaction::Transaction
};

use thirtyfour::{
    prelude::*,
    DesiredCapabilities
};

async fn get_transactions<S: Scraper>(driver: &WebDriver, filter: Option<S::Filter>) -> WebDriverResult<Vec<Transaction>> {
    S::open_login(driver).await?;
    S::await_login(driver).await?;
    S::open_transactions(driver, filter).await?;
    std::thread::sleep(std::time::Duration::from_secs(5));
    S::transactions(driver).await
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let barclays_filter = BarclaysFilter::default()
        .with_search(String::from("money"))
        .with_direction(MoneyDirection::In);

    let mut capabilities = DesiredCapabilities::chrome();
    capabilities.set_binary(r"C:\Program Files\BraveSoftware\Brave-Browser\Application\brave.exe")?;

    let webdriver = WebDriver::new("http://localhost:9515/wd/hub/session", capabilities).await?;
    
    let transactions = get_transactions::<BarclaysScraper>(&webdriver, Some(barclays_filter)).await;

    webdriver.quit().await?;

    println!("{:?}", transactions);

    Ok(())
}
