<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

const userId = ref("");
const password = ref("");
const status = ref("Disconnected");
const logs = ref<string[]>([]);
const isConnecting = ref(false);

const addLog = (msg: string) => {
  const time = new Date().toLocaleTimeString();
  logs.value.unshift(`[${time}] ${msg}`);
};

async function startVPN() {
  if (!userId.value || !password.value) {
    addLog("Error: Please enter User ID and Password");
    return;
  }

  isConnecting.value = true;
  addLog("Starting VPN connection...");

  try {
    // 1. Generate Keys (or load)
    addLog("Checking WireGuard keys...");
    const [privKey, pubKey] = await invoke<[string, string]>("generate_keys");
    addLog(`Keys generated. Public Key: ${pubKey.substring(0, 10)}...`);

    // 2. Authenticate
    addLog("Authenticating...");
    // Mock device code
    const deviceCode = "DEV-" + Math.random().toString(36).substring(7).toUpperCase();
    
    const token = await invoke<string>("authenticate", {
      userId: userId.value,
      password: password.value,
      deviceCode: deviceCode,
      wgPublicKey: pubKey
    });
    addLog("Authentication successful. Token obtained.");

    // 3. Connect
    addLog("Initiating connection sequence...");
    const res = await invoke<string>("start_connection", { token });
    status.value = res;
    addLog(`Success: ${res}`);

  } catch (error) {
    status.value = "Connection Failed";
    addLog(`Error: ${error}`);
  } finally {
    isConnecting.value = false;
  }
}
</script>

<template>
  <main class="container">
    <h1>Internal Network VPN</h1>

    <div class="login-box">
      <div class="input-group">
        <label>User ID</label>
        <input v-model="userId" placeholder="Enter User ID" :disabled="isConnecting" />
      </div>
      <div class="input-group">
        <label>Password</label>
        <input v-model="password" type="password" placeholder="Enter Password" :disabled="isConnecting" />
      </div>
      
      <button @click="startVPN" :disabled="isConnecting">
        {{ isConnecting ? 'Connecting...' : 'Connect VPN' }}
      </button>
    </div>

    <div class="status-box" :class="{ connected: status.includes('Connected'), error: status.includes('Failed') }">
      <h3>Status: {{ status }}</h3>
    </div>

    <div class="logs-box">
      <h4>Activity Log</h4>
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

button {
  padding: 1rem;
  background-color: #007bff;
  color: white;
  border: none;
  border-radius: 4px;
  font-weight: bold;
  cursor: pointer;
  transition: background 0.3s;
}

button:hover:not(:disabled) {
  background-color: #0056b3;
}

button:disabled {
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

.status-box.error {
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
}
</style>
