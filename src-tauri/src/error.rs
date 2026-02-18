use thiserror::Error;

#[derive(Error, Debug)]
pub enum BudgyError {
    #[error("Database error: {0}")]
    Diesel(#[from] diesel::result::Error),

    #[error("Migration error: {0}")]
    Migration(String),

    #[error("CSV parsing error: {0}")]
    CsvParse(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("{0}")]
    General(String),
}

impl serde::Serialize for BudgyError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
