const { invoke } = window.__TAURI__.core;

let startBtn, stopBtn, statusEl, verifyBtn, connectionStatus;

async function loadConfig() {
  try {
    const config = await invoke("get_config");
    document.getElementById("tally-url").value = config.tally_url;
    document.getElementById("server-port").value = config.server_port;
    document.getElementById("allow-list-url").value = config.allow_list_url;
  } catch (e) {
    console.error("Failed to load config:", e);
  }
}

async function updateStatus() {
  try {
    const status = await invoke("get_server_status");
    const isRunning = status !== "stopped";
    statusEl.textContent = isRunning ? `Server ${status}` : "Server stopped";
    statusEl.className = `status visible ${isRunning ? "running" : "stopped"}`;
    stopBtn.disabled = !isRunning;
    startBtn.disabled = isRunning;
  } catch (e) {
    console.error("Failed to get status:", e);
  }
}

async function startServer() {
  const tallyUrl = document.getElementById("tally-url").value.trim();
  const serverPortVal = parseInt(document.getElementById("server-port").value);
  const allowListUrl = document.getElementById("allow-list-url").value.trim();

  try {
    const result = await invoke("start_server", {
      config: {
        tally_url: tallyUrl,
        server_port: serverPortVal,
        allow_list_url: allowListUrl,
      },
    });
    statusEl.textContent = result;
    statusEl.className = "status visible running";
    stopBtn.disabled = false;
    startBtn.disabled = true;
  } catch (e) {
    statusEl.textContent = `Error: ${e}`;
    statusEl.className = "status visible error";
  }
}

async function stopServer() {
  try {
    const result = await invoke("stop_server");
    statusEl.textContent = result;
    statusEl.className = "status visible stopped";
    stopBtn.disabled = true;
    startBtn.disabled = false;
  } catch (e) {
    statusEl.textContent = `Error: ${e}`;
    statusEl.className = "status visible error";
  }
}

async function verifyTallyConnection() {
  const tallyUrl = document.getElementById("tally-url").value.trim();
  if (!tallyUrl) return;

  connectionStatus.textContent = "Verifying...";
  connectionStatus.className = "connection-status visible verifying";

  try {
    const result = await invoke("verify_tally_connection", { tallyUrl });
    if (result === "connected") {
      connectionStatus.textContent = "Tally connected";
      connectionStatus.className = "connection-status visible connected";
    } else {
      connectionStatus.textContent = "Tally unreachable";
      connectionStatus.className = "connection-status visible unreachable";
    }
  } catch (e) {
    connectionStatus.textContent = `Error: ${e}`;
    connectionStatus.className = "connection-status visible error";
  }
}

window.addEventListener("DOMContentLoaded", async () => {
  startBtn = document.getElementById("start-btn");
  stopBtn = document.getElementById("stop-btn");
  statusEl = document.getElementById("status");
  verifyBtn = document.getElementById("verify-btn");
  connectionStatus = document.getElementById("connection-status");

  await loadConfig();
  await updateStatus();

  document.getElementById("config-form").addEventListener("submit", (e) => {
    e.preventDefault();
    startServer();
  });

  stopBtn.addEventListener("click", stopServer);
  verifyBtn.addEventListener("click", verifyTallyConnection);
});
