<script setup>
import { ref, watch } from "vue";
import { invoke } from "@tauri-apps/api/core";
import Chart from "primevue/chart";

const props = defineProps({
  year: { type: Number, required: true },
  month: { type: Number, required: true },
  refreshKey: { type: Number, default: 0 },
});

const chartData = ref(null);
const chartOptions = ref({
  responsive: true,
  maintainAspectRatio: false,
  plugins: {
    legend: { position: "top" },
  },
  scales: {
    x: { stacked: false },
    y: {
      ticks: {
        callback: (value) => (value / 100).toFixed(0),
      },
    },
  },
});

function daysInMonth(year, month) {
  return new Date(year, month, 0).getDate();
}

async function loadDailySummary() {
  try {
    const data = await invoke("daily_summary", {
      year: props.year,
      month: props.month,
    });

    const totalDays = daysInMonth(props.year, props.month);
    const byDate = new Map(data.map((d) => [d.date, d]));

    const labels = [];
    const income = [];
    const expenses = [];

    for (let day = 1; day <= totalDays; day++) {
      const dateStr = `${props.year}-${String(props.month).padStart(2, "0")}-${String(day).padStart(2, "0")}`;
      labels.push(String(day));
      const entry = byDate.get(dateStr);
      income.push(entry ? entry.total_income_cents : 0);
      expenses.push(entry ? entry.total_expenses_cents : 0);
    }

    const styles = getComputedStyle(document.documentElement);
    const green = styles.getPropertyValue("--p-green-500").trim();
    const red = styles.getPropertyValue("--p-red-500").trim();

    chartData.value = {
      labels,
      datasets: [
        {
          label: "Income",
          backgroundColor: green,
          data: income,
        },
        {
          label: "Expenses",
          backgroundColor: red,
          data: expenses,
        },
      ],
    };
  } catch (e) {
    console.error("Failed to load daily summary:", e);
  }
}

watch(
  () => [props.year, props.month, props.refreshKey],
  () => loadDailySummary(),
  { immediate: true },
);
</script>

<template>
  <section v-if="chartData">
    <h2>Daily Income & Expenses</h2>
    <div class="chart-container">
      <Chart type="bar" :data="chartData" :options="chartOptions" style="height: 100%" />
    </div>
  </section>
</template>

<style scoped>
h2 {
  font-size: 1.1rem;
  margin: 0 0 0.75rem 0;
}

.chart-container {
  height: 300px;
  margin-bottom: 1.5rem;
  padding: 1rem;
  border-radius: 6px;
  border: 1px solid var(--p-content-border-color);
  background: var(--p-content-background);
}
</style>
