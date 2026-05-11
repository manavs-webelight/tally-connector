use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub tally_url: String,
    pub server_port: u16,
    pub allow_list_url: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            tally_url: "http://localhost:9000".to_string(),
            server_port: 8004,
            allow_list_url: "http://localhost:8005".to_string(),
        }
    }
}

impl AppConfig {
    pub fn load(store: &tauri_plugin_store::Store<tauri_plugin_store::WryAppHandle>) -> Self {
        store
            .get("config")
            .and_then(|v| serde_json::from_value(v.clone()).ok())
            .unwrap_or_default()
    }

    pub fn save(
        &self,
        store: &tauri_plugin_store::Store<tauri_plugin_store::WryAppHandle>,
    ) -> Result<(), String> {
        let value = serde_json::to_value(self).map_err(|e| e.to_string())?;
        store
            .set("config", value)
            .map_err(|e| e.to_string())?;
        store
            .save()
            .map_err(|e| format!("failed to save store: {}", e))?;
        Ok(())
    }
}
