<script setup>
import { computed } from "vue";
import Chart from "primevue/chart";
import Tag from "primevue/tag";

const props = defineProps({
  summaries: { type: Map, required: true },
  year: { type: Number, required: true },
});

const MONTH_LABELS = [
  "Jan", "Feb", "Mar", "Apr", "May", "Jun",
  "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
];

const styles = getComputedStyle(document.documentElement);
const green = styles.getPropertyValue("--p-green-500").trim();
const red = styles.getPropertyValue("--p-red-500").trim();
const axisColor = styles.getPropertyValue("--p-text-muted-color").trim();

const sortedKeys = computed(() => {
  return [...props.summaries.keys()]
    .filter((key) => props.summaries.get(key).year === props.year)
    .sort();
});

const overallNet = computed(() => {
  let total = 0;
  for (const key of sortedKeys.value) {
    total += props.summaries.get(key).net_cents;
  }
  return total;
});

const netLinePlugin = {
  id: "netLine",
  afterDatasetsDraw(chart) {
    const yScale = chart.scales.y;
    const yPixel = yScale.getPixelForValue(overallNet.value);
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

const chartPlugins = [netLinePlugin];

const chartData = computed(() => {
  const labels = [];
  const income = [];
  const expenses = [];

  for (const key of sortedKeys.value) {
    const s = props.summaries.get(key);
    const label = `${MONTH_LABELS[s.month - 1]} ${String(s.year).slice(2)}`;
    labels.push(label);
    income.push(s.total_income_cents);
    expenses.push(s.total_expenses_cents);
  }

  return {
    labels,
    datasets: [
      { label: "Income", backgroundColor: green, data: income },
      { label: "Expenses", backgroundColor: red, data: expenses },
    ],
  };
});

const chartOptions = computed(() => ({
  responsive: true,
  maintainAspectRatio: false,
  plugins: {
    legend: { position: "top" },
    tooltip: {
      mode: "nearest",
      intersect: true,
      callbacks: {
        title(tooltipItems) {
          const item = tooltipItems[0];
          const amount = (item.raw / 100).toFixed(2);
          return `${item.dataset.label}: ${amount} EUR`;
        },
        label() {
          return null;
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
      suggestedMin: Math.min(overallNet.value, 0),
      suggestedMax: Math.max(overallNet.value, 0),
    },
  },
}));
</script>

<template>
  <section v-if="summaries.size > 0">
    <h2>Monthly Income & Expenses <Tag :value="String(year)" :pt="{ root: { class: 'year-tag' } }" /></h2>
    <div class="chart-container">
      <Chart type="bar" :data="chartData" :options="chartOptions" :plugins="chartPlugins"
        :pt="{ root: { style: 'height: 100%' } }" />
    </div>
  </section>
</template>

<style scoped>
h2 {
  font-size: 1.1rem;
  margin: 1.5rem 0 0.75rem 0;
}

.chart-container {
  height: 300px;
  padding: 1rem;
  border-radius: 6px;
  border: 1px solid var(--p-content-border-color);
  background: var(--p-content-background);
}

:deep(.year-tag) {
  background: var(--p-surface-900);
  color: var(--p-surface-0);
}
</style>
