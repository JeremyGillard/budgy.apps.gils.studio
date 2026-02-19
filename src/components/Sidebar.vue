<script setup>
defineProps({
  currentPage: { type: String, default: "months" },
});

const emit = defineEmits(["navigate"]);
</script>

<template>
  <aside class="sidebar">
    <div class="brand">
      <img src="/icon.png" alt="Budgy" class="brand-icon" />
      <span class="brand-title">Budgy</span>
    </div>
    <nav>
      <a
        class="nav-item"
        :class="{ active: currentPage === 'overview' }"
        href="#"
        @click.prevent="emit('navigate', 'overview')"
      >
        <i class="pi pi-th-large"></i>
        <span>Overview</span>
      </a>
      <a
        class="nav-item"
        :class="{ active: currentPage === 'months' }"
        href="#"
        @click.prevent="emit('navigate', 'months')"
      >
        <i class="pi pi-calendar"></i>
        <span>Monthly Balance</span>
      </a>
      <a
        class="nav-item"
        :class="{ active: currentPage === 'categorize' }"
        href="#"
        @click.prevent="emit('navigate', 'categorize')"
      >
        <i class="pi pi-tags"></i>
        <span>Categorize</span>
      </a>
      <a
        class="nav-item"
        :class="{ active: currentPage === 'categories' }"
        href="#"
        @click.prevent="emit('navigate', 'categories')"
      >
        <i class="pi pi-bookmark"></i>
        <span>Categories</span>
      </a>
    </nav>
  </aside>
</template>

<style scoped>
.sidebar {
  width: 200px;
  min-width: 200px;
  height: 100vh;
  background: var(--p-content-background);
  border-right: 1px solid var(--p-content-border-color);
  box-shadow: 2px 0 8px rgba(0, 0, 0, 0.15);
  display: flex;
  flex-direction: column;
  padding: 1rem 0;
}

.brand {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.5rem 1rem 1.5rem;
}

.brand-icon {
  width: 28px;
  height: 28px;
}

.brand-title {
  font-size: 1.25rem;
  font-weight: 700;
  color: var(--p-text-color);
}

nav {
  display: flex;
  flex-direction: column;
  gap: 0.125rem;
}

.nav-item {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.5rem 1rem;
  color: var(--p-text-muted-color);
  text-decoration: none;
  font-size: 0.875rem;
  border-radius: 0;
  transition: background 0.15s;
}

.nav-item:hover {
  background: var(--p-surface-hover);
  color: var(--p-text-color);
}

.nav-item.active {
  background: var(--p-primary-color);
  color: var(--p-primary-contrast-color);
}

.nav-item i {
  font-size: 1rem;
}
</style>
