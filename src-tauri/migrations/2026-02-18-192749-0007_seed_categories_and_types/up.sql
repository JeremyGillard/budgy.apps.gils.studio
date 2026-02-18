-- Seed transaction types
INSERT INTO transaction_types (code, label) VALUES ('BANCONTACT_PURCHASE', 'Bancontact Purchase');
INSERT INTO transaction_types (code, label) VALUES ('BANCONTACT_APP', 'Bancontact App Payment');
INSERT INTO transaction_types (code, label) VALUES ('BANCONTACT_ATM', 'Bancontact ATM Withdrawal');
INSERT INTO transaction_types (code, label) VALUES ('ATM_WITHDRAWAL', 'ATM Withdrawal');
INSERT INTO transaction_types (code, label) VALUES ('DEBIT_MASTERCARD', 'Debit Mastercard Payment');
INSERT INTO transaction_types (code, label) VALUES ('MASTERCARD_STATEMENT', 'Mastercard Statement');
INSERT INTO transaction_types (code, label) VALUES ('TRANSFER_OUT', 'Outgoing Transfer');
INSERT INTO transaction_types (code, label) VALUES ('TRANSFER_IN', 'Incoming Transfer');
INSERT INTO transaction_types (code, label) VALUES ('INSTANT_TRANSFER_IN', 'Instant Incoming Transfer');
INSERT INTO transaction_types (code, label) VALUES ('STANDING_ORDER', 'Standing Order');
INSERT INTO transaction_types (code, label) VALUES ('DIRECT_DEBIT', 'Direct Debit');
INSERT INTO transaction_types (code, label) VALUES ('CARD_LOAD', 'Prepaid Card Loading');
INSERT INTO transaction_types (code, label) VALUES ('CARD_UNLOAD', 'Prepaid Card Unloading');
INSERT INTO transaction_types (code, label) VALUES ('CASH_DEPOSIT', 'Cash Deposit');
INSERT INTO transaction_types (code, label) VALUES ('FEES', 'Bank Fees');
INSERT INTO transaction_types (code, label) VALUES ('MOBILE_RECEIVE', 'Mobile App Receive');
INSERT INTO transaction_types (code, label) VALUES ('PAYCONIQ', 'Payconiq Transfer');
INSERT INTO transaction_types (code, label) VALUES ('OTHER', 'Other');

-- Seed top-level categories
INSERT INTO categories (name, icon, color) VALUES ('Uncategorized', NULL, '#9E9E9E');
INSERT INTO categories (name, icon, color) VALUES ('Food & Groceries', NULL, '#4CAF50');
INSERT INTO categories (name, icon, color) VALUES ('Transport', NULL, '#2196F3');
INSERT INTO categories (name, icon, color) VALUES ('Housing', NULL, '#FF9800');
INSERT INTO categories (name, icon, color) VALUES ('Health', NULL, '#F44336');
INSERT INTO categories (name, icon, color) VALUES ('Entertainment', NULL, '#9C27B0');
INSERT INTO categories (name, icon, color) VALUES ('Shopping', NULL, '#E91E63');
INSERT INTO categories (name, icon, color) VALUES ('Income', NULL, '#8BC34A');
INSERT INTO categories (name, icon, color) VALUES ('Transfers', NULL, '#607D8B');
INSERT INTO categories (name, icon, color) VALUES ('Fees & Charges', NULL, '#795548');
INSERT INTO categories (name, icon, color) VALUES ('Cash', NULL, '#FF5722');
