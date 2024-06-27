use thirtyfour::prelude::*;

use crate::{
    filter::Filter,
    transaction::Transaction
};

pub trait Scraper {
    async fn open_login(driver: &WebDriver) -> WebDriverResult<()>;
    async fn await_login(driver: &WebDriver) -> WebDriverResult<()>;
    async fn open_transactions<F: Filter>(driver: &WebDriver, filter: Option<F>) -> WebDriverResult<()>;
    async fn transactions(driver: &WebDriver) -> WebDriverResult<Vec<Transaction>>;
}