set shell := ["bash", "-cu"]
set windows-shell := ["powershell", "-Command"]

dev-backend:
    cd backend; cargo run --bin backend

run-backend:
    cd backend; cargo run --bin backend --release

dev-backend-hr:
    cd backend; cargo watch -x run

check-backend:
    cd backend; cargo check

check-web:
    cd web; bun run tsc

dev-web:
    cd web; bun run dev

gen-api:
    cd web; bun run api

db-ui:
    cd web; bun run drizzle-kit studio

db-seed:
    cd backend; cargo run --bin seed

db-migrate:
    cd backend; diesel migration run
    cd backend; diesel print-schema > src/schema.rs
