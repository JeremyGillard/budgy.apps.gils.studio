import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import type { Transaction, PaginatedResponse } from "../types";

export function useTransactions() {
  const transactions = ref<Transaction[]>([]);
  const total = ref(0);
  const totalPages = ref(0);
  const page = ref(1);
  const perPage = ref(25);
  const loading = ref(false);

  async function fetchTransactions(p?: number, pp?: number) {
    loading.value = true;
    try {
      const result = await invoke<PaginatedResponse<Transaction>>(
        "list_transactions",
        {
          page: p ?? page.value,
          perPage: pp ?? perPage.value,
        }
      );
      transactions.value = result.data;
      total.value = result.total;
      totalPages.value = result.total_pages;
      page.value = result.page;
      perPage.value = result.per_page;
    } finally {
      loading.value = false;
    }
  }

  return {
    transactions,
    total,
    totalPages,
    page,
    perPage,
    loading,
    fetchTransactions,
  };
}
