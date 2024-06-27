use std::{str::FromStr, time::Duration};

use crate::{
    filter::Filter,
    scraper::Scraper,
    transaction::Transaction,
    barclays::filter::BarclaysFilter
};

use chrono::NaiveDate;
use thirtyfour::{action_chain::ActionChain, common::action::Action, prelude::*};

pub struct BarclaysScraper;

impl Scraper for BarclaysScraper {
    type Filter = BarclaysFilter;

    async fn open_login(driver: &WebDriver) -> WebDriverResult<()> {
        driver.goto("https://bank.barclays.co.uk/olb/authlogin/loginAppContainer.do").await
    }

    async fn await_login(driver: &WebDriver) -> WebDriverResult<()> {
        driver.query(By::Id("Logout"))
            .wait(Duration::MAX, Duration::from_millis(500))
            .and_displayed().first().await?;

        Ok(())
    }

    async fn open_transactions(driver: &WebDriver, filter: Option<Self::Filter>) -> WebDriverResult<()> {
        // TODO: Better way to select this?
        let account_details_button = driver.find(By::Css(".c-account__detail a")).await?;
        account_details_button.click().await?;

        driver.query(By::Id("advancedSearch"))
            .and_clickable()
            .first().await?
            .click().await?;

        if let Some(f) = filter {
            f.apply_to_page(driver)?;
        }

        let search_button = driver.find(By::Id("advanceSearchSubmitBtn")).await?;
        // Using click alone has a ElementClickIntercepted error
        driver.action_chain().click_element(&search_button).perform().await?;

        Ok(())
    }

    async fn transactions(driver: &WebDriver) -> WebDriverResult<Vec<Transaction>> {
        let mut all_transactions = Vec::new();

        let pagination_buttons_res = driver.find_all(By::Css("button[data-testid=pagination-item]")).await?;

        let mut page_number = 0;
        loop {
            let transaction_days = driver.find_all(By::ClassName("transactionList")).await?;
            for day in transaction_days {
                let transactions = day.find_all(By::Tag("button")).await?;
                for transaction_button in transactions {
                    let transaction = transaction_from_barclays_element(&transaction_button).await;

                    if let Some(t) = transaction {
                        all_transactions.push(t);
                    }
                }
            }

            page_number += 1;
            if let Some(next_page_button) = pagination_buttons_res.get(page_number) {
                next_page_button.click().await?;
            } else {
                break
            }
        }

        Ok(all_transactions)
    }
}

async fn transaction_from_barclays_element(transaction_element: &WebElement) -> Option<Transaction> {
    let transaction_amount_string = transaction_element.find(By::Id("txnamount0")).await.ok()?
        .find(By::Tag("p")).await.ok()?
        .text().await.ok()?;

    let is_income = transaction_amount_string.chars().nth(0)? == '+';

    let transaction_amount = if is_income {
        f32::from_str(&transaction_amount_string[3..]).ok()?
    } else {
        f32::from_str(&transaction_amount_string[1..]).ok()?
    };

    let transaction_name = transaction_element.find(By::Id("narrativeLine0")).await.ok()?
        .text().await.ok()?;

    let date_text = transaction_element.parent().await.ok()?.parent().await.ok()?
        .find(By::Id("txnDate")).await.ok()?
        .text().await.ok()?;

    let date = NaiveDate::parse_from_str(&date_text, "%A, %e %B %Y").ok()?;

    Some(Transaction {
        name: transaction_name,
        amount: transaction_amount,
        date
    })
}
