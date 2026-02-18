<script setup>
import { ref, computed, watch } from "vue";
import { invoke } from "@tauri-apps/api/core";
import DataTable from "primevue/datatable";
import Column from "primevue/column";
import Select from "primevue/select";
import Button from "primevue/button";
import DatePicker from "primevue/datepicker";
import InputText from "primevue/inputtext";
import IconField from "primevue/iconfield";
import InputIcon from "primevue/inputicon";
import Tag from "primevue/tag";
import { useToast } from "primevue/usetoast";

const toast = useToast();

const props = defineProps({
  year: { type: Number, required: true },
  month: { type: Number, required: true },
  refreshKey: { type: Number, default: 0 },
});

const emit = defineEmits(["prev", "next", "navigate"]);

const selectedDate = computed({
  get: () => new Date(props.year, props.month - 1, 1),
  set: (val) => {
    if (val) {
      emit("navigate", { year: val.getFullYear(), month: val.getMonth() + 1 });
    }
  },
});

const transactions = ref([]);
const categories = ref([]);
const searchQuery = ref("");
const searchInputRef = ref(null);
const selectedRows = ref([]);
const bulkCategoryId = ref(null);
const loading = ref(false);
const globalStats = ref(null);
const suggestions = ref({});

const categoryMap = computed(() => {
  const map = {};
  for (const c of categories.value) {
    map[c.id] = c;
  }
  return map;
});

const categoryOptions = computed(() =>
  categories.value.map((c) => ({ label: c.name, value: c.id, color: c.color }))
);

const sortedTransactions = computed(() => {
  let txs = [...transactions.value];
  const q = searchQuery.value.trim().toLowerCase();
  if (q) {
    txs = txs.filter((t) => t.description?.toLowerCase().includes(q));
  }
  txs.sort((a, b) => {
    // Uncategorized first
    const aUncat = a.category_id == null ? 0 : 1;
    const bUncat = b.category_id == null ? 0 : 1;
    if (aUncat !== bUncat) return aUncat - bUncat;
    // Then by date descending
    return b.accounting_date.localeCompare(a.accounting_date);
  });
  return txs;
});

const uncategorizedCount = computed(
  () => transactions.value.filter((t) => t.category_id == null).length
);

function focusSearch() {
  searchInputRef.value?.$el?.focus();
}

function formatAmount(cents) {
  const val = (cents / 100).toFixed(2);
  return val.replace(/\B(?=(\d{3})+(?!\d))/g, ",");
}

function rowClass(data) {
  return data.category_id == null ? "row-uncategorized" : "";
}

async function loadData() {
  loading.value = true;
  try {
    const [txs, cats, stats, sugs] = await Promise.all([
      invoke("list_transactions_by_month", { year: props.year, month: props.month }),
      invoke("list_categories"),
      invoke("get_categorization_stats"),
      invoke("get_category_suggestions", { year: props.year, month: props.month }),
    ]);
    transactions.value = txs;
    categories.value = cats;
    globalStats.value = stats;
    const map = {};
    for (const s of sugs) {
      map[s.transaction_id] = s.suggested_category_id;
    }
    suggestions.value = map;
  } catch (e) {
    console.error("Failed to load data:", e);
  } finally {
    loading.value = false;
  }
}

async function onCategoryChange(transaction, categoryId) {
  try {
    const wasUncategorized = transaction.category_id == null;
    await invoke("categorize_transaction", {
      transactionId: transaction.id,
      categoryId,
    });
    transaction.category_id = categoryId;
    if (wasUncategorized && globalStats.value) {
      globalStats.value.uncategorized--;
    }
  } catch (e) {
    toast.add({ severity: "error", summary: "Failed to categorize", detail: String(e), life: 5000 });
  }
}

async function confirmSuggestion(transaction) {
  const categoryId = suggestions.value[transaction.id];
  if (categoryId != null) {
    await onCategoryChange(transaction, categoryId);
  }
}

async function bulkApply() {
  if (!selectedRows.value.length || bulkCategoryId.value == null) return;
  const ids = selectedRows.value.map((r) => r.id);
  try {
    const count = await invoke("bulk_categorize_transactions", {
      transactionIds: ids,
      categoryId: bulkCategoryId.value,
    });
    // Patch local state
    let newlyCategorized = 0;
    for (const tx of transactions.value) {
      if (ids.includes(tx.id)) {
        if (tx.category_id == null) newlyCategorized++;
        tx.category_id = bulkCategoryId.value;
      }
    }
    if (globalStats.value) {
      globalStats.value.uncategorized -= newlyCategorized;
    }
    const catName = categoryMap.value[bulkCategoryId.value]?.name ?? "Unknown";
    toast.add({
      severity: "success",
      summary: "Bulk categorize",
      detail: `Assigned "${catName}" to ${count} transaction(s)`,
      life: 4000,
    });
    selectedRows.value = [];
    bulkCategoryId.value = null;
  } catch (e) {
    toast.add({ severity: "error", summary: "Bulk categorize failed", detail: String(e), life: 5000 });
  }
}

watch(
  () => [props.year, props.month, props.refreshKey],
  () => {
    searchQuery.value = "";
    loadData();
  },
  { immediate: true },
);
</script>

<template>
  <section>
    <div class="categorize-header">
      <h2>Categorize Transactions</h2>
      <Tag
        v-if="uncategorizedCount > 0"
        :value="`${uncategorizedCount} uncategorized`"
        severity="warn"
      />
      <Tag v-else value="All categorized" severity="success" />
    </div>

    <div v-if="globalStats" class="summary-cards">
      <div class="card total">
        <span class="card-label">Total Transactions</span>
        <span class="card-value">{{ globalStats.total }}</span>
      </div>
      <div class="card uncategorized">
        <span class="card-label">Uncategorized</span>
        <span class="card-value">{{ globalStats.uncategorized }}</span>
      </div>
    </div>

    <div class="month-nav">
      <Button
        icon="pi pi-chevron-left"
        text
        rounded
        aria-label="Previous month"
        @click="emit('prev')"
      />
      <DatePicker
        v-model="selectedDate"
        view="month"
        dateFormat="MM yy"
        showIcon
        iconDisplay="input"
        class="month-picker"
      />
      <Button
        icon="pi pi-chevron-right"
        text
        rounded
        aria-label="Next month"
        @click="emit('next')"
      />
      <IconField class="search-field">
        <InputIcon class="pi pi-search" @click="focusSearch" style="cursor: pointer" />
        <InputText ref="searchInputRef" v-model="searchQuery" placeholder="Search by description..." />
      </IconField>
    </div>

    <div v-if="selectedRows.length" class="bulk-toolbar">
      <span class="bulk-count">{{ selectedRows.length }} selected</span>
      <Select
        v-model="bulkCategoryId"
        :options="categoryOptions"
        optionLabel="label"
        optionValue="value"
        placeholder="Choose category"
        class="bulk-select"
      />
      <Button
        label="Apply"
        icon="pi pi-check"
        :disabled="bulkCategoryId == null"
        @click="bulkApply"
        size="small"
      />
    </div>

    <DataTable
      v-model:selection="selectedRows"
      :value="sortedTransactions"
      :loading="loading"
      :rowClass="rowClass"
      dataKey="id"
      tableStyle="min-width: 60rem"
    >
      <Column selectionMode="multiple" headerStyle="width: 3rem" />
      <Column header="" style="width: 2.5rem">
        <template #body="{ data }">
          <i
            v-if="data.category_id == null"
            class="pi pi-exclamation-circle"
            style="color: var(--p-orange-500); font-size: 1.1rem"
          />
          <i
            v-else
            class="pi pi-check-circle"
            :style="{ color: categoryMap[data.category_id]?.color || 'var(--p-green-500)', fontSize: '1.1rem' }"
          />
        </template>
      </Column>
      <Column field="accounting_date" header="Date" style="width: 9%" />
      <Column field="description" header="Description" style="width: 28%">
        <template #body="{ data }">
          <span class="description-cell" :title="data.description">
            {{ data.description }}
          </span>
        </template>
      </Column>
      <Column field="counterparty_name" header="Counterparty" style="width: 15%" />
      <Column header="Category" style="width: 20%">
        <template #body="{ data }">
          <div class="category-cell">
            <Button
              v-if="data.category_id == null && suggestions[data.id] != null"
              icon="pi pi-check"
              outlined size="small" severity="success"
              aria-label="Accept suggestion"
              class="confirm-btn"
              @click="confirmSuggestion(data)"
            />
            <Select
              :modelValue="data.category_id ?? suggestions[data.id] ?? null"
              @update:modelValue="onCategoryChange(data, $event)"
              :options="categoryOptions"
              optionLabel="label"
              optionValue="value"
              placeholder="Select category"
              class="category-select"
              :class="{ 'suggestion-active': data.category_id == null && suggestions[data.id] != null }"
            />
          </div>
        </template>
      </Column>
      <Column field="amount_cents" header="Amount" style="width: 12%">
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
.categorize-header {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  margin-bottom: 0.75rem;
}

.categorize-header h2 {
  font-size: 1.1rem;
  margin: 0;
}

.month-nav {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  margin-bottom: 1rem;
}

.month-picker {
  width: 12rem;
}

.search-field {
  flex: 1;
}

.summary-cards {
  display: flex;
  gap: 1rem;
  margin-bottom: 1rem;
}

.card {
  flex: 1;
  padding: 1rem;
  border-radius: 6px;
  border: 1px solid var(--p-content-border-color);
  background: var(--p-content-background);
}

.card-label {
  display: block;
  font-size: 0.75rem;
  color: var(--p-text-muted-color);
  text-transform: uppercase;
  margin-bottom: 0.25rem;
}

.card-value {
  display: block;
  font-size: 1.3rem;
  font-weight: 700;
}

.card.total .card-value {
  color: var(--p-blue-500);
}

.card.uncategorized .card-value {
  color: var(--p-orange-500);
}

.bulk-toolbar {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  padding: 0.5rem 0.75rem;
  margin-bottom: 0.75rem;
  background: var(--p-content-background);
  border-radius: 6px;
  border: 1px solid var(--p-content-border-color);
}

.bulk-count {
  font-size: 0.85rem;
  font-weight: 600;
  color: var(--p-text-color);
}

.bulk-select {
  min-width: 180px;
}

.category-cell {
  display: flex;
  align-items: center;
  gap: 0.25rem;
}

.category-cell .category-select {
  flex: 1;
  min-width: 0;
}

.confirm-btn {
  flex-shrink: 0;
  background: var(--p-content-background) !important;
  border: 1px solid var(--p-content-border-color) !important;
  color: var(--p-green-500) !important;
}

.suggestion-active :deep(.p-select-label) {
  font-style: italic;
  opacity: 0.7;
}

.description-cell {
  display: block;
  max-width: 300px;
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

<style>
.row-uncategorized {
  background-color: color-mix(in srgb, var(--p-orange-500) 6%, transparent) !important;
}
</style>
