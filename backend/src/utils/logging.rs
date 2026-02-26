use tracing::{error, info, warn};

pub fn log_auth_success(user_id: &str, method: &str) {
    info!(
        event = "auth_success",
        user_id = %user_id,
        method = %method,
        "User successfully authenticated"
    );
}

pub fn log_auth_failure(identifier: &str, reason: &str) {
    warn!(
        event = "auth_failure",
        identifier = %identifier,
        reason = %reason,
        "Authentication attempt failed"
    );
}

pub fn log_session_created(user_id: &str, session_id: &str) {
    info!(
        event = "session_created",
        user_id = %user_id,
        session_id = %session_id,
        "New session established"
    );
}

pub fn log_session_revoked(session_id: &str, reason: &str) {
    info!(
        event = "session_revoked",
        session_id = %session_id,
        reason = %reason,
        "Session revoked"
    );
}

pub fn log_permission_denied(user_id: &str, resource: &str, permission: &str) {
    warn!(
        event = "permission_denied",
        user_id = %user_id,
        resource = %resource,
        permission = %permission,
        "Access denied to resource"
    );
}

pub fn log_iam_error(event: &str, error: &dyn std::error::Error) {
    error!(
        event = %event,
        error = %error,
        "Internal IAM error occurred"
    );
}
