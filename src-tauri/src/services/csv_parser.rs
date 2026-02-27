use sha2::{Digest, Sha256};

#[derive(Debug, Clone)]
pub struct CsvMetadata {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct ParsedTransaction {
    pub account_iban: String,
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
}

#[derive(Debug)]
pub struct ParsedCsvFile {
    pub metadata: Vec<CsvMetadata>,
    pub transactions: Vec<ParsedTransaction>,
    pub account_iban: String,
}

pub fn parse_csv(content: &[u8]) -> Result<ParsedCsvFile, String> {
    let text = decode_content(content);
    let lines: Vec<&str> = text.lines().collect();

    if lines.len() < 14 {
        return Err("CSV file too short".to_string());
    }

    let metadata = parse_metadata_lines(&lines[0..11]);

    // Line 11 (index 11) is separator ";"
    // Line 12 (index 12) is header row
    // Data starts at line 13 (index 13)
    let mut transactions = Vec::new();
    let mut account_iban = String::new();

    for line in &lines[13..] {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed == ";" {
            continue;
        }

        let fields: Vec<&str> = trimmed.split(';').collect();
        if fields.len() < 15 {
            continue;
        }

        let iban = fields[0].trim().to_string();
        if account_iban.is_empty() && !iban.is_empty() {
            account_iban = iban.clone();
        }

        let row_hash = compute_row_hash(trimmed);

        let transaction = ParsedTransaction {
            account_iban: iban,
            accounting_date: parse_date(fields[1].trim()),
            statement_number: fields[2].trim().to_string(),
            sequence_number: fields[3].trim().parse::<i32>().unwrap_or(0),
            counterparty_account: non_empty(fields[4].trim()),
            counterparty_name: non_empty(fields[5].trim()),
            counterparty_street: non_empty(fields[6].trim()),
            counterparty_city: non_empty(fields[7].trim()),
            transaction_description: fields[8].trim().to_string(),
            value_date: parse_date(fields[9].trim()),
            amount_cents: parse_amount_cents(fields[10].trim()),
            currency: fields[11].trim().to_string(),
            bic: non_empty(fields[12].trim()),
            country_code: non_empty(fields[13].trim()),
            communication: non_empty(fields[14].trim()),
            row_hash,
        };

        transactions.push(transaction);
    }

    Ok(ParsedCsvFile {
        metadata,
        transactions,
        account_iban,
    })
}

pub fn parse_metadata_lines(lines: &[&str]) -> Vec<CsvMetadata> {
    lines
        .iter()
        .filter_map(|line| {
            let parts: Vec<&str> = line.splitn(2, ';').collect();
            if parts.len() == 2 {
                let key = parts[0].trim().to_string();
                let value = parts[1].trim().to_string();
                if !key.is_empty() && !value.is_empty() {
                    return Some(CsvMetadata { key, value });
                }
            }
            None
        })
        .collect()
}

pub fn parse_date(date_str: &str) -> String {
    if date_str.is_empty() {
        return String::new();
    }
    let parts: Vec<&str> = date_str.split('/').collect();
    if parts.len() == 3 {
        format!("{}-{}-{}", parts[2], parts[1], parts[0])
    } else {
        date_str.to_string()
    }
}

pub fn parse_amount_cents(amount_str: &str) -> i32 {
    if amount_str.is_empty() {
        return 0;
    }
    // Belgian format: -1.234,56 -> strip dots, replace comma with dot
    let cleaned = amount_str.replace('.', "").replace(',', ".");
    let value: f64 = cleaned.parse().unwrap_or(0.0);
    (value * 100.0).round() as i32
}

pub fn compute_sha256(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}

pub fn match_transaction_type(description: &str) -> Option<&'static str> {
    let desc = description.trim();
    let matchers: &[(&str, &str)] = &[
        ("BANCONTACT ACHAT", "bancontact_purchase"),
        ("BANCONTACT RETRAIT D'ESPECES", "bancontact_cash_withdrawal"),
        ("BANCONTACT APP OU MOBILE BANKING APP RECU", "bancontact_p2p_received"),
        ("PAIEMENT DEBITMASTERCARD", "mastercard_payment"),
        ("MASTERCARD RELEVE", "mastercard_statement"),
        ("CHARGEMENT DE LA CARTE MASTERCARD PREPAID", "mastercard_prepaid_load"),
        ("DECHARGEMENT DE LA CARTE MASTERCARD PREPAID", "mastercard_prepaid_unload"),
        ("VIREMENT BELFIUS MOBILE", "mobile_transfer"),
        ("VIREMENT PAYCONIQ", "payconiq_transfer"),
        ("VOTRE DOMICILIATION EUROPEENNE", "sepa_direct_debit"),
        ("ORDRE PERMANENT", "standing_order"),
        ("VERSEMENT DE", "incoming_transfer"),
        ("VERSEMENT DU", "incoming_transfer"),
        ("VERSEMENT INSTANTANE", "instant_transfer_received"),
        ("ARGENT RECU VIA VOTRE APP MOBILE BANKING", "mobile_received"),
        ("RETRAIT D'ESPECES AVEC CARTE", "cash_withdrawal"),
        ("DEPOT ESPECES AVEC CARTE", "cash_deposit"),
        ("PARTICIPATION AUX FRAIS", "account_fees"),
        ("FRAIS SUR ORDRE(S) NON-EXECUTE(S)", "failed_order_fees"),
    ];

    for (prefix, code) in matchers {
        if desc.starts_with(prefix) {
            return Some(code);
        }
    }
    None
}

fn decode_content(content: &[u8]) -> String {
    // Try UTF-8 first
    if let Ok(text) = std::str::from_utf8(content) {
        return text.replace('\r', "");
    }
    // Fall back to Windows-1252
    let (cow, _, _) = encoding_rs::WINDOWS_1252.decode(content);
    cow.replace('\r', "")
}

fn non_empty(s: &str) -> Option<String> {
    if s.is_empty() {
        None
    } else {
        Some(s.to_string())
    }
}

fn compute_row_hash(row: &str) -> String {
    compute_sha256(row.as_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_date_valid() {
        assert_eq!(parse_date("31/12/2024"), "2024-12-31");
        assert_eq!(parse_date("01/08/2024"), "2024-08-01");
    }

    #[test]
    fn test_parse_date_empty() {
        assert_eq!(parse_date(""), "");
    }

    #[test]
    fn test_parse_amount_cents_positive() {
        assert_eq!(parse_amount_cents("1.422,72"), 142272);
    }

    #[test]
    fn test_parse_amount_cents_negative() {
        assert_eq!(parse_amount_cents("-1.000,00"), -100000);
        assert_eq!(parse_amount_cents("-50,00"), -5000);
    }

    #[test]
    fn test_parse_amount_cents_small() {
        assert_eq!(parse_amount_cents("-8,90"), -890);
        assert_eq!(parse_amount_cents("933,68"), 93368);
    }

    #[test]
    fn test_parse_amount_cents_empty() {
        assert_eq!(parse_amount_cents(""), 0);
    }

    #[test]
    fn test_compute_sha256() {
        let hash = compute_sha256(b"hello");
        assert_eq!(
            hash,
            "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
        );
    }

    #[test]
    fn test_match_transaction_type_bancontact() {
        assert_eq!(
            match_transaction_type("BANCONTACT ACHAT - COLRUYT"),
            Some("bancontact_purchase")
        );
    }

    #[test]
    fn test_match_transaction_type_mastercard() {
        assert_eq!(
            match_transaction_type("PAIEMENT DEBITMASTERCARD 28/12 LSP*Oakberry"),
            Some("mastercard_payment")
        );
    }

    #[test]
    fn test_match_transaction_type_transfer() {
        assert_eq!(
            match_transaction_type("VIREMENT BELFIUS MOBILE VERS BE51"),
            Some("mobile_transfer")
        );
    }

    #[test]
    fn test_match_transaction_type_incoming() {
        assert_eq!(
            match_transaction_type("VERSEMENT DU BE29 0689 0044 5064 ENSPIRIT"),
            Some("incoming_transfer")
        );
        assert_eq!(
            match_transaction_type("VERSEMENT DE BE30 6792 0020 9111"),
            Some("incoming_transfer")
        );
    }

    #[test]
    fn test_match_transaction_type_sepa() {
        assert_eq!(
            match_transaction_type("VOTRE DOMICILIATION EUROPEENNE B014304999"),
            Some("sepa_direct_debit")
        );
    }

    #[test]
    fn test_match_transaction_type_standing_order() {
        assert_eq!(
            match_transaction_type("ORDRE PERMANENT 18895241"),
            Some("standing_order")
        );
    }

    #[test]
    fn test_match_transaction_type_cash() {
        assert_eq!(
            match_transaction_type("RETRAIT D'ESPECES AVEC CARTE N°"),
            Some("cash_withdrawal")
        );
        assert_eq!(
            match_transaction_type("DEPOT ESPECES AVEC CARTE N°"),
            Some("cash_deposit")
        );
    }

    #[test]
    fn test_match_transaction_type_unknown() {
        assert_eq!(match_transaction_type("SOMETHING UNKNOWN"), None);
    }

    #[test]
    fn test_parse_metadata_lines() {
        let lines = vec![
            "Date de comptabilisation à partir de;01/08/2024",
            "Date de comptabilisation jusqu'au;31/12/2024",
            "Montant à partir de;",
        ];
        let metadata = parse_metadata_lines(&lines);
        assert_eq!(metadata.len(), 2);
        assert_eq!(metadata[0].key, "Date de comptabilisation à partir de");
        assert_eq!(metadata[0].value, "01/08/2024");
    }

    #[test]
    fn test_decode_utf8_content() {
        let text = "hello\r\nworld\r\n";
        let result = decode_content(text.as_bytes());
        assert_eq!(result, "hello\nworld\n");
    }
}
