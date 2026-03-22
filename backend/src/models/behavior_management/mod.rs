pub mod behavior_incident;
pub mod behavior_incident_actions;
pub mod behavior_incident_details;
pub mod behavior_incident_evidence;
pub mod behavior_incident_followups;
pub mod behavior_incident_participants;
pub mod behavior_incident_severity_levels;
pub mod behavior_incident_type;
pub mod behavior_incident_types;
pub mod detention;

pub mod incident_details;

pub use detention::*;

// Re-export request types from incident_details
pub use incident_details::{
    CreateBehaviorIncidentTypeRequest, UpdateBehaviorIncidentTypeRequest,
    RecordBehaviorIncidentRequest, UpdateBehaviorIncidentRequest,
    CreateBehaviorIncidentSeverityLevelRequest, UpdateBehaviorIncidentSeverityLevelRequest,
    CreateBehaviorIncidentActionRequest, UpdateBehaviorIncidentActionRequest,
    CreateBehaviorIncidentEvidenceRequest, UpdateBehaviorIncidentEvidenceRequest,
    CreateBehaviorIncidentFollowupRequest, UpdateBehaviorIncidentFollowupRequest,
    CreateBehaviorIncidentDetailsRequest, UpdateBehaviorIncidentDetailsRequest,
};

// Re-export model types
pub use incident_details::{
    BehaviorIncidentSeverityLevel, BehaviorIncidentAction,
    BehaviorIncidentEvidence, BehaviorIncidentFollowup,
    BehaviorIncidentDetail,
};
pub use behavior_incident::{BehaviorIncident, NewBehaviorIncidentDetail};
pub use behavior_incident_types::BehaviorIncidentType;

