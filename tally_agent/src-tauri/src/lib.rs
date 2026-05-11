mod allow_list;
mod config;
mod error;
mod server;
mod tally_client;

use std::sync::Mutex;
use tauri::{AppHandle, Manager};
use tokio::sync::oneshot;
use tracing::{error, info};
use tracing_subscriber::fmt::format::FmtSpan;

struct ServerHandle {
    shutdown_tx: Option<oneshot::Sender<()>>,
    port: u16,
}

#[tauri::command]
async fn start_server(
    app: AppHandle,
    config: config::AppConfig,
) -> Result<String, String> {
    config.save(&app)?;

    let server_handle = app.state::<Mutex<ServerHandle>>();

    {
        let mut handle = server_handle.inner().lock().map_err(|e| e.to_string())?;
        if handle.shutdown_tx.is_some() {
            return Err(format!("Server already running on port {}", handle.port));
        }

        let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();
        let port = config.server_port;

        let server_config = config.clone();
        tokio::spawn(async move {
            if let Err(e) = server::run_server(server_config, shutdown_rx).await {
                error!("Server error: {}", e);
            }
        });

        handle.shutdown_tx = Some(shutdown_tx);
        handle.port = port;

        info!("Server started on port {}", port);
    }

    Ok(format!("Server started on port {}", config.server_port))
}

#[tauri::command]
async fn stop_server(app: AppHandle) -> Result<String, String> {
    let server_handle = app.state::<Mutex<ServerHandle>>();
    let mut handle = server_handle.inner().lock().map_err(|e| e.to_string())?;
    if let Some(tx) = handle.shutdown_tx.take() {
        let _ = tx.send(());
        handle.port = 0;
        info!("Server stopped");
        return Ok("Server stopped".to_string());
    }
    Ok("Server was not running".to_string())
}

#[tauri::command]
async fn get_config(app: AppHandle) -> Result<config::AppConfig, String> {
    Ok(config::AppConfig::load(&app))
}

#[tauri::command]
async fn get_server_status(app: AppHandle) -> Result<String, String> {
    let server_handle = app.state::<Mutex<ServerHandle>>();
    let handle = server_handle.inner().lock().map_err(|e| e.to_string())?;
    if handle.shutdown_tx.is_some() {
        Ok(format!("running on port {}", handle.port))
    } else {
        Ok("stopped".to_string())
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .with_span_events(FmtSpan::CLOSE)
        .init();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .manage(Mutex::new(ServerHandle {
            shutdown_tx: None,
            port: 0,
        }))
        .invoke_handler(tauri::generate_handler![
            start_server,
            stop_server,
            get_config,
            get_server_status
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
