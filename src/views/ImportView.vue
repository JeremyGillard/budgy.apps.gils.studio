<script setup lang="ts">
import { open } from "@tauri-apps/plugin-dialog";
import Button from "primevue/button";
import Message from "primevue/message";
import { useImport } from "../composables/useImport";

const { results, importing, importCsv } = useImport();

async function selectAndImport() {
  const selected = await open({
    multiple: true,
    filters: [{ name: "CSV", extensions: ["csv"] }],
  });

  if (!selected) return;

  const paths = Array.isArray(selected) ? selected : [selected];
  await importCsv(paths);
}
</script>

<template>
  <div class="view-container">
    <h1>Import CSV</h1>
    <p>Select one or more Belfius CSV files to import transactions.</p>

    <Button
      label="Select CSV Files"
      icon="pi pi-upload"
      :loading="importing"
      @click="selectAndImport"
    />

    <div v-if="results.length > 0" class="results">
      <h2>Import Results</h2>
      <div v-for="(result, index) in results" :key="index" class="result-item">
        <Message v-if="result.error" severity="error">
          {{ result.file_name }}: {{ result.error }}
        </Message>
        <Message v-else severity="success">
          {{ result.file_name }}: {{ result.imported_count }} imported,
          {{ result.skipped_count }} skipped
        </Message>
      </div>
    </div>
  </div>
</template>

<style scoped>
.view-container {
  padding: 1.5rem;
  flex: 1;
}

h1 {
  margin: 0 0 0.5rem 0;
  font-size: 1.5rem;
}

h2 {
  font-size: 1.125rem;
  margin: 1.5rem 0 0.75rem 0;
}

.results {
  margin-top: 1rem;
}

.result-item {
  margin-bottom: 0.5rem;
}
</style>
