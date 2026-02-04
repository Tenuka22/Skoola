CREATE TABLE terms (
    id TEXT NOT NULL PRIMARY KEY,
    academic_year_id TEXT NOT NULL,
    term_number INTEGER NOT NULL,
    name VARCHAR NOT NULL,
    start_date DATE NOT NULL,
    end_date DATE NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (academic_year_id) REFERENCES academic_years(id) ON DELETE CASCADE
);

SELECT diesel_manage_updated_at('terms');