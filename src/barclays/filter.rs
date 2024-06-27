use crate::filter::Filter;
use chrono::NaiveDate;
use thirtyfour::prelude::*;

pub enum BarclaysTransactionType {
    BillPayment,
    BulkPayment
}

pub enum MoneyDirection {
    In,
    Out
}

pub struct BarclaysFilter {
    pub date_range: (Option<NaiveDate>, Option<NaiveDate>),
    pub search: Option<String>,
    pub value_range: (Option<f32>, Option<f32>),
    pub transaction_type: Option<BarclaysTransactionType>,
    pub direction: Option<MoneyDirection>
}

impl Filter for BarclaysFilter {
    fn apply_to_page(&self, webdriver: &WebDriver) -> WebDriverResult<()> {
        Ok(())
    }
}

impl Default for BarclaysFilter {
    fn default() -> Self {
        Self {
            date_range: (None, None),
            search: None,
            value_range: (None, None),
            transaction_type: None,
            direction: None,
        }
    }
}

impl BarclaysFilter {
    pub fn with_search(mut self, search: String) -> Self {
        self.search = Some(search);
        self
    }

    pub fn with_date_range(mut self, date_range: (Option<NaiveDate>, Option<NaiveDate>)) -> Self {
        self.date_range = date_range;
        self
    }

    pub fn with_value_range(mut self, value_range: (Option<f32>, Option<f32>)) -> Self {
        self.value_range = value_range;
        self
    }

    pub fn with_transaction_type(mut self, transaction_type: BarclaysTransactionType) -> Self {
        self.transaction_type = Some(transaction_type);
        self
    }

    pub fn with_direction(mut self, direction: MoneyDirection) -> Self {
        self.direction = Some(direction);
        self
    }
}
