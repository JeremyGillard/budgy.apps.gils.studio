// @generated automatically by Diesel CLI.

diesel::table! {
    accounts (id) {
        id -> Integer,
        iban -> Text,
        label -> Nullable<Text>,
        currency -> Text,
        created_at -> Text,
        updated_at -> Text,
    }
}

diesel::table! {
    categories (id) {
        id -> Integer,
        name -> Text,
        parent_id -> Nullable<Integer>,
        icon -> Nullable<Text>,
        color -> Nullable<Text>,
    }
}

diesel::table! {
    category_rules (id) {
        id -> Integer,
        category_id -> Integer,
        match_field -> Text,
        match_pattern -> Text,
        priority -> Integer,
    }
}

diesel::table! {
    counterparties (id) {
        id -> Integer,
        iban -> Nullable<Text>,
        name -> Text,
        street -> Nullable<Text>,
        postal_code_city -> Nullable<Text>,
        bic -> Nullable<Text>,
        country_code -> Nullable<Text>,
        created_at -> Text,
        updated_at -> Text,
    }
}

diesel::table! {
    imports (id) {
        id -> Integer,
        filename -> Text,
        account_id -> Integer,
        imported_at -> Text,
        record_count -> Integer,
        date_from -> Nullable<Text>,
        date_to -> Nullable<Text>,
    }
}

diesel::table! {
    transaction_types (id) {
        id -> Integer,
        code -> Text,
        label -> Text,
    }
}

diesel::table! {
    transactions (id) {
        id -> Integer,
        account_id -> Integer,
        counterparty_id -> Nullable<Integer>,
        category_id -> Nullable<Integer>,
        transaction_type_id -> Integer,
        accounting_date -> Text,
        value_date -> Text,
        statement_number -> Nullable<Text>,
        transaction_number -> Nullable<Text>,
        amount_cents -> Integer,
        currency -> Text,
        description -> Text,
        communication -> Nullable<Text>,
        import_hash -> Text,
        created_at -> Text,
        updated_at -> Text,
    }
}

diesel::joinable!(category_rules -> categories (category_id));
diesel::joinable!(imports -> accounts (account_id));
diesel::joinable!(transactions -> accounts (account_id));
diesel::joinable!(transactions -> categories (category_id));
diesel::joinable!(transactions -> counterparties (counterparty_id));
diesel::joinable!(transactions -> transaction_types (transaction_type_id));

diesel::allow_tables_to_appear_in_same_query!(
    accounts,
    categories,
    category_rules,
    counterparties,
    imports,
    transaction_types,
    transactions,
);
