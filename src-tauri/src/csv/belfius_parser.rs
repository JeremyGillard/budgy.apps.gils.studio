use chrono::NaiveDate;
use csv::ReaderBuilder;

use crate::csv::parser_trait::BankCsvParser;
use crate::csv::types::CsvTransaction;
use crate::error::BudgyError;

pub struct BelfiusParser;

const METADATA_LINES: usize = 13; // 12 metadata + 1 header

pub fn parse_date(s: &str) -> Result<NaiveDate, BudgyError> {
    NaiveDate::parse_from_str(s.trim(), "%d/%m/%Y")
        .map_err(|e| BudgyError::CsvParse(format!("Invalid date '{}': {}", s, e)))
}

pub fn parse_amount_cents(s: &str) -> Result<i32, BudgyError> {
    let cleaned = s.trim().replace(',', ".");
    let float: f64 = cleaned
        .parse()
        .map_err(|e| BudgyError::CsvParse(format!("Invalid amount '{}': {}", s, e)))?;
    Ok((float * 100.0).round() as i32)
}

pub fn extract_transaction_type_code(description: &str) -> String {
    let desc = description.trim();

    if desc.starts_with("BANCONTACT ACHAT") {
        return "BANCONTACT_PURCHASE".to_string();
    }
    if desc.starts_with("BANCONTACT APP") || desc.starts_with("BANCONTACT RETRAIT") {
        if desc.contains("RETRAIT") {
            return "BANCONTACT_ATM".to_string();
        }
        return "BANCONTACT_APP".to_string();
    }
    if desc.starts_with("RETRAIT D'ESPECES") || desc.starts_with("RETRAIT D\u{2019}ESPECES") {
        return "ATM_WITHDRAWAL".to_string();
    }
    if desc.starts_with("PAIEMENT DEBITMASTERCARD") {
        return "DEBIT_MASTERCARD".to_string();
    }
    if desc.starts_with("MASTERCARD RELEVE") {
        return "MASTERCARD_STATEMENT".to_string();
    }
    if desc.starts_with("VIREMENT BELFIUS MOBILE") {
        return "TRANSFER_OUT".to_string();
    }
    if desc.starts_with("VIREMENT PAYCONIQ") {
        return "PAYCONIQ".to_string();
    }
    if desc.starts_with("VERSEMENT INSTANTANE") {
        return "INSTANT_TRANSFER_IN".to_string();
    }
    if desc.starts_with("VERSEMENT") {
        return "TRANSFER_IN".to_string();
    }
    if desc.starts_with("ORDRE PERMANENT") {
        return "STANDING_ORDER".to_string();
    }
    if desc.contains("DOMICILIATION") {
        return "DIRECT_DEBIT".to_string();
    }
    if desc.starts_with("CHARGEMENT") {
        return "CARD_LOAD".to_string();
    }
    if desc.starts_with("DECHARGEMENT") {
        return "CARD_UNLOAD".to_string();
    }
    if desc.starts_with("DEPOT ESPECES") {
        return "CASH_DEPOSIT".to_string();
    }
    if desc.starts_with("FRAIS") || desc.starts_with("PARTICIPATION AUX FRAIS") {
        return "FEES".to_string();
    }
    if desc.starts_with("ARGENT RECU") {
        return "MOBILE_RECEIVE".to_string();
    }

    "OTHER".to_string()
}

fn decode_windows_1252(bytes: &[u8]) -> String {
    let (cow, _, _) = encoding_rs::WINDOWS_1252.decode(bytes);
    cow.into_owned()
}

fn opt_str(s: &str) -> Option<String> {
    let trimmed = s.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

fn parse_row(record: &csv::StringRecord) -> Result<CsvTransaction, BudgyError> {
    if record.len() < 13 {
        return Err(BudgyError::CsvParse(format!(
            "Expected at least 13 columns, got {}",
            record.len()
        )));
    }

    let account_iban = record[0].trim().to_string();
    let accounting_date = parse_date(&record[1])?;
    let statement_number = opt_str(&record[2]);
    let transaction_number = opt_str(&record[3]);
    let counterparty_iban = opt_str(&record[4]);
    let counterparty_name = opt_str(&record[5]);
    let counterparty_street = opt_str(&record[6]);
    let counterparty_postal_code_city = opt_str(&record[7]);
    let description = record[8].trim().to_string();
    let value_date = parse_date(&record[9])?;
    let amount_cents = parse_amount_cents(&record[10])?;
    let currency = record[11].trim().to_string();
    let bic = opt_str(&record[12]);
    let country_code = if record.len() > 13 {
        opt_str(&record[13])
    } else {
        None
    };
    let communication = if record.len() > 14 {
        opt_str(&record[14])
    } else {
        None
    };

    let transaction_type_code = extract_transaction_type_code(&description);

    Ok(CsvTransaction {
        account_iban,
        accounting_date,
        statement_number,
        transaction_number,
        counterparty_iban,
        counterparty_name,
        counterparty_street,
        counterparty_postal_code_city,
        description,
        value_date,
        amount_cents,
        currency,
        bic,
        country_code,
        communication,
        transaction_type_code,
    })
}

impl BankCsvParser for BelfiusParser {
    fn detect(content: &[u8]) -> bool {
        let text = decode_windows_1252(content);
        let first_lines: String = text.lines().take(3).collect::<Vec<_>>().join("\n");
        first_lines.contains("Date de comptabilisation")
            || first_lines.contains("Compte;Date de comptabilisation")
    }

    fn parse(content: &[u8]) -> Result<Vec<CsvTransaction>, BudgyError> {
        let text = decode_windows_1252(content);
        let lines: Vec<&str> = text.lines().collect();

        if lines.len() < METADATA_LINES {
            return Err(BudgyError::CsvParse(
                "File too short to be a Belfius CSV".to_string(),
            ));
        }

        // Skip metadata lines (12 lines) and header (1 line) — data starts at index 13
        let data_section = lines[METADATA_LINES..].join("\n");

        let mut reader = ReaderBuilder::new()
            .delimiter(b';')
            .has_headers(false)
            .flexible(true)
            .from_reader(data_section.as_bytes());

        let mut transactions = Vec::new();
        for result in reader.records() {
            let record = result.map_err(|e| BudgyError::CsvParse(e.to_string()))?;
            // Skip empty rows
            if record.iter().all(|f| f.trim().is_empty()) {
                continue;
            }
            transactions.push(parse_row(&record)?);
        }

        Ok(transactions)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_date() {
        let date = parse_date("31/12/2024").unwrap();
        assert_eq!(date, NaiveDate::from_ymd_opt(2024, 12, 31).unwrap());
    }

    #[test]
    fn test_parse_date_leading_zeros() {
        let date = parse_date("01/08/2024").unwrap();
        assert_eq!(date, NaiveDate::from_ymd_opt(2024, 8, 1).unwrap());
    }

    #[test]
    fn test_parse_date_invalid() {
        assert!(parse_date("2024-12-31").is_err());
        assert!(parse_date("").is_err());
    }

    #[test]
    fn test_parse_amount_negative() {
        assert_eq!(parse_amount_cents("-50,00").unwrap(), -5000);
    }

    #[test]
    fn test_parse_amount_positive() {
        assert_eq!(parse_amount_cents("1422,72").unwrap(), 142272);
    }

    #[test]
    fn test_parse_amount_small() {
        assert_eq!(parse_amount_cents("-8,90").unwrap(), -890);
    }

    #[test]
    fn test_parse_amount_invalid() {
        assert!(parse_amount_cents("abc").is_err());
    }

    #[test]
    fn test_extract_type_bancontact_purchase() {
        assert_eq!(
            extract_transaction_type_code("BANCONTACT ACHAT - COLRUYT ETTRBK 3104"),
            "BANCONTACT_PURCHASE"
        );
    }

    #[test]
    fn test_extract_type_atm_withdrawal() {
        assert_eq!(
            extract_transaction_type_code("RETRAIT D'ESPECES AVEC CARTE N° 5169"),
            "ATM_WITHDRAWAL"
        );
    }

    #[test]
    fn test_extract_type_debit_mastercard() {
        assert_eq!(
            extract_transaction_type_code("PAIEMENT DEBITMASTERCARD 28/12 LSP*Oakberry"),
            "DEBIT_MASTERCARD"
        );
    }

    #[test]
    fn test_extract_type_transfer_out() {
        assert_eq!(
            extract_transaction_type_code("VIREMENT BELFIUS MOBILE VERS BE51 0834"),
            "TRANSFER_OUT"
        );
    }

    #[test]
    fn test_extract_type_transfer_in() {
        assert_eq!(
            extract_transaction_type_code("VERSEMENT DU BE29 0689 0044 5064 ENSPIRIT"),
            "TRANSFER_IN"
        );
    }

    #[test]
    fn test_extract_type_instant_transfer() {
        assert_eq!(
            extract_transaction_type_code("VERSEMENT INSTANTANE DE BE18 0634"),
            "INSTANT_TRANSFER_IN"
        );
    }

    #[test]
    fn test_extract_type_standing_order() {
        assert_eq!(
            extract_transaction_type_code("ORDRE PERMANENT 18895205 POUR BE84"),
            "STANDING_ORDER"
        );
    }

    #[test]
    fn test_extract_type_direct_debit() {
        assert_eq!(
            extract_transaction_type_code("VOTRE DOMICILIATION EUROPEENNE 400000044097 POUR"),
            "DIRECT_DEBIT"
        );
    }

    #[test]
    fn test_extract_type_fees() {
        assert_eq!(
            extract_transaction_type_code("FRAIS SUR ORDRE(S) NON-EXECUTE(S)"),
            "FEES"
        );
        assert_eq!(
            extract_transaction_type_code("PARTICIPATION AUX FRAIS DE GESTION"),
            "FEES"
        );
    }

    #[test]
    fn test_extract_type_card_load() {
        assert_eq!(
            extract_transaction_type_code("CHARGEMENT DE LA CARTE MASTERCARD"),
            "CARD_LOAD"
        );
    }

    #[test]
    fn test_extract_type_card_unload() {
        assert_eq!(
            extract_transaction_type_code("DECHARGEMENT DE LA CARTE MASTERCARD"),
            "CARD_UNLOAD"
        );
    }

    #[test]
    fn test_extract_type_cash_deposit() {
        assert_eq!(
            extract_transaction_type_code("DEPOT ESPECES AVEC CARTE"),
            "CASH_DEPOSIT"
        );
    }

    #[test]
    fn test_extract_type_mobile_receive() {
        assert_eq!(
            extract_transaction_type_code("ARGENT RECU VIA VOTRE APP MOBILE"),
            "MOBILE_RECEIVE"
        );
    }

    #[test]
    fn test_extract_type_payconiq() {
        assert_eq!(
            extract_transaction_type_code("VIREMENT PAYCONIQ VERS BE79"),
            "PAYCONIQ"
        );
    }

    #[test]
    fn test_extract_type_mastercard_statement() {
        assert_eq!(
            extract_transaction_type_code("MASTERCARD RELEVE NUMERO 207"),
            "MASTERCARD_STATEMENT"
        );
    }

    #[test]
    fn test_extract_type_bancontact_app() {
        assert_eq!(
            extract_transaction_type_code("BANCONTACT APP OU MOBILE BANKING APP RECU"),
            "BANCONTACT_APP"
        );
    }

    #[test]
    fn test_extract_type_bancontact_atm() {
        assert_eq!(
            extract_transaction_type_code("BANCONTACT RETRAIT D'ESPECES - ARGENTAA2010025"),
            "BANCONTACT_ATM"
        );
    }

    #[test]
    fn test_detect_belfius_format() {
        let content = b"Date de comptabilisation \xe0 partir de;01/08/2024\r\n";
        assert!(BelfiusParser::detect(content));
    }

    #[test]
    fn test_detect_non_belfius() {
        let content = b"Date,Amount,Description\r\n";
        assert!(!BelfiusParser::detect(content));
    }

    #[test]
    #[ignore] // requires .samples/ directory (not available in CI)
    fn test_parse_full_sample_file() {
        let content = std::fs::read(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../.samples/BE34 0634 5590 5590 2025-08-08 9-59-57 1.csv"
        ))
        .expect("Sample CSV should exist");

        let transactions = BelfiusParser::parse(&content).expect("Parsing should succeed");

        assert!(
            transactions.len() >= 250,
            "Expected ~255 transactions, got {}",
            transactions.len()
        );

        // Check first transaction (last line in file = earliest by reverse order)
        let last = transactions.last().unwrap();
        assert_eq!(last.account_iban, "BE34 0634 5590 5590");

        // Check first record
        let first = &transactions[0];
        assert_eq!(first.account_iban, "BE34 0634 5590 5590");
        assert_eq!(first.amount_cents, -5000);
        assert_eq!(first.transaction_type_code, "ATM_WITHDRAWAL");
    }

    #[test]
    #[ignore] // requires .samples/ directory (not available in CI)
    fn test_parse_row_with_counterparty() {
        let content = std::fs::read(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../.samples/BE34 0634 5590 5590 2025-08-08 9-59-57 1.csv"
        ))
        .unwrap();

        let transactions = BelfiusParser::parse(&content).unwrap();

        // Find a transaction with a counterparty (e.g., ENSPIRIT salary)
        let salary = transactions
            .iter()
            .find(|t| t.description.contains("ENSPIRIT") && t.description.contains("Salaire"))
            .expect("Should find ENSPIRIT salary transaction");

        assert_eq!(
            salary.counterparty_iban,
            Some("BE29 0689 0044 5064".to_string())
        );
        assert_eq!(salary.counterparty_name, Some("ENSPIRIT".to_string()));
        assert!(salary.amount_cents > 0, "Salary should be positive");
    }
}
