<script setup>
import { ref, watch, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import DataTable from "primevue/datatable";
import Column from "primevue/column";
import { useToast } from "primevue/usetoast";

const props = defineProps({
  categoriesVersion: { type: Number, default: 0 },
});

const toast = useToast();
const rows = ref([]);
const loading = ref(false);

async function load() {
  loading.value = true;
  try {
    rows.value = await invoke("category_transaction_counts");
  } catch (e) {
    toast.add({ severity: "error", summary: "Failed to load insights", detail: String(e), life: 5000 });
  } finally {
    loading.value = false;
  }
}

watch(() => props.categoriesVersion, load);
onMounted(load);
</script>

<template>
  <section>
    <h2>Category Insights</h2>
    <DataTable :value="rows" :loading="loading" dataKey="category_id" tableStyle="min-width: 30rem">
      <Column header="" style="width: 3rem">
        <template #body="{ data }">
          <span
            class="color-swatch"
            :style="{ background: data.category_color || 'var(--p-surface-400)' }"
          />
        </template>
      </Column>
      <Column field="category_name" header="Category" />
      <Column field="count" header="Transactions" style="width: 10rem" />
    </DataTable>
  </section>
</template>

<style scoped>
h2 {
  font-size: 1.1rem;
  margin: 0 0 1rem 0;
}

.color-swatch {
  display: inline-block;
  width: 1.25rem;
  height: 1.25rem;
  border-radius: 4px;
  border: 1px solid var(--p-content-border-color);
}
</style>
