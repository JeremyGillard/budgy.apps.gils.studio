import { createRouter, createWebHistory } from "vue-router";
import TransactionsView from "../views/TransactionsView.vue";

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: "/",
      name: "transactions",
      component: TransactionsView,
    },
    {
      path: "/import",
      name: "import",
      component: () => import("../views/ImportView.vue"),
    },
  ],
});

export default router;
