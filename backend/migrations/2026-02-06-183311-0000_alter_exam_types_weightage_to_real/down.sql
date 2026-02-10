-- Create a temporary table with the old schema
CREATE TABLE exam_types_old (
    id VARCHAR(36) PRIMARY KEY NOT NULL,
    name VARCHAR(100) NOT NULL,
    description TEXT,
    weightage INTEGER NOT NULL DEFAULT 0, -- Reverted to INTEGER
    created_at DATETIME NOT NULL,
    updated_at DATETIME NOT NULL
);

-- Copy data from the current table to the temporary table, casting REAL back to INTEGER
INSERT INTO exam_types_old (id, name, description, weightage, created_at, updated_at)
SELECT id, name, description, CAST(weightage AS INTEGER), created_at, updated_at
FROM exam_types;

-- Drop the current table
DROP TABLE exam_types;

-- Rename the temporary table to the original table's name
ALTER TABLE exam_types_old RENAME TO exam_types;