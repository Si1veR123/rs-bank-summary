use thirtyfour::prelude::*;

use crate::{
    filter::Filter,
    transaction::Transaction
};

#[allow(async_fn_in_trait)]
pub trait Scraper {
    type Filter: Filter;

    async fn open_login(driver: &WebDriver) -> WebDriverResult<()>;
    async fn await_login(driver: &WebDriver) -> WebDriverResult<()>;
    async fn open_transactions(driver: &WebDriver, filter: Option<Self::Filter>) -> WebDriverResult<()>;
    async fn transactions(driver: &WebDriver) -> WebDriverResult<Vec<Transaction>>;
}