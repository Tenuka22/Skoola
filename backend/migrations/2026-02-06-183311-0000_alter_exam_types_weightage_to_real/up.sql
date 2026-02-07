-- Create a new table with the desired schema
CREATE TABLE exam_types_new (
    id VARCHAR(36) PRIMARY KEY NOT NULL,
    name VARCHAR(100) NOT NULL,
    description TEXT,
    weightage REAL NOT NULL DEFAULT 0.0, -- Changed to REAL
    created_at DATETIME NOT NULL,
    updated_at DATETIME NOT NULL
);

-- Copy data from the old table to the new table
INSERT INTO exam_types_new (id, name, description, weightage, created_at, updated_at)
SELECT id, name, description, CAST(weightage AS REAL), created_at, updated_at
FROM exam_types;

-- Drop the old table
DROP TABLE exam_types;

-- Rename the new table to the old table's name
ALTER TABLE exam_types_new RENAME TO exam_types;