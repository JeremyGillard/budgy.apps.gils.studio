<script setup lang="ts">
import TransactionTable from "../components/TransactionTable.vue";
</script>

<template>
  <div class="view-container">
    <h1>Transactions</h1>
    <TransactionTable />
  </div>
</template>

<style scoped>
.view-container {
  padding: 1.5rem;
  flex: 1;
}

h1 {
  margin: 0 0 1rem 0;
  font-size: 1.5rem;
}
</style>
