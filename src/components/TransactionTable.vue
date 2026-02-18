<script setup>
import { ref, watch } from "vue";
import { invoke } from "@tauri-apps/api/core";
import DataTable from "primevue/datatable";
import Column from "primevue/column";

const props = defineProps({
  year: { type: Number, required: true },
  month: { type: Number, required: true },
  refreshKey: { type: Number, default: 0 },
});

const transactions = ref([]);
const loading = ref(false);

function formatAmount(cents) {
  const val = (cents / 100).toFixed(2);
  return val.replace(/\B(?=(\d{3})+(?!\d))/g, ",");
}

async function loadTransactions() {
  loading.value = true;
  try {
    const txs = await invoke("list_transactions_by_month", {
      year: props.year,
      month: props.month,
    });
    transactions.value = txs;
  } catch (e) {
    console.error("Failed to load transactions:", e);
  } finally {
    loading.value = false;
  }
}

watch(
  () => [props.year, props.month, props.refreshKey],
  () => loadTransactions(),
  { immediate: true },
);
</script>

<template>
  <section>
    <h2>
      Transactions
      <span class="tx-count">({{ transactions.length }})</span>
    </h2>
    <DataTable
      :value="transactions"
      :loading="loading"
      stripedRows
      sortField="accounting_date"
      :sortOrder="-1"
      tableStyle="min-width: 50rem"
    >
      <Column field="accounting_date" header="Date" sortable style="width: 10%" />
      <Column field="description" header="Description" sortable style="width: 35%">
        <template #body="{ data }">
          <span class="description-cell" :title="data.description">
            {{ data.description }}
          </span>
        </template>
      </Column>
      <Column field="counterparty_name" header="Counterparty" sortable style="width: 20%" />
      <Column field="communication" header="Communication" style="width: 20%" />
      <Column field="amount_cents" header="Amount" sortable style="width: 15%">
        <template #body="{ data }">
          <span :class="['amount', data.amount_cents >= 0 ? 'positive' : 'negative']">
            {{ formatAmount(data.amount_cents) }} {{ data.currency }}
          </span>
        </template>
      </Column>
    </DataTable>
  </section>
</template>

<style scoped>
h2 {
  font-size: 1.1rem;
  margin: 0 0 0.75rem 0;
}

.tx-count {
  font-weight: 400;
  color: var(--p-text-muted-color);
  font-size: 0.85rem;
}

.description-cell {
  display: block;
  max-width: 350px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.amount {
  font-variant-numeric: tabular-nums;
  white-space: nowrap;
  text-align: right;
  display: block;
}

.amount.positive {
  color: var(--p-green-500);
}

.amount.negative {
  color: var(--p-red-500);
}
</style>
