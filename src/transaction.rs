use chrono::NaiveDate;

#[derive(Debug)]
pub struct Transaction {
    pub name: String,
    pub amount: f32,
    pub date: NaiveDate
}
