use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyDecision {
    pub allowed: bool,
    pub reason: String,
}

pub fn evaluate_policy(action: &str, approved: bool) -> PolicyDecision {
    if !approved {
        PolicyDecision {
            allowed: false,
            reason: format!("Action '{action}' requires explicit approval before execution."),
        }
    } else {
        PolicyDecision {
            allowed: true,
            reason: format!("Action '{action}' approved under the current policy model."),
        }
    }
}
