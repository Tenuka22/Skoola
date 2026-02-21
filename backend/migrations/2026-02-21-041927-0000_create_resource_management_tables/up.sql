CREATE TABLE resources (
    id TEXT PRIMARY KEY NOT NULL,
    resource_name TEXT NOT NULL,
    resource_type TEXT NOT NULL CHECK (resource_type IN ('Equipment', 'Venue', 'Vehicle', 'Other')),
    description TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE resource_bookings (
    id TEXT PRIMARY KEY NOT NULL,
    resource_id TEXT NOT NULL,
    booked_by_user_id TEXT NOT NULL,
    start_time DATETIME NOT NULL,
    end_time DATETIME NOT NULL,
    related_event_id TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (resource_id) REFERENCES resources(id) ON DELETE CASCADE,
    FOREIGN KEY (booked_by_user_id) REFERENCES users(id) ON DELETE RESTRICT
);

CREATE INDEX idx_resources_name ON resources(resource_name);
CREATE INDEX idx_resource_bookings_resource_id ON resource_bookings(resource_id);
CREATE INDEX idx_resource_bookings_booked_by_user_id ON resource_bookings(booked_by_user_id);
CREATE INDEX idx_resource_bookings_start_time ON resource_bookings(start_time);
CREATE INDEX idx_resource_bookings_end_time ON resource_bookings(end_time);