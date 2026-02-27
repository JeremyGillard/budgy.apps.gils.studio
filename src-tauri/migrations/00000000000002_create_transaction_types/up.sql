CREATE TABLE transaction_types (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    code TEXT NOT NULL UNIQUE,
    label_fr TEXT NOT NULL,
    label_en TEXT NOT NULL,
    direction TEXT NOT NULL CHECK (direction IN ('credit', 'debit'))
);

-- Seed 18 Belfius transaction types
INSERT INTO transaction_types (code, label_fr, label_en, direction) VALUES
    ('bancontact_purchase', 'BANCONTACT ACHAT', 'Bancontact purchase', 'debit'),
    ('bancontact_cash_withdrawal', 'BANCONTACT RETRAIT D''ESPECES', 'Bancontact cash withdrawal', 'debit'),
    ('bancontact_p2p_received', 'BANCONTACT APP OU MOBILE BANKING APP RECU', 'Bancontact P2P received', 'credit'),
    ('mastercard_payment', 'PAIEMENT DEBITMASTERCARD', 'Mastercard payment', 'debit'),
    ('mastercard_statement', 'MASTERCARD RELEVE', 'Mastercard statement', 'debit'),
    ('mastercard_prepaid_load', 'CHARGEMENT DE LA CARTE MASTERCARD PREPAID', 'Mastercard prepaid load', 'debit'),
    ('mastercard_prepaid_unload', 'DECHARGEMENT DE LA CARTE MASTERCARD PREPAID', 'Mastercard prepaid unload', 'credit'),
    ('mobile_transfer', 'VIREMENT BELFIUS MOBILE', 'Mobile transfer', 'debit'),
    ('payconiq_transfer', 'VIREMENT PAYCONIQ', 'Payconiq transfer', 'debit'),
    ('sepa_direct_debit', 'VOTRE DOMICILIATION EUROPEENNE', 'SEPA direct debit', 'debit'),
    ('standing_order', 'ORDRE PERMANENT', 'Standing order', 'debit'),
    ('incoming_transfer', 'VERSEMENT DE/DU', 'Incoming transfer', 'credit'),
    ('instant_transfer_received', 'VERSEMENT INSTANTANE', 'Instant transfer received', 'credit'),
    ('mobile_received', 'ARGENT RECU VIA VOTRE APP MOBILE BANKING', 'Mobile received', 'credit'),
    ('cash_withdrawal', 'RETRAIT D''ESPECES AVEC CARTE', 'Cash withdrawal', 'debit'),
    ('cash_deposit', 'DEPOT ESPECES AVEC CARTE', 'Cash deposit', 'credit'),
    ('account_fees', 'PARTICIPATION AUX FRAIS', 'Account fees', 'debit'),
    ('failed_order_fees', 'FRAIS SUR ORDRE(S) NON-EXECUTE(S)', 'Failed order fees', 'debit');
