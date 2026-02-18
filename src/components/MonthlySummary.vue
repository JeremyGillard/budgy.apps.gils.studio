<script setup>
import { ref, computed, watch } from "vue";
import { invoke } from "@tauri-apps/api/core";
import Button from "primevue/button";
import DatePicker from "primevue/datepicker";

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

const income = ref(null);
const expenses = ref(null);
const net = ref(null);

function formatAmount(cents) {
  if (cents == null) return "-";
  const val = (cents / 100).toFixed(2);
  return val.replace(/\B(?=(\d{3})+(?!\d))/g, ",");
}

async function loadSummary() {
  try {
    const summary = await invoke("monthly_summary", {
      year: props.year,
      month: props.month,
    });
    income.value = summary.total_income_cents;
    expenses.value = summary.total_expenses_cents;
    net.value = summary.net_cents;
  } catch (e) {
    console.error("Failed to load summary:", e);
  }
}

watch(
  () => [props.year, props.month, props.refreshKey],
  () => loadSummary(),
  { immediate: true },
);
</script>

<template>
  <section>
    <h2>Monthly Summary</h2>
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
    </div>
    <div class="summary-cards">
      <div class="card income">
        <span class="card-label">Income</span>
        <span class="card-value">+{{ formatAmount(income) }} EUR</span>
      </div>
      <div class="card expenses">
        <span class="card-label">Expenses</span>
        <span class="card-value">{{ formatAmount(expenses) }} EUR</span>
      </div>
      <div class="card net">
        <span class="card-label">Net</span>
        <span class="card-value">{{ formatAmount(net) }} EUR</span>
      </div>
    </div>
  </section>
</template>

<style scoped>
h2 {
  font-size: 1.1rem;
  margin: 0 0 0.75rem 0;
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

.summary-cards {
  display: flex;
  gap: 1rem;
  margin-bottom: 1.5rem;
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

.card.income .card-value {
  color: var(--p-green-500);
}

.card.expenses .card-value {
  color: var(--p-red-500);
}

.card.net .card-value {
  color: var(--p-blue-500);
}
</style>
