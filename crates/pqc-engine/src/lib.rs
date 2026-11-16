use qryptara_crypto_discovery::CryptoAsset;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PqcRecommendation {
    MlKem768,
    MlDsa65,
    SlhDsa,
    HybridKeyExchange,
    ManualReview,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommendationResult {
    pub summary: String,
    pub recommendation: PqcRecommendation,
}

pub struct PqcEngine;

impl PqcEngine {
    pub fn recommend_for(asset: &CryptoAsset) -> RecommendationResult {
        match asset.protocol.as_str() {
            "TLS" => RecommendationResult {
                summary: "Prioritize ML-KEM for key establishment and verify certificate interoperability before rollout.".to_string(),
                recommendation: PqcRecommendation::MlKem768,
            },
            "SSH" => RecommendationResult {
                summary: "Assess hybrid PQC support and update SSH policy to allow quantum-safe key exchange.".to_string(),
                recommendation: PqcRecommendation::HybridKeyExchange,
            },
            _ => RecommendationResult {
                summary: "Review cryptographic inventory and run a formal migration assessment before production changes.".to_string(),
                recommendation: PqcRecommendation::ManualReview,
            },
        }
    }
}
