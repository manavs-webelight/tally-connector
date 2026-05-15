use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use tokio::sync::oneshot;
use tracing::{error, info, warn};

use crate::allow_list::AllowListService;
use crate::config::AppConfig;
use crate::tally_client::TallyClient;

#[derive(Debug, Deserialize)]
pub struct TallyCallRequest {
    pub request_type: String,
    pub xml_body: String,
}

#[derive(Debug, Serialize)]
pub struct TallyCallResponse {
    pub success: bool,
    pub tally_response: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub service: String,
}

#[derive(Clone)]
pub struct ServerState {
    pub tally_client: TallyClient,
    pub allow_list: AllowListService,
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
        service: "Squirrel IntelliTools Tally Agent".to_string(),
    })
}

async fn call_tally(
    State(ServerState {
        tally_client,
        allow_list,
    }): State<ServerState>,
    Json(req): Json<TallyCallRequest>,
) -> (StatusCode, Json<TallyCallResponse>) {
    info!(
        "Received request_type={}, xml_body length={}",
        req.request_type,
        req.xml_body.len()
    );

    if !allow_list.is_request_allowed(&req.request_type).await {
        warn!("Request type '{}' not in allow list", req.request_type);
        return (
            StatusCode::FORBIDDEN,
            Json(TallyCallResponse {
                success: false,
                tally_response: None,
                error: Some(format!("Request type '{}' is not allowed", req.request_type)),
            }),
        );
    }

    match tally_client.post_xml(&req.xml_body, &req.request_type).await {
        Ok(response) => (
            StatusCode::OK,
            Json(TallyCallResponse {
                success: true,
                tally_response: Some(response),
                error: None,
            }),
        ),
        Err(e) => {
            error!("Tally call failed: {}", e);
            (
                StatusCode::BAD_GATEWAY,
                Json(TallyCallResponse {
                    success: false,
                    tally_response: None,
                    error: Some("An error occured while sending request to tally, please check if tally is running".to_string()),
                }),
            )
        }
    }
}

pub async fn run_server(
    config: AppConfig,
    shutdown: oneshot::Receiver<()>,
) -> Result<(), String> {
    let tally_client = TallyClient::new(config.tally_url.clone(), 30);
    let allow_list = AllowListService::new(config.allow_list_url.clone());

    let state = ServerState { tally_client, allow_list };

    let app = Router::new()
        .route("/health", get(health))
        .route("/call", post(call_tally))
        .with_state(state);

    let addr_string = format!("0.0.0.0:{}", config.server_port);
    let addr: std::net::SocketAddr = addr_string
        .parse()
        .map_err(|e| format!("invalid address: {}", e))?;

    info!("Starting server on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .map_err(|e| format!("failed to bind TCP listener: {}", e))?;

    axum::serve(listener, app.into_make_service())
        .with_graceful_shutdown(async {
            shutdown.await.ok();
        })
        .await
        .map_err(|e| format!("server error: {}", e))
}
