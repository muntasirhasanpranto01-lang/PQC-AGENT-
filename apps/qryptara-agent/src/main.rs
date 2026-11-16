use qryptara_agent_core::{execute, AgentAction, AgentRequest};
use qryptara_crypto_discovery::{Algorithm, CryptoAsset};
use qryptara_pqc_engine::PqcEngine;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let asset = CryptoAsset {
        id: Uuid::new_v4(),
        hostname: "bank-server-01".to_string(),
        protocol: "TLS".to_string(),
        algorithm: Algorithm::RSA,
        key_size: Some(2048),
        owner: "IT Security".to_string(),
        quantum_exposure: "Unassessed".to_string(),
    };

    let recommendation = PqcEngine::recommend_for(&asset);
    println!("QRYPTARA agent: {}", recommendation.summary);

    let request = AgentRequest {
        action: AgentAction::AnalyzeQuantumRisk,
        target_id: asset.id.to_string(),
        approved: true,
    };

    match execute(request).await {
        Ok(result) => println!("{result}"),
        Err(err) => eprintln!("Agent error: {err}"),
    }
}
