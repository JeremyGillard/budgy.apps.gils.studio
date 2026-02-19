<script setup>
import { ref } from "vue";
import Tabs from "primevue/tabs";
import TabList from "primevue/tablist";
import Tab from "primevue/tab";
import TabPanels from "primevue/tabpanels";
import TabPanel from "primevue/tabpanel";
import CategorizeView from "./CategorizeView.vue";
import CategoriesView from "./CategoriesView.vue";

defineProps({
  year: { type: Number, required: true },
  month: { type: Number, required: true },
  refreshKey: { type: Number, default: 0 },
});

const emit = defineEmits(["prev", "next", "navigate"]);

const categoriesVersion = ref(0);
</script>

<template>
  <Tabs value="categorize">
    <TabList>
      <Tab value="categorize">Categorize</Tab>
      <Tab value="manage">Manage</Tab>
    </TabList>
    <TabPanels>
      <TabPanel value="categorize">
        <CategorizeView
          :year="year"
          :month="month"
          :refresh-key="refreshKey"
          :categories-version="categoriesVersion"
          @prev="emit('prev')"
          @next="emit('next')"
          @navigate="emit('navigate', $event)"
        />
      </TabPanel>
      <TabPanel value="manage">
        <CategoriesView @categories-changed="categoriesVersion++" />
      </TabPanel>
    </TabPanels>
  </Tabs>
</template>
