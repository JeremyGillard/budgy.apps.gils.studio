import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import type { ImportResult } from "../types";

export function useImport() {
  const results = ref<ImportResult[]>([]);
  const importing = ref(false);

  async function importCsv(filePaths: string[]) {
    importing.value = true;
    try {
      results.value = await invoke<ImportResult[]>("import_csv", {
        filePaths,
      });
    } finally {
      importing.value = false;
    }
  }

  return {
    results,
    importing,
    importCsv,
  };
}
