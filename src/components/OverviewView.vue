<script setup>
import { ref, computed, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import CategorySpendChart from "./CategorySpendChart.vue";
import YearlyTotals from "./YearlyTotals.vue";
import MonthlyBalanceChart from "./MonthlyBalanceChart.vue";

const emit = defineEmits(["navigate"]);

const props = defineProps({
  refreshKey: { type: Number, default: 0 },
});

const importedMonths = ref([]);
const summaries = ref(new Map());

const MONTH_LABELS = [
  "Jan", "Feb", "Mar", "Apr", "May", "Jun",
  "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
];

const importedSet = computed(() => {
  const set = new Set();
  for (const m of importedMonths.value) {
    set.add(`${m.year}-${String(m.month).padStart(2, "0")}`);
  }
  return set;
});

const years = computed(() => {
  if (importedMonths.value.length === 0) return [];
  const ys = importedMonths.value.map((m) => m.year);
  const min = Math.min(...ys);
  const max = Math.max(...ys);
  const result = [];
  for (let y = min; y <= max; y++) {
    result.push(y);
  }
  return result;
});

const maxAbsBalance = computed(() => {
  let max = 0;
  for (const s of summaries.value.values()) {
    max = Math.max(max, Math.abs(s.net_cents));
  }
  return max;
});

const lastFullYear = computed(() => {
  if (importedMonths.value.length === 0) return null;
  const ys = [...new Set(importedMonths.value.map((m) => m.year))];
  const maxYear = Math.max(...ys);
  const currentYear = new Date().getFullYear();
  if (maxYear >= currentYear && ys.includes(maxYear - 1)) {
    return maxYear - 1;
  }
  return maxYear;
});

function hasData(year, month) {
  return importedSet.value.has(`${year}-${String(month).padStart(2, "0")}`);
}

function cellStyle(year, month) {
  const key = `${year}-${String(month).padStart(2, "0")}`;
  const s = summaries.value.get(key);
  if (!s || maxAbsBalance.value === 0) return {};

  const ratio = Math.abs(s.net_cents) / maxAbsBalance.value;
  const alpha = 0.15 + ratio * 0.45;

  if (s.net_cents > 0) {
    return { background: `rgba(76,175,80,${alpha})` };
  } else if (s.net_cents < 0) {
    return { background: `rgba(211,47,47,${alpha})` };
  }

  return {};
}

function onCellClick(year, month) {
  if (hasData(year, month)) {
    emit("navigate", { year, month });
  }
}

async function loadMonths() {
  try {
    importedMonths.value = await invoke("get_imported_months");

    const results = await Promise.all(
      importedMonths.value.map((m) =>
        invoke("monthly_summary", { year: m.year, month: m.month })
      )
    );

    const map = new Map();
    for (const s of results) {
      map.set(`${s.year}-${String(s.month).padStart(2, "0")}`, s);
    }
    summaries.value = map;
  } catch (e) {
    console.error("Failed to load imported months:", e);
  }
}

onMounted(loadMonths);
</script>

<template>
  <section>
    <h2 class="page-title">Overview</h2>
    <p v-if="years.length === 0" class="empty-msg">
      No data imported yet. Import a CSV file to get started.
    </p>
    <template v-else>
      <YearlyTotals v-if="lastFullYear" :year="lastFullYear" :refresh-key="props.refreshKey" />
      <MonthlyBalanceChart v-if="lastFullYear" :summaries="summaries" :year="lastFullYear" />
      <CategorySpendChart :refresh-key="props.refreshKey" />
      <h2>All months</h2>
      <div class="overview-grid">
        <div class="grid-header">
          <div class="year-label"></div>
          <div v-for="label in MONTH_LABELS" :key="label" class="month-header">
            {{ label }}
          </div>
        </div>
        <div v-for="year in years" :key="year" class="grid-row">
          <div class="year-label">{{ year }}</div>
          <div
            v-for="month in 12"
            :key="month"
            class="month-cell"
            :class="{ imported: hasData(year, month), empty: !hasData(year, month) }"
            :style="hasData(year, month) ? cellStyle(year, month) : {}"
            @click="onCellClick(year, month)"
          >
            {{ month }}
          </div>
        </div>
      </div>
    </template>
  </section>
</template>

<style scoped>
h2 {
  font-size: 1.1rem;
  margin: 1.5rem 0 0.75rem 0;
}

.page-title {
  margin-top: 0;
}

.empty-msg {
  color: var(--p-text-muted-color);
}

.overview-grid {
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
}

.grid-header,
.grid-row {
  display: grid;
  grid-template-columns: 60px repeat(12, 1fr);
  gap: 0.25rem;
}

.year-label {
  font-weight: 700;
  font-size: 0.85rem;
  display: flex;
  align-items: center;
  color: var(--p-text-color);
}

.month-header {
  text-align: center;
  font-size: 0.75rem;
  font-weight: 600;
  color: var(--p-text-muted-color);
  padding: 0.25rem 0;
}

.month-cell {
  position: relative;
  min-height: 48px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 6px;
  font-size: 0.8rem;
  font-weight: 500;
  transition: background 0.15s, transform 0.1s;
  user-select: none;
}

.month-cell.empty {
  background: var(--p-surface-600);
  color: var(--p-text-muted-color);
}

.month-cell.imported {
  background: var(--p-surface-100);
  color: var(--p-text-color);
  cursor: pointer;
}

.month-cell.imported:hover {
  filter: brightness(0.9);
  transform: scale(1.05);
}
</style>
