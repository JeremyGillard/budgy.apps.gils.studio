<script setup lang="ts">
import { useRouter, useRoute } from "vue-router";
import Button from "primevue/button";

const router = useRouter();
const route = useRoute();

function isActive(path: string): boolean {
  return route.path === path;
}
</script>

<template>
  <aside class="app-sidebar">
    <div class="sidebar-header">
      <h2>Budgy</h2>
    </div>
    <nav class="sidebar-nav">
      <Button
        label="Transactions"
        icon="pi pi-list"
        :severity="isActive('/') ? undefined : 'secondary'"
        text
        class="nav-button"
        @click="router.push('/')"
      />
      <Button
        label="Import"
        icon="pi pi-upload"
        :severity="isActive('/import') ? undefined : 'secondary'"
        text
        class="nav-button"
        @click="router.push('/import')"
      />
    </nav>
  </aside>
</template>

<style scoped>
.app-sidebar {
  width: 200px;
  min-height: 100vh;
  border-right: 1px solid #e2e8f0;
  padding: 1rem;
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.sidebar-header h2 {
  margin: 0;
  font-size: 1.25rem;
}

.sidebar-nav {
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
}

.nav-button {
  justify-content: flex-start;
  width: 100%;
}
</style>
