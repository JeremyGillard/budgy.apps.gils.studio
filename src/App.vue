<script setup>
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import Button from "primevue/button";
import Select from "primevue/select";
import Toast from "primevue/toast";
import { useToast } from "primevue/usetoast";
import MonthlySummary from "./components/MonthlySummary.vue";
import DailyChart from "./components/DailyChart.vue";
import TransactionTable from "./components/TransactionTable.vue";
import Sidebar from "./components/Sidebar.vue";
import CategorizeView from "./components/CategorizeView.vue";
import CategoriesView from "./components/CategoriesView.vue";
import OverviewView from "./components/OverviewView.vue";
import LockScreen from "./components/LockScreen.vue";

const toast = useToast();

const unlocked = ref(false);
const dbStatus = ref(null);

const now = new Date();
const currentYear = ref(now.getFullYear());
const currentMonth = ref(now.getMonth() + 1);
const currentPage = ref("months");

const accounts = ref([]);
const selectedAccount = ref(null);

const refreshKey = ref(0);

onMounted(async () => {
  try {
    dbStatus.value = await invoke("get_db_status");
  } catch (e) {
    console.error("Failed to get DB status:", e);
  }
});

function onUnlocked() {
  unlocked.value = true;
  loadAccounts();
}

async function loadAccounts() {
  try {
    const result = await invoke("list_accounts");
    accounts.value = result.map((a) => ({
      label: a.label || a.iban,
      value: a.id,
    }));
  } catch (e) {
    console.error("Failed to load accounts:", e);
  }
}

async function importCsv() {
  try {
    const filePaths = await open({
      multiple: true,
      filters: [{ name: "CSV", extensions: ["csv"] }],
    });

    if (!filePaths?.length) return;

    const results = await invoke("import_csv", { filePaths });

    const totalImported = results.reduce((s, r) => s + r.imported, 0);
    const totalSkipped = results.reduce((s, r) => s + r.skipped_duplicates, 0);
    let detail = `Imported ${totalImported} transactions from ${results.length} file(s)`;
    if (totalSkipped > 0) {
      detail += ` (${totalSkipped} duplicates skipped)`;
    }

    toast.add({ severity: "success", summary: "Import complete", detail, life: 5000 });

    const firstDate = results.find((r) => r.date_from)?.date_from;
    if (firstDate) {
      const [y, m] = firstDate.split("-").map(Number);
      currentYear.value = y;
      currentMonth.value = m;
    }

    await loadAccounts();
    refreshKey.value++;
  } catch (e) {
    toast.add({ severity: "error", summary: "Import failed", detail: String(e), life: 8000 });
  }
}

function prevMonth() {
  if (currentMonth.value === 1) {
    currentMonth.value = 12;
    currentYear.value--;
  } else {
    currentMonth.value--;
  }
}

function nextMonth() {
  if (currentMonth.value === 12) {
    currentMonth.value = 1;
    currentYear.value++;
  } else {
    currentMonth.value++;
  }
}
</script>

<template>
  <LockScreen
    v-if="!unlocked && dbStatus !== null"
    :dbStatus="dbStatus"
    @unlocked="onUnlocked"
  />
  <div v-if="unlocked" class="app-layout">
    <Sidebar :currentPage="currentPage" @navigate="currentPage = $event" />
    <div class="main-content">
      <Toast />
      <div class="app-container">
        <header>
          <div class="actions">
            <Button label="Import CSV" icon="pi pi-upload" @click="importCsv" />
            <Select
              v-model="selectedAccount"
              :options="accounts"
              optionLabel="label"
              optionValue="value"
              placeholder="All accounts"
              showClear
              class="account-select"
            />
          </div>
        </header>

        <OverviewView
          v-if="currentPage === 'overview'"
          :refresh-key="refreshKey"
          @navigate="({ year, month }) => { currentYear = year; currentMonth = month; currentPage = 'months'; }"
        />

        <template v-if="currentPage === 'months'">
          <MonthlySummary
            :year="currentYear"
            :month="currentMonth"
            :refresh-key="refreshKey"
            @prev="prevMonth"
            @next="nextMonth"
            @navigate="({ year, month }) => { currentYear = year; currentMonth = month; }"
          />

          <DailyChart
            :year="currentYear"
            :month="currentMonth"
            :refresh-key="refreshKey"
          />

          <TransactionTable
            :year="currentYear"
            :month="currentMonth"
            :refresh-key="refreshKey"
          />
        </template>

        <CategoriesView v-if="currentPage === 'categories'" />

        <CategorizeView
          v-if="currentPage === 'categorize'"
          :year="currentYear"
          :month="currentMonth"
          :refresh-key="refreshKey"
          @prev="prevMonth"
          @next="nextMonth"
          @navigate="({ year, month }) => { currentYear = year; currentMonth = month; }"
        />
      </div>
    </div>
  </div>
</template>

<style>
:root {
  font-family: system-ui, -apple-system, sans-serif;
  font-size: 14px;
}

* {
  box-sizing: border-box;
}

body {
  margin: 0;
  padding: 0;
  background-color: var(--p-surface-ground);
  color: var(--p-text-color);
}
</style>

<style scoped>
.app-layout {
  display: flex;
  height: 100vh;
}

.main-content {
  flex: 1;
  overflow-y: auto;
  padding: 1.5rem;
}

.app-container {
  max-width: 1100px;
  margin: 0 auto;
}

header {
  display: flex;
  justify-content: flex-end;
  align-items: center;
  margin-bottom: 1.5rem;
}

.actions {
  display: flex;
  gap: 0.5rem;
  align-items: center;
}

.account-select {
  min-width: 200px;
}
</style>
