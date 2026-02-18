use crate::csv::types::CsvTransaction;
use crate::error::BudgyError;

pub trait BankCsvParser {
    fn detect(content: &[u8]) -> bool;
    fn parse(content: &[u8]) -> Result<Vec<CsvTransaction>, BudgyError>;
}
