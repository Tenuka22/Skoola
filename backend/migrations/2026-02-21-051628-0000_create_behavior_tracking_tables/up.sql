CREATE TABLE behavior_incident_types (
    id TEXT PRIMARY KEY NOT NULL,
    type_name TEXT NOT NULL,
    default_points INTEGER NOT NULL,
    description TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE behavior_incidents (
    id TEXT PRIMARY KEY NOT NULL,
    student_id TEXT NOT NULL,
    reported_by_user_id TEXT NOT NULL,
    incident_type_id TEXT NOT NULL,
    description TEXT NOT NULL,
    incident_date DATETIME NOT NULL,
    points_awarded INTEGER NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (student_id) REFERENCES students(id) ON DELETE CASCADE,
    FOREIGN KEY (reported_by_user_id) REFERENCES users(id) ON DELETE RESTRICT,
    FOREIGN KEY (incident_type_id) REFERENCES behavior_incident_types(id) ON DELETE RESTRICT
);

CREATE INDEX idx_behavior_incident_types_name ON behavior_incident_types(type_name);
CREATE INDEX idx_behavior_incidents_student_id ON behavior_incidents(student_id);
CREATE INDEX idx_behavior_incidents_incident_type_id ON behavior_incidents(incident_type_id);
CREATE INDEX idx_behavior_incidents_incident_date ON behavior_incidents(incident_date);