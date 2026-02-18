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
const txByDate = ref(new Map());
const netCents = ref(0);

const netLinePlugin = {
  id: "netLine",
  afterDatasetsDraw(chart) {
    const yScale = chart.scales.y;
    const yPixel = yScale.getPixelForValue(netCents.value);
    const { left, right } = chart.chartArea;
    const ctx = chart.ctx;
    ctx.save();
    ctx.beginPath();
    ctx.setLineDash([6, 4]);
    ctx.strokeStyle = getComputedStyle(document.documentElement)
      .getPropertyValue("--p-blue-500")
      .trim();
    ctx.lineWidth = 1.5;
    ctx.moveTo(left, yPixel);
    ctx.lineTo(right, yPixel);
    ctx.stroke();
    ctx.restore();
  },
};
const chartPlugins = ref([netLinePlugin]);

const axisColor = getComputedStyle(document.documentElement)
  .getPropertyValue("--p-text-muted-color")
  .trim();

const chartOptions = ref({
  responsive: true,
  maintainAspectRatio: false,
  plugins: {
    legend: { position: "top" },
    tooltip: {
      mode: "nearest",
      intersect: true,
      bodyFont: { size: 11 },
      callbacks: {
        title(tooltipItems) {
          const item = tooltipItems[0];
          const total = (item.raw / 100).toFixed(2);
          return `${item.dataset.label}: ${total} EUR`;
        },
        label() {
          return null;
        },
        afterBody(tooltipItems) {
          if (!tooltipItems.length) return [];
          const item = tooltipItems[0];
          const day = item.dataIndex + 1;
          const dateStr = `${props.year}-${String(props.month).padStart(2, "0")}-${String(day).padStart(2, "0")}`;
          const txs = txByDate.value.get(dateStr);
          if (!txs?.length) return [];
          const isIncome = item.datasetIndex === 0;
          const filtered = txs.filter((tx) =>
            isIncome ? tx.amount_cents > 0 : tx.amount_cents < 0,
          );
          return filtered.map((tx) => {
            const desc =
              tx.description.length > 40
                ? tx.description.slice(0, 40) + "..."
                : tx.description;
            const amount = (tx.amount_cents / 100).toFixed(2);
            return `${desc} — ${amount} EUR`;
          });
        },
      },
    },
  },
  scales: {
    x: {
      stacked: false,
      border: { display: false },
      ticks: { color: axisColor },
    },
    y: {
      border: { color: axisColor },
      grid: {
        color: (ctx) =>
          ctx.tick.value === 0 ? axisColor : "rgba(160,160,160,0.2)",
      },
      ticks: {
        color: axisColor,
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
    const [data, transactions] = await Promise.all([
      invoke("daily_summary", { year: props.year, month: props.month }),
      invoke("list_transactions_by_month", { year: props.year, month: props.month }),
    ]);

    const grouped = new Map();
    for (const tx of transactions) {
      const list = grouped.get(tx.accounting_date) || [];
      list.push(tx);
      grouped.set(tx.accounting_date, list);
    }
    txByDate.value = grouped;

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

    netCents.value =
      income.reduce((s, v) => s + v, 0) + expenses.reduce((s, v) => s + v, 0);

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
      <Chart type="bar" :data="chartData" :options="chartOptions" :plugins="chartPlugins"
        :pt="{ root: { style: 'height: 100%' } }" />
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
