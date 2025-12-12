<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { useI18n } from 'vue-i18n';

const { t, locale } = useI18n();

const userId = ref("");
const password = ref("");
const status = ref("disconnected");
const logs = ref<string[]>([]);
const isConnecting = ref(false);

const toggleLocale = () => {
  locale.value = locale.value === 'zh' ? 'en' : 'zh';
};

const addLog = (msgKey: string, params: Record<string, any> = {}) => {
  const time = new Date().toLocaleTimeString();
  const msg = t(msgKey, params);
  logs.value.unshift(`[${time}] ${msg}`);
};

async function startVPN() {
  if (!userId.value || !password.value) {
    addLog("errors.missingCredentials");
    return;
  }

  isConnecting.value = true;
  addLog("logs.starting");

  try {
    // 1. Generate Keys (or load)
    addLog("logs.checkingKeys");
    const [privKey, pubKey] = await invoke<[string, string]>("generate_keys");
    addLog("logs.keysGenerated", { key: pubKey.substring(0, 10) });

    // 2. Authenticate
    addLog("logs.authenticating");
    // Mock device code
    const deviceCode = "DEV-" + Math.random().toString(36).substring(7).toUpperCase();
    
    const token = await invoke<string>("authenticate", {
      userId: userId.value,
      password: password.value,
      deviceCode: deviceCode,
      wgPublicKey: pubKey
    });
    addLog("logs.authSuccess");

    // 3. Connect
    addLog("logs.initiating");
    const res = await invoke<string>("start_connection", { token });
    status.value = "connected"; // Simplified for i18n demo
    addLog("logs.success", { msg: res });

  } catch (error) {
    status.value = "failed";
    addLog("errors.connectionFailed");
    // Also log raw error for debug
    const time = new Date().toLocaleTimeString();
    logs.value.unshift(`[${time}] RAW ERROR: ${error}`);
  } finally {
    isConnecting.value = false;
  }
}
</script>

<template>
  <main class="container">
    <div class="header">
      <h1>{{ t('title') }}</h1>
      <button class="lang-btn" @click="toggleLocale">
        {{ locale === 'zh' ? 'English' : '中文' }}
      </button>
    </div>

    <div class="login-box">
      <div class="input-group">
        <label>{{ t('userId') }}</label>
        <input v-model="userId" :placeholder="t('enterUserId')" :disabled="isConnecting" />
      </div>
      <div class="input-group">
        <label>{{ t('password') }}</label>
        <input v-model="password" type="password" :placeholder="t('enterPassword')" :disabled="isConnecting" />
      </div>
      
      <button class="connect-btn" @click="startVPN" :disabled="isConnecting">
        {{ isConnecting ? t('connecting') : t('connect') }}
      </button>
    </div>

    <div class="status-box" :class="status">
      <h3>{{ t('status') }}: {{ t(status) }}</h3>
    </div>

    <div class="logs-box">
      <h4>{{ t('activityLog') }}</h4>
      <div class="logs">
        <div v-for="(log, index) in logs" :key="index" class="log-entry">{{ log }}</div>
      </div>
    </div>
  </main>
</template>

<style scoped>
.container {
  max-width: 600px;
  margin: 0 auto;
  padding: 2rem;
  font-family: 'Segoe UI', sans-serif;
}

.header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 2rem;
}

h1 {
  margin: 0;
  font-size: 1.5rem;
}

.lang-btn {
  padding: 0.4rem 0.8rem;
  font-size: 0.9rem;
  background: transparent;
  border: 1px solid #ccc;
  color: inherit;
}

.login-box {
  display: flex;
  flex-direction: column;
  gap: 1rem;
  margin-bottom: 2rem;
  background: #f5f5f5;
  padding: 1.5rem;
  border-radius: 8px;
}

.input-group {
  display: flex;
  flex-direction: column;
  text-align: left;
}

.input-group label {
  font-size: 0.9rem;
  margin-bottom: 0.5rem;
  color: #666;
}

input {
  padding: 0.8rem;
  border: 1px solid #ddd;
  border-radius: 4px;
}

.connect-btn {
  padding: 1rem;
  background-color: #007bff;
  color: white;
  border: none;
  border-radius: 4px;
  font-weight: bold;
  cursor: pointer;
  transition: background 0.3s;
}

.connect-btn:hover:not(:disabled) {
  background-color: #0056b3;
}

.connect-btn:disabled {
  background-color: #ccc;
  cursor: not-allowed;
}

.status-box {
  padding: 1rem;
  border-radius: 8px;
  background: #eee;
  margin-bottom: 1rem;
  text-align: center;
}

.status-box.connected {
  background: #d4edda;
  color: #155724;
}

.status-box.failed {
  background: #f8d7da;
  color: #721c24;
}

.logs-box {
  border: 1px solid #ddd;
  border-radius: 8px;
  padding: 1rem;
  text-align: left;
}

.logs {
  height: 200px;
  overflow-y: auto;
  font-family: monospace;
  font-size: 0.9rem;
  background: #1e1e1e;
  color: #0f0;
  padding: 0.5rem;
  border-radius: 4px;
}

.log-entry {
  margin-bottom: 4px;
}

@media (prefers-color-scheme: dark) {
  .container {
    color: #fff;
  }
  .login-box {
    background: #333;
  }
  .input-group label {
    color: #aaa;
  }
  input {
    background: #444;
    color: #fff;
    border-color: #555;
  }
  .status-box {
    background: #333;
  }
  .logs-box {
    border-color: #555;
  }
  .lang-btn {
    border-color: #555;
  }
}
</style>
