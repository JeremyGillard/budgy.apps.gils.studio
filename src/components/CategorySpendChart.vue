<script setup>
import { ref, watch } from "vue";
import { invoke } from "@tauri-apps/api/core";
import Chart from "primevue/chart";

const props = defineProps({
  refreshKey: { type: Number, default: 0 },
});

const categories = ref([]);
const chartData = ref(null);

const chartOptions = ref({
  responsive: true,
  maintainAspectRatio: false,
  plugins: {
    legend: { display: false },
    tooltip: {
      callbacks: {
        label(ctx) {
          const value = (ctx.raw / 100).toFixed(2);
          return `${ctx.label}: ${value} EUR/month`;
        },
      },
    },
  },
});

function fmt(cents) {
  return (cents / 100).toFixed(2);
}

async function load() {
  try {
    const data = await invoke("avg_monthly_spend");
    categories.value = data;

    if (data.length === 0) {
      chartData.value = null;
      return;
    }

    chartData.value = {
      labels: data.map((c) => c.category_name || "Uncategorized"),
      datasets: [
        {
          data: data.map((c) => Math.abs(c.avg_cents_per_month)),
          backgroundColor: data.map((c) => c.category_color || "#9E9E9E"),
        },
      ],
    };
  } catch (err) {
    console.error("Failed to load avg monthly spend:", err);
  }
}

watch(() => props.refreshKey, () => load(), { immediate: true });
</script>

<template>
  <section v-if="categories.length > 0">
    <h2>Average Monthly Spend by Category</h2>
    <div class="spend-layout">
      <ul class="cat-list">
        <li v-for="(cat, i) in categories" :key="i">
          <span class="dot" :style="{ background: cat.category_color || '#9E9E9E' }"></span>
          <span class="cat-name">{{ cat.category_name || "Uncategorized" }}</span>
          <span class="cat-amount">{{ fmt(cat.avg_cents_per_month) }}</span>
        </li>
      </ul>
      <div class="chart-wrap" v-if="chartData">
        <Chart type="pie" :data="chartData" :options="chartOptions"
          :pt="{ root: { style: 'height: 100%' } }" />
      </div>
    </div>
  </section>
</template>

<style scoped>
h2 {
  font-size: 1.1rem;
  margin: 1.5rem 0 0.75rem 0;
}

.spend-layout {
  display: flex;
  gap: 1.5rem;
  padding: 1rem;
  border-radius: 6px;
  border: 1px solid var(--p-content-border-color);
  background: var(--p-content-background);
}

.cat-list {
  flex: 1;
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
  color: var(--p-red-500);
}

.chart-wrap {
  width: 280px;
  height: 280px;
  flex-shrink: 0;
}
</style>
