use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEvent {
    pub event_type: String,
    pub actor: String,
    pub details: String,
}

pub fn log_event(event: &AuditEvent) -> String {
    format!(
        "AUDIT: event_type={}, actor={}, details={}",
        event.event_type, event.actor, event.details
    )
}
