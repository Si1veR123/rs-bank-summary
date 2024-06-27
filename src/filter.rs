use thirtyfour::prelude::*;

pub trait Filter {
    fn apply_to_page(&self, webdriver: &WebDriver) -> WebDriverResult<()>;
}
