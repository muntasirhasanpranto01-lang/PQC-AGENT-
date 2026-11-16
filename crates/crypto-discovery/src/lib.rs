use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Algorithm {
    RSA,
    ECDSA,
    ECDH,
    AES,
    SHA2,
    MLKEM,
    MLDSA,
    SLHDSA,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CryptoAsset {
    pub id: Uuid,
    pub hostname: String,
    pub protocol: String,
    pub algorithm: Algorithm,
    pub key_size: Option<u32>,
    pub owner: String,
    pub quantum_exposure: String,
}

pub struct CryptoDiscoveryAgent;

impl CryptoDiscoveryAgent {
    pub fn analyze(asset: &CryptoAsset) -> String {
        match asset.algorithm {
            Algorithm::RSA | Algorithm::ECDSA | Algorithm::ECDH => {
                "Requires PQC migration assessment".to_string()
            }
            Algorithm::MLKEM | Algorithm::MLDSA | Algorithm::SLHDSA => {
                "PQC algorithm detected; verify configuration".to_string()
            }
            Algorithm::Unknown => "Unknown algorithm; manual investigation required".to_string(),
            _ => "Review algorithm and security policy".to_string(),
        }
    }
}
