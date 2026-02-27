export interface Transaction {
  id: number;
  import_id: number;
  account_id: number;
  transaction_type_id: number | null;
  accounting_date: string;
  statement_number: string;
  sequence_number: number;
  counterparty_account: string | null;
  counterparty_name: string | null;
  counterparty_street: string | null;
  counterparty_city: string | null;
  transaction_description: string;
  value_date: string;
  amount_cents: number;
  currency: string;
  bic: string | null;
  country_code: string | null;
  communication: string | null;
  row_hash: string;
  created_at: string;
  updated_at: string;
}

export interface PaginatedResponse<T> {
  data: T[];
  total: number;
  page: number;
  per_page: number;
  total_pages: number;
}

export interface ImportResult {
  file_name: string;
  imported_count: number;
  skipped_count: number;
  error: string | null;
}
