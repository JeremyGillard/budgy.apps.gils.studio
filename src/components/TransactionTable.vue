<script setup lang="ts">
import { onMounted, watch } from "vue";
import DataTable, { type DataTablePageEvent } from "primevue/datatable";
import Column from "primevue/column";
import { useTransactions } from "../composables/useTransactions";

const { transactions, total, perPage, loading, fetchTransactions } =
  useTransactions();

function formatAmount(cents: number): string {
  const value = cents / 100;
  return new Intl.NumberFormat("fr-BE", {
    style: "currency",
    currency: "EUR",
  }).format(value);
}

function amountClass(cents: number): string {
  return cents >= 0 ? "amount-positive" : "amount-negative";
}

function onPage(event: DataTablePageEvent) {
  const newPage = (event.first ?? 0) / (event.rows ?? 25) + 1;
  fetchTransactions(newPage, event.rows);
}

onMounted(() => {
  fetchTransactions();
});

watch(perPage, () => {
  fetchTransactions(1);
});
</script>

<template>
  <DataTable
    :value="transactions"
    :lazy="true"
    :paginator="true"
    :rows="perPage"
    :totalRecords="total"
    :rowsPerPageOptions="[10, 25, 50]"
    :loading="loading"
    @page="onPage"
    stripedRows
    size="small"
  >
    <Column field="accounting_date" header="Date" style="width: 100px" />
    <Column field="counterparty_name" header="Counterparty" style="width: 200px">
      <template #body="{ data }">
        {{ data.counterparty_name || "—" }}
      </template>
    </Column>
    <Column field="transaction_description" header="Description" style="min-width: 300px">
      <template #body="{ data }">
        <span :title="data.transaction_description">
          {{ data.transaction_description.substring(0, 80) }}
          <span v-if="data.transaction_description.length > 80">…</span>
        </span>
      </template>
    </Column>
    <Column field="amount_cents" header="Amount" style="width: 120px; text-align: right">
      <template #body="{ data }">
        <span :class="amountClass(data.amount_cents)">
          {{ formatAmount(data.amount_cents) }}
        </span>
      </template>
    </Column>
  </DataTable>
</template>

<style scoped>
.amount-positive {
  color: #22c55e;
  font-weight: 600;
}

.amount-negative {
  color: #ef4444;
  font-weight: 600;
}
</style>
