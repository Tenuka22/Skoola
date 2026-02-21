CREATE TABLE audit_log (
    id TEXT PRIMARY KEY NOT NULL,
    user_id TEXT NOT NULL,
    action_type TEXT NOT NULL CHECK (action_type IN ('CREATE', 'UPDATE', 'DELETE', 'LOGIN', 'LOGOUT', 'OTHER')),
    table_name TEXT NOT NULL,
    record_pk TEXT NOT NULL,
    old_value_json TEXT,
    new_value_json TEXT,
    timestamp DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE RESTRICT
);

CREATE INDEX idx_audit_log_user_id ON audit_log(user_id);
CREATE INDEX idx_audit_log_table_name ON audit_log(table_name);
CREATE INDEX idx_audit_log_record_pk ON audit_log(record_pk);
CREATE INDEX idx_audit_log_timestamp ON audit_log(timestamp);