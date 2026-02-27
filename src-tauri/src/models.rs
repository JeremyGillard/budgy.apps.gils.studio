use diesel::prelude::*;
use serde::Serialize;

use crate::schema::*;

#[derive(Queryable, Selectable, Serialize, Debug)]
#[diesel(table_name = accounts)]
pub struct Account {
    pub id: i32,
    pub iban: String,
    pub name: Option<String>,
    pub currency: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Insertable)]
#[diesel(table_name = accounts)]
pub struct NewAccount<'a> {
    pub iban: &'a str,
    pub name: Option<&'a str>,
    pub currency: &'a str,
    pub created_at: &'a str,
    pub updated_at: &'a str,
}

#[derive(Queryable, Selectable, Serialize, Debug)]
#[diesel(table_name = transaction_types)]
pub struct TransactionType {
    pub id: i32,
    pub code: String,
    pub label_fr: String,
    pub label_en: String,
    pub direction: String,
}

#[derive(Queryable, Selectable, Serialize, Debug)]
#[diesel(table_name = imports)]
pub struct Import {
    pub id: i32,
    pub account_id: i32,
    pub file_name: String,
    pub file_hash: String,
    pub record_count: i32,
    pub date_range_from: Option<String>,
    pub date_range_to: Option<String>,
    pub imported_at: String,
}

#[derive(Insertable)]
#[diesel(table_name = imports)]
pub struct NewImport<'a> {
    pub account_id: i32,
    pub file_name: &'a str,
    pub file_hash: &'a str,
    pub record_count: i32,
    pub date_range_from: Option<&'a str>,
    pub date_range_to: Option<&'a str>,
    pub imported_at: &'a str,
}

#[derive(Queryable, Selectable, Serialize, Debug)]
#[diesel(table_name = import_metadata)]
pub struct ImportMetadata {
    pub id: i32,
    pub import_id: i32,
    pub key: String,
    pub value: String,
}

#[derive(Insertable)]
#[diesel(table_name = import_metadata)]
pub struct NewImportMetadata<'a> {
    pub import_id: i32,
    pub key: &'a str,
    pub value: &'a str,
}

#[derive(Queryable, Selectable, Serialize, Debug)]
#[diesel(table_name = transactions)]
pub struct Transaction {
    pub id: i32,
    pub import_id: i32,
    pub account_id: i32,
    pub transaction_type_id: Option<i32>,
    pub accounting_date: String,
    pub statement_number: String,
    pub sequence_number: i32,
    pub counterparty_account: Option<String>,
    pub counterparty_name: Option<String>,
    pub counterparty_street: Option<String>,
    pub counterparty_city: Option<String>,
    pub transaction_description: String,
    pub value_date: String,
    pub amount_cents: i32,
    pub currency: String,
    pub bic: Option<String>,
    pub country_code: Option<String>,
    pub communication: Option<String>,
    pub row_hash: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Insertable)]
#[diesel(table_name = transactions)]
pub struct NewTransaction<'a> {
    pub import_id: i32,
    pub account_id: i32,
    pub transaction_type_id: Option<i32>,
    pub accounting_date: &'a str,
    pub statement_number: &'a str,
    pub sequence_number: i32,
    pub counterparty_account: Option<&'a str>,
    pub counterparty_name: Option<&'a str>,
    pub counterparty_street: Option<&'a str>,
    pub counterparty_city: Option<&'a str>,
    pub transaction_description: &'a str,
    pub value_date: &'a str,
    pub amount_cents: i32,
    pub currency: &'a str,
    pub bic: Option<&'a str>,
    pub country_code: Option<&'a str>,
    pub communication: Option<&'a str>,
    pub row_hash: &'a str,
    pub created_at: &'a str,
    pub updated_at: &'a str,
}
