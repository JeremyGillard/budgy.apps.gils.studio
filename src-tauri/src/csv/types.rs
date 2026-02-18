use chrono::NaiveDate;

#[derive(Debug, Clone)]
pub struct CsvTransaction {
    pub account_iban: String,
    pub accounting_date: NaiveDate,
    pub statement_number: Option<String>,
    pub transaction_number: Option<String>,
    pub counterparty_iban: Option<String>,
    pub counterparty_name: Option<String>,
    pub counterparty_street: Option<String>,
    pub counterparty_postal_code_city: Option<String>,
    pub description: String,
    pub value_date: NaiveDate,
    pub amount_cents: i32,
    pub currency: String,
    pub bic: Option<String>,
    pub country_code: Option<String>,
    pub communication: Option<String>,
    pub transaction_type_code: String,
}
