PRAGMA foreign_keys=OFF;

CREATE TABLE terms_new (
    id TEXT NOT NULL PRIMARY KEY,
    academic_year_id TEXT NOT NULL,
    term_number INTEGER NOT NULL,
    name VARCHAR NOT NULL,
    start_date DATE NOT NULL,
    end_date DATE NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (academic_year_id) REFERENCES academic_years(id) ON DELETE CASCADE
);

INSERT INTO terms_new (id, academic_year_id, term_number, name, start_date, end_date, created_at, updated_at)
SELECT CAST(id AS TEXT), CAST(academic_year_id AS TEXT), term_number, name, start_date, end_date, created_at, updated_at FROM terms;

DROP TABLE terms;
ALTER TABLE terms_new RENAME TO terms;

PRAGMA foreign_keys=ON;
