const { invoke } = window.__TAURI__.core;
import { open } from "@tauri-apps/plugin-dialog";

let currentYear = new Date().getFullYear();
let currentMonth = new Date().getMonth() + 1;

const monthNames = [
  "January", "February", "March", "April", "May", "June",
  "July", "August", "September", "October", "November", "December",
];

function formatAmount(cents) {
  const val = (cents / 100).toFixed(2);
  return val.replace(/\B(?=(\d{3})+(?!\d))/g, ",");
}

function renderMonth() {
  document.getElementById("current-month").textContent =
    `${monthNames[currentMonth - 1]} ${currentYear}`;
}

async function loadSummary() {
  try {
    const summary = await invoke("monthly_summary", {
      year: currentYear,
      month: currentMonth,
    });
    document.getElementById("income-amount").textContent =
      `+${formatAmount(summary.total_income_cents)} EUR`;
    document.getElementById("expenses-amount").textContent =
      `${formatAmount(summary.total_expenses_cents)} EUR`;
    document.getElementById("net-amount").textContent =
      `${formatAmount(summary.net_cents)} EUR`;
  } catch (e) {
    console.error("Failed to load summary:", e);
  }
}

async function loadTransactions() {
  try {
    const txs = await invoke("list_transactions_by_month", {
      year: currentYear,
      month: currentMonth,
    });

    document.getElementById("tx-count").textContent = `(${txs.length})`;

    const tbody = document.getElementById("transactions-body");
    tbody.innerHTML = "";

    for (const tx of txs) {
      const tr = document.createElement("tr");

      const dateTd = document.createElement("td");
      dateTd.textContent = tx.accounting_date;
      tr.appendChild(dateTd);

      const descTd = document.createElement("td");
      descTd.className = "description";
      descTd.textContent = tx.description;
      descTd.title = tx.description;
      tr.appendChild(descTd);

      const cpTd = document.createElement("td");
      cpTd.textContent = tx.communication || "";
      tr.appendChild(cpTd);

      const commTd = document.createElement("td");
      commTd.textContent = tx.communication || "";
      tr.appendChild(commTd);

      const amtTd = document.createElement("td");
      amtTd.className = `amount ${tx.amount_cents >= 0 ? "positive" : "negative"}`;
      amtTd.textContent = `${formatAmount(tx.amount_cents)} ${tx.currency}`;
      tr.appendChild(amtTd);

      tbody.appendChild(tr);
    }
  } catch (e) {
    console.error("Failed to load transactions:", e);
  }
}

async function loadAccounts() {
  try {
    const accounts = await invoke("list_accounts");
    const select = document.getElementById("account-select");
    // Keep first "All accounts" option
    while (select.children.length > 1) {
      select.removeChild(select.lastChild);
    }
    for (const acct of accounts) {
      const opt = document.createElement("option");
      opt.value = acct.id;
      opt.textContent = acct.label || acct.iban;
      select.appendChild(opt);
    }
  } catch (e) {
    console.error("Failed to load accounts:", e);
  }
}

async function refresh() {
  renderMonth();
  await Promise.all([loadSummary(), loadTransactions(), loadAccounts()]);
}

document.addEventListener("DOMContentLoaded", () => {
  // Import CSV button
  document.getElementById("import-btn").addEventListener("click", async () => {
    const resultDiv = document.getElementById("import-result");
    try {
      const filePath = await open({
        multiple: false,
        filters: [{ name: "CSV", extensions: ["csv"] }],
      });

      if (!filePath) return;

      const result = await invoke("import_csv", { filePath });

      resultDiv.hidden = false;
      resultDiv.className = "notice";
      resultDiv.textContent =
        `Imported ${result.imported} transactions from ${result.account_iban}` +
        (result.skipped_duplicates > 0
          ? ` (${result.skipped_duplicates} duplicates skipped)`
          : "") +
        (result.date_from
          ? ` | ${result.date_from} to ${result.date_to}`
          : "");

      // Auto-navigate to the most recent month of imported data
      if (result.date_to) {
        const [y, m] = result.date_to.split("-").map(Number);
        currentYear = y;
        currentMonth = m;
      }

      await refresh();
    } catch (e) {
      resultDiv.hidden = false;
      resultDiv.className = "notice error";
      resultDiv.textContent = `Import failed: ${e}`;
    }
  });

  // Month navigation
  document.getElementById("prev-month").addEventListener("click", () => {
    currentMonth--;
    if (currentMonth < 1) {
      currentMonth = 12;
      currentYear--;
    }
    refresh();
  });

  document.getElementById("next-month").addEventListener("click", () => {
    currentMonth++;
    if (currentMonth > 12) {
      currentMonth = 1;
      currentYear++;
    }
    refresh();
  });

  refresh();
});
