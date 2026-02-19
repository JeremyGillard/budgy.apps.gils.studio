<script setup>
import { ref, computed, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import DataTable from "primevue/datatable";
import Column from "primevue/column";
import Button from "primevue/button";
import Dialog from "primevue/dialog";
import InputText from "primevue/inputtext";
import ColorPicker from "primevue/colorpicker";
import Select from "primevue/select";
import { useToast } from "primevue/usetoast";

const emit = defineEmits(["categories-changed"]);
const toast = useToast();

const categories = ref([]);
const loading = ref(false);

const showDialog = ref(false);
const editingId = ref(null);
const form = ref({ name: "", parent_id: null, icon: null, color: null });

const showDeleteConfirm = ref(false);
const deletingCategory = ref(null);
const reassignTo = ref(null);

const isEditing = computed(() => editingId.value !== null);

const parentOptions = computed(() => {
  const opts = categories.value
    .filter((c) => c.id !== editingId.value)
    .map((c) => ({ label: c.name, value: c.id }));
  return [{ label: "None", value: null }, ...opts];
});

const reassignOptions = computed(() => {
  if (!deletingCategory.value) return [];
  return categories.value
    .filter((c) => c.id !== deletingCategory.value.id)
    .map((c) => ({ label: c.name, value: c.id }));
});

const categoryMap = computed(() => {
  const map = {};
  for (const c of categories.value) {
    map[c.id] = c;
  }
  return map;
});

async function loadCategories() {
  loading.value = true;
  try {
    categories.value = await invoke("list_categories");
  } catch (e) {
    toast.add({ severity: "error", summary: "Failed to load categories", detail: String(e), life: 5000 });
  } finally {
    loading.value = false;
  }
}

function openCreate() {
  editingId.value = null;
  form.value = { name: "", parent_id: null, icon: null, color: null };
  showDialog.value = true;
}

function openEdit(category) {
  editingId.value = category.id;
  form.value = {
    name: category.name,
    parent_id: category.parent_id,
    icon: category.icon,
    color: category.color ? category.color.replace("#", "") : null,
  };
  showDialog.value = true;
}

async function saveCategory() {
  const payload = {
    name: form.value.name,
    parent_id: form.value.parent_id,
    icon: form.value.icon || null,
    color: form.value.color ? `#${form.value.color}` : null,
  };

  try {
    if (isEditing.value) {
      await invoke("update_category", { id: editingId.value, changes: payload });
      toast.add({ severity: "success", summary: "Category updated", life: 3000 });
    } else {
      await invoke("create_category", { input: payload });
      toast.add({ severity: "success", summary: "Category created", life: 3000 });
    }
    showDialog.value = false;
    await loadCategories();
    emit("categories-changed");
  } catch (e) {
    toast.add({ severity: "error", summary: "Save failed", detail: String(e), life: 5000 });
  }
}

function confirmDelete(category) {
  deletingCategory.value = category;
  const uncategorized = categories.value.find((c) => c.name === "Uncategorized");
  reassignTo.value = uncategorized ? uncategorized.id : null;
  showDeleteConfirm.value = true;
}

async function executeDelete() {
  try {
    const targetName = categories.value.find((c) => c.id === reassignTo.value)?.name ?? "Unknown";
    await invoke("delete_category", { id: deletingCategory.value.id, reassignTo: reassignTo.value });
    toast.add({ severity: "success", summary: "Category deleted", detail: `"${deletingCategory.value.name}" deleted. Transactions reassigned to "${targetName}".`, life: 4000 });
    showDeleteConfirm.value = false;
    deletingCategory.value = null;
    await loadCategories();
    emit("categories-changed");
  } catch (e) {
    toast.add({ severity: "error", summary: "Delete failed", detail: String(e), life: 5000 });
  }
}

function isUncategorized(category) {
  return category.name === "Uncategorized";
}

onMounted(loadCategories);
</script>

<template>
  <section>
    <div class="categories-header">
      <h2>Categories</h2>
      <Button label="Add Category" icon="pi pi-plus" @click="openCreate" />
    </div>

    <DataTable :value="categories" :loading="loading" dataKey="id" tableStyle="min-width: 40rem">
      <Column header="" style="width: 3rem">
        <template #body="{ data }">
          <span
            class="color-swatch"
            :style="{ background: data.color || 'var(--p-surface-400)' }"
          />
        </template>
      </Column>
      <Column field="name" header="Name" style="width: 35%" />
      <Column header="Parent" style="width: 25%">
        <template #body="{ data }">
          {{ data.parent_id ? (categoryMap[data.parent_id]?.name ?? '—') : '—' }}
        </template>
      </Column>
      <Column header="Actions" style="width: 10rem">
        <template #body="{ data }">
          <div class="action-buttons">
            <Button
              icon="pi pi-pencil"
              text
              rounded
              size="small"
              aria-label="Edit"
              @click="openEdit(data)"
            />
            <Button
              icon="pi pi-trash"
              text
              rounded
              size="small"
              severity="danger"
              aria-label="Delete"
              :disabled="isUncategorized(data)"
              @click="confirmDelete(data)"
            />
          </div>
        </template>
      </Column>
    </DataTable>

    <Dialog
      v-model:visible="showDialog"
      :header="isEditing ? 'Edit Category' : 'New Category'"
      modal
      :style="{ width: '28rem' }"
    >
      <div class="form-field">
        <label for="cat-name">Name</label>
        <InputText id="cat-name" v-model="form.name" class="w-full" />
      </div>
      <div class="form-field">
        <label for="cat-color">Color</label>
        <ColorPicker id="cat-color" v-model="form.color" />
      </div>
      <div class="form-field">
        <label for="cat-parent">Parent</label>
        <Select
          id="cat-parent"
          v-model="form.parent_id"
          :options="parentOptions"
          optionLabel="label"
          optionValue="value"
          placeholder="None"
          class="w-full"
        />
      </div>
      <template #footer>
        <Button label="Cancel" text @click="showDialog = false" />
        <Button label="Save" icon="pi pi-check" :disabled="!form.name.trim()" @click="saveCategory" />
      </template>
    </Dialog>

    <Dialog
      v-model:visible="showDeleteConfirm"
      header="Delete Category"
      modal
      :style="{ width: '28rem' }"
    >
      <p>
        Delete <strong>{{ deletingCategory?.name }}</strong>?
        Its transactions will be reassigned to the category below.
      </p>
      <div class="form-field">
        <label for="reassign-category">Reassign to</label>
        <Select
          id="reassign-category"
          v-model="reassignTo"
          :options="reassignOptions"
          optionLabel="label"
          optionValue="value"
          placeholder="Select a category"
          class="w-full"
        />
      </div>
      <template #footer>
        <Button label="Cancel" text @click="showDeleteConfirm = false" />
        <Button label="Delete" icon="pi pi-trash" severity="danger" :disabled="!reassignTo" @click="executeDelete" />
      </template>
    </Dialog>
  </section>
</template>

<style scoped>
.categories-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 1rem;
}

.categories-header h2 {
  font-size: 1.1rem;
  margin: 0;
}

.color-swatch {
  display: inline-block;
  width: 1.25rem;
  height: 1.25rem;
  border-radius: 4px;
  border: 1px solid var(--p-content-border-color);
}

.action-buttons {
  display: flex;
  gap: 0.25rem;
}

.form-field {
  margin-bottom: 1rem;
}

.form-field label {
  display: block;
  font-size: 0.85rem;
  font-weight: 600;
  margin-bottom: 0.35rem;
  color: var(--p-text-muted-color);
}

.w-full {
  width: 100%;
}
</style>
