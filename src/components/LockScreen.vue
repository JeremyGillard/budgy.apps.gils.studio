<script setup>
import { ref, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";
import Password from "primevue/password";
import Button from "primevue/button";

const props = defineProps({
  dbStatus: { type: String, required: true },
});

const emit = defineEmits(["unlocked"]);

const password = ref("");
const confirmPassword = ref("");
const error = ref("");
const loading = ref(false);
const showWarning = ref(false);

const isSetup = computed(
  () => props.dbStatus === "FirstTime" || props.dbStatus === "Unencrypted",
);

const canSubmit = computed(() => {
  if (!password.value || password.value.length < 8) return false;
  if (isSetup.value && password.value !== confirmPassword.value) return false;
  return true;
});

async function submit() {
  error.value = "";
  loading.value = true;
  try {
    if (isSetup.value) {
      await invoke("setup_encryption", { password: password.value });
      showWarning.value = true;
      return;
    } else {
      await invoke("unlock_db", { password: password.value });
    }
    emit("unlocked");
  } catch (e) {
    error.value = String(e);
  } finally {
    loading.value = false;
  }
}
</script>

<template>
  <div class="lock-screen">
    <div v-if="showWarning" class="lock-card">
      <span class="warning-icon">&#9888;</span>
      <h1 class="lock-title">Important</h1>
      <p class="warning-message">
        Your password cannot be recovered. If you forget it, your data will be
        permanently lost.
      </p>
      <Button
        label="I understand, continue"
        @click="emit('unlocked')"
        class="lock-button"
      />
    </div>

    <div v-else class="lock-card">
      <img src="/icon.png" alt="Budgy" class="lock-icon" />
      <h1 class="lock-title">Budgy</h1>
      <p v-if="isSetup" class="lock-subtitle">
        Set a password to encrypt your data
      </p>
      <p v-else class="lock-subtitle">Enter your password to unlock</p>

      <form @submit.prevent="submit" class="lock-form">
        <div class="field">
          <Password
            v-model="password"
            :feedback="false"
            toggleMask
            placeholder="Password"
            inputClass="lock-input"
            @keyup.enter="!isSetup && canSubmit && submit()"
          />
        </div>

        <div v-if="isSetup" class="field">
          <Password
            v-model="confirmPassword"
            :feedback="false"
            toggleMask
            placeholder="Confirm password"
            inputClass="lock-input"
            @keyup.enter="canSubmit && submit()"
          />
          <small v-if="password.length > 0 && password.length < 8" class="hint">
            Minimum 8 characters
          </small>
          <small
            v-if="
              confirmPassword.length > 0 && password !== confirmPassword
            "
            class="hint error-text"
          >
            Passwords do not match
          </small>
        </div>

        <small v-if="error" class="error-text">{{ error }}</small>

        <Button
          :label="isSetup ? 'Set password & unlock' : 'Unlock'"
          type="submit"
          :disabled="!canSubmit"
          :loading="loading"
          class="lock-button"
        />
      </form>
    </div>
  </div>
</template>

<style scoped>
.lock-screen {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 100vh;
  background: var(--p-surface-ground);
}

.lock-card {
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 2.5rem;
  background: var(--p-content-background);
  border: 1px solid var(--p-content-border-color);
  border-radius: 12px;
  box-shadow: 0 4px 24px rgba(0, 0, 0, 0.12);
  min-width: 340px;
}

.lock-icon {
  width: 64px;
  height: 64px;
  margin-bottom: 0.75rem;
}

.lock-title {
  margin: 0 0 0.25rem;
  font-size: 1.5rem;
  font-weight: 700;
  color: var(--p-text-color);
}

.lock-subtitle {
  margin: 0 0 1.5rem;
  color: var(--p-text-muted-color);
  font-size: 0.875rem;
}

.lock-form {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
  width: 100%;
}

.field {
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
}

:deep(.lock-input) {
  width: 100%;
}

:deep(.p-password) {
  width: 100%;
}

.hint {
  color: var(--p-text-muted-color);
  font-size: 0.75rem;
}

.error-text {
  color: var(--p-red-500);
  font-size: 0.8rem;
}

.lock-button {
  margin-top: 0.5rem;
  width: 100%;
}

.warning-icon {
  font-size: 2.5rem;
  margin-bottom: 0.5rem;
  color: var(--p-yellow-500);
}

.warning-message {
  margin: 0 0 1.5rem;
  color: var(--p-text-color);
  font-size: 0.9rem;
  text-align: center;
  line-height: 1.5;
}
</style>
