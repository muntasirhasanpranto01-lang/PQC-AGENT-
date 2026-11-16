use axum::{
    extract::DefaultBodyLimit,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use qryptara_agent_core::{execute, AgentAction, AgentRequest};
use qryptara_crypto_discovery::{Algorithm, CryptoAsset, CryptoDiscoveryAgent};
use qryptara_pqc_engine::{PqcEngine, PqcRecommendation};
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
    service: &'static str,
    version: &'static str,
}

#[derive(Serialize)]
struct ScanResponse {
    hostname: String,
    protocol: String,
    assessment: String,
    recommendation: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/scan", post(scan_asset))
        .route("/agent/execute", post(run_agent_action))
        .layer(DefaultBodyLimit::max(1024 * 1024));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .expect("failed to bind TCP listener");

    tracing::info!("QRYPTARA API listening on 0.0.0.0:8080");
    axum::serve(listener, app).await.expect("server failed");
}

async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        service: "qryptara-api",
        version: "0.1.0",
    })
}

async fn scan_asset() -> Result<Json<ScanResponse>, StatusCode> {
    let asset = CryptoAsset {
        id: Uuid::new_v4(),
        hostname: "bank-server-01".to_string(),
        protocol: "TLS".to_string(),
        algorithm: Algorithm::RSA,
        key_size: Some(2048),
        owner: "IT Security".to_string(),
        quantum_exposure: "Unassessed".to_string(),
    };

    let assessment = CryptoDiscoveryAgent::analyze(&asset);
    let recommendation = PqcEngine::recommend_for(&asset);

    Ok(Json(ScanResponse {
        hostname: asset.hostname,
        protocol: asset.protocol,
        assessment,
        recommendation: recommendation.summary,
    }))
}

async fn run_agent_action(Json(request): Json<AgentRequest>) -> Result<String, StatusCode> {
    match execute(request).await {
        Ok(message) => Ok(message),
        Err(err) => {
            tracing::error!("agent execution failed: {err}");
            Err(StatusCode::BAD_REQUEST)
        }
    }
}
