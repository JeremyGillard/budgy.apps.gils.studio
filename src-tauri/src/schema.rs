// @generated automatically by Diesel CLI.

diesel::table! {
    accounts (id) {
        id -> Integer,
        iban -> Text,
        name -> Nullable<Text>,
        currency -> Text,
        created_at -> Text,
        updated_at -> Text,
    }
}

diesel::table! {
    import_metadata (id) {
        id -> Integer,
        import_id -> Integer,
        key -> Text,
        value -> Text,
    }
}

diesel::table! {
    imports (id) {
        id -> Integer,
        account_id -> Integer,
        file_name -> Text,
        file_hash -> Text,
        record_count -> Integer,
        date_range_from -> Nullable<Text>,
        date_range_to -> Nullable<Text>,
        imported_at -> Text,
    }
}

diesel::table! {
    transaction_types (id) {
        id -> Integer,
        code -> Text,
        label_fr -> Text,
        label_en -> Text,
        direction -> Text,
    }
}

diesel::table! {
    transactions (id) {
        id -> Integer,
        import_id -> Integer,
        account_id -> Integer,
        transaction_type_id -> Nullable<Integer>,
        accounting_date -> Text,
        statement_number -> Text,
        sequence_number -> Integer,
        counterparty_account -> Nullable<Text>,
        counterparty_name -> Nullable<Text>,
        counterparty_street -> Nullable<Text>,
        counterparty_city -> Nullable<Text>,
        transaction_description -> Text,
        value_date -> Text,
        amount_cents -> Integer,
        currency -> Text,
        bic -> Nullable<Text>,
        country_code -> Nullable<Text>,
        communication -> Nullable<Text>,
        row_hash -> Text,
        created_at -> Text,
        updated_at -> Text,
    }
}

diesel::joinable!(import_metadata -> imports (import_id));
diesel::joinable!(imports -> accounts (account_id));
diesel::joinable!(transactions -> accounts (account_id));
diesel::joinable!(transactions -> imports (import_id));
diesel::joinable!(transactions -> transaction_types (transaction_type_id));

diesel::allow_tables_to_appear_in_same_query!(
    accounts,
    import_metadata,
    imports,
    transaction_types,
    transactions,
);
