<script setup>
import { ref, computed, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";

const emit = defineEmits(["navigate"]);

const props = defineProps({
  refreshKey: { type: Number, default: 0 },
});

const importedMonths = ref([]);

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

function hasData(year, month) {
  return importedSet.value.has(`${year}-${String(month).padStart(2, "0")}`);
}

function onCellClick(year, month) {
  if (hasData(year, month)) {
    emit("navigate", { year, month });
  }
}

async function loadMonths() {
  try {
    importedMonths.value = await invoke("get_imported_months");
  } catch (e) {
    console.error("Failed to load imported months:", e);
  }
}

onMounted(loadMonths);
</script>

<template>
  <section>
    <h2>Overview</h2>
    <p v-if="years.length === 0" class="empty-msg">
      No data imported yet. Import a CSV file to get started.
    </p>
    <div v-else class="overview-grid">
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
          @click="onCellClick(year, month)"
        >
          {{ month }}
        </div>
      </div>
    </div>
  </section>
</template>

<style scoped>
h2 {
  font-size: 1.1rem;
  margin: 0 0 0.75rem 0;
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
  text-align: center;
  padding: 0.5rem 0.25rem;
  border-radius: 6px;
  font-size: 0.8rem;
  font-weight: 500;
  transition: background 0.15s, transform 0.1s;
  user-select: none;
}

.month-cell.empty {
  background: var(--p-surface-200);
  color: var(--p-text-muted-color);
}

.month-cell.imported {
  background: var(--p-green-500);
  color: var(--p-green-contrast-color, #fff);
  cursor: pointer;
}

.month-cell.imported:hover {
  background: var(--p-green-600);
  transform: scale(1.05);
}
</style>
