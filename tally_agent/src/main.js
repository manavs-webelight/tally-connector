const { invoke } = window.__TAURI__.core;

let startBtn, stopBtn, statusEl, connectionStatus;
let tallyUrlInput, serverPortInput, allowListUrlInput;
let isServerRunning = false;

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
    isServerRunning = status !== "stopped";
    statusEl.textContent = isServerRunning ? `Server ${status}` : "Server stopped";
    statusEl.className = `status visible ${isRunning ? "running" : "stopped"}`;
    stopBtn.disabled = !isRunning;
    startBtn.disabled = isRunning;
    setFieldsDisabled(isRunning);
  } catch (e) {
    console.error("Failed to get status:", e);
  }
}

function validateForm() {
  const tallyUrl = tallyUrlInput.value.trim();
  const serverPort = serverPortInput.value.trim();
  const allowListUrl = allowListUrlInput.value.trim();

  const allFilled = tallyUrl && serverPort && allowListUrl;
  startBtn.disabled = !allFilled || isServerRunning;
}

function setFieldsDisabled(disabled) {
  tallyUrlInput.disabled = disabled;
  serverPortInput.disabled = disabled;
  allowListUrlInput.disabled = disabled;
}

async function verifyTallyConnection() {
  const tallyUrl = tallyUrlInput.value.trim();
  if (!tallyUrl) return false;

  connectionStatus.textContent = "Verifying Tally connection...";
  connectionStatus.className = "connection-status visible verifying";

  try {
    const result = await invoke("verify_tally_connection", { tallyUrl });
    if (result === "connected") {
      connectionStatus.textContent = "Tally connected";
      connectionStatus.className = "connection-status visible connected";
      return true;
    } else {
      connectionStatus.textContent = "Tally unreachable";
      connectionStatus.className = "connection-status visible unreachable";
      return false;
    }
  } catch (e) {
    connectionStatus.textContent = `Error: ${e}`;
    connectionStatus.className = "connection-status visible error";
    return false;
  }
}

async function startServer() {
  const tallyUrl = tallyUrlInput.value.trim();
  const serverPortVal = parseInt(serverPortInput.value);
  const allowListUrl = allowListUrlInput.value.trim();

  statusEl.textContent = "Verifying Tally connection before starting...";
  statusEl.className = "status visible verifying";

  const isVerified = await verifyTallyConnection();
  if (!isVerified) {
    statusEl.textContent = "Cannot start server: Tally is not connected";
    statusEl.className = "status visible error";
    return;
  }

  try {
    const result = await invoke("start_server", {
      config: {
        tally_url: tallyUrl,
        server_port: serverPortVal,
        allow_list_url: allowListUrl,
      },
    });
    isServerRunning = true;
    statusEl.textContent = result;
    statusEl.className = "status visible running";
    stopBtn.disabled = false;
    startBtn.disabled = true;
    setFieldsDisabled(true);
  } catch (e) {
    statusEl.textContent = `Error: ${e}`;
    statusEl.className = "status visible error";
  }
}

async function stopServer() {
  try {
    const result = await invoke("stop_server");
    isServerRunning = false;
    statusEl.textContent = result;
    statusEl.className = "status visible stopped";
    stopBtn.disabled = true;
    startBtn.disabled = false;
    setFieldsDisabled(false);
  } catch (e) {
    statusEl.textContent = `Error: ${e}`;
    statusEl.className = "status visible error";
  }
}

window.addEventListener("DOMContentLoaded", async () => {
  startBtn = document.getElementById("start-btn");
  stopBtn = document.getElementById("stop-btn");
  statusEl = document.getElementById("status");
  connectionStatus = document.getElementById("connection-status");

  tallyUrlInput = document.getElementById("tally-url");
  serverPortInput = document.getElementById("server-port");
  allowListUrlInput = document.getElementById("allow-list-url");

  await loadConfig();
  await updateStatus();

  tallyUrlInput.addEventListener("input", validateForm);
  serverPortInput.addEventListener("input", validateForm);
  allowListUrlInput.addEventListener("input", validateForm);

  document.getElementById("config-form").addEventListener("submit", (e) => {
    e.preventDefault();
    startServer();
  });

  stopBtn.addEventListener("click", stopServer);

  validateForm();
});
