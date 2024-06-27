use crate::filter::Filter;
use chrono::NaiveDate;
use thirtyfour::prelude::*;

pub enum NatwestTransactionType {

}

pub struct NatwestFilter {
    date_range: (Option<NaiveDate>, Option<NaiveDate>),
    search: Option<String>,
    value: Option<f32>,
    transaction_type: Option<NatwestTransactionType>
}

impl Filter for NatwestFilter {
    fn apply_to_page(&self, webdriver: &WebDriver) -> WebDriverResult<()> {
        Ok(())
    }
}

impl Default for NatwestFilter {
    fn default() -> Self {
        Self {
            date_range: (None, None),
            search: None,
            value: None,
            transaction_type: None
        }
    }
}

impl NatwestFilter {
    pub fn with_search(mut self, search: String) -> Self {
        self.search = Some(search);
        self
    }

    pub fn with_date_range(mut self, date_range: (Option<NaiveDate>, Option<NaiveDate>)) -> Self {
        self.date_range = date_range;
        self
    }

    pub fn with_value(mut self, value: f32) -> Self {
        self.value = Some(value);
        self
    }

    pub fn with_transaction_type(mut self, transaction_type: NatwestTransactionType) -> Self {
        self.transaction_type = Some(transaction_type);
        self
    }
}
