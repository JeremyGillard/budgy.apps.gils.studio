<script setup>
import { ref, watch } from "vue";
import { invoke } from "@tauri-apps/api/core";
import Tag from "primevue/tag";

const props = defineProps({
  year: { type: Number, required: true },
  refreshKey: { type: Number, default: 0 },
});

const earnings = ref(null);
const expenses = ref(null);

function fmt(cents) {
  return (cents / 100).toFixed(2);
}

async function load() {
  if (!props.year) return;
  try {
    const [e, x] = await Promise.all([
      invoke("yearly_earnings", { year: props.year }),
      invoke("yearly_expenses", { year: props.year }),
    ]);
    earnings.value = e;
    expenses.value = x;
  } catch (err) {
    console.error("Failed to load yearly totals:", err);
  }
}

watch(() => [props.year, props.refreshKey], () => load(), { immediate: true });
</script>

<template>
  <section v-if="earnings && expenses">
    <h2>Yearly Totals <Tag :value="String(year)" :pt="{ root: { class: 'year-tag' } }" /></h2>
    <div class="cards">
      <div class="card earnings">
        <div class="card-total green">+ {{ fmt(earnings.total_cents) }} EUR</div>
        <h3>Top Earnings</h3>
        <ul class="cat-list">
          <li v-for="(cat, i) in earnings.categories" :key="i">
            <span class="dot" :style="{ background: cat.category_color || '#9E9E9E' }"></span>
            <span class="cat-name">{{ cat.category_name || "Uncategorized" }}</span>
            <span class="cat-amount green">{{ fmt(cat.total_cents) }}</span>
          </li>
        </ul>
        <p v-if="earnings.categories.length === 0" class="empty">No earnings</p>
      </div>
      <div class="card expenses">
        <div class="card-total red">{{ fmt(expenses.total_cents) }} EUR</div>
        <h3>Top Expenses</h3>
        <ul class="cat-list">
          <li v-for="(cat, i) in expenses.categories" :key="i">
            <span class="dot" :style="{ background: cat.category_color || '#9E9E9E' }"></span>
            <span class="cat-name">{{ cat.category_name || "Uncategorized" }}</span>
            <span class="cat-amount red">{{ fmt(cat.total_cents) }}</span>
          </li>
        </ul>
        <p v-if="expenses.categories.length === 0" class="empty">No expenses</p>
      </div>
    </div>
  </section>
</template>

<style scoped>
h2 {
  font-size: 1.1rem;
  margin: 1.5rem 0 0.75rem 0;
}

h3 {
  font-size: 0.9rem;
  margin: 0.75rem 0 0.5rem 0;
  color: var(--p-text-muted-color);
}

.cards {
  display: flex;
  gap: 1rem;
}

.card {
  flex: 1;
  padding: 1rem;
  border-radius: 6px;
  border: 1px solid var(--p-content-border-color);
  background: var(--p-content-background);
}

.card-total {
  font-size: 1.3rem;
  font-weight: 700;
}

.green { color: var(--p-green-500); }
.red { color: var(--p-red-500); }

.cat-list {
  list-style: none;
  margin: 0;
  padding: 0;
}

.cat-list li {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.3rem 0;
  font-size: 0.85rem;
}

.dot {
  width: 10px;
  height: 10px;
  border-radius: 50%;
  flex-shrink: 0;
}

.cat-name {
  flex: 1;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.cat-amount {
  font-weight: 600;
  white-space: nowrap;
}

.empty {
  color: var(--p-text-muted-color);
  font-size: 0.85rem;
}

:deep(.year-tag) {
  background: var(--p-surface-900);
  color: var(--p-surface-0);
}
</style>
