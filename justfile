set shell := ["bash", "-cu"]
set windows-shell := ["powershell", "-Command"]

# ---------------- BACKEND ----------------

dev-backend:
    cd backend; cargo run --bin backend

run-backend:
    cd backend; cargo run --bin backend --release

fetch-run-backend:
    cd backend; bash build_and_fetch.sh; ./backend

cloud-run-backend:
    cd backend; ./backend

dev-backend-hr:
    cd backend; cargo watch -x "run --bin backend"

check-backend:
    cd backend; cargo check

db-seed:
    cd backend; cargo run --bin seed

db-migrate:
    cd backend; diesel migration run; diesel print-schema > src/schema.rs


# ---------------- WEB ----------------

check-web:
    cd web; bun run tsc

lint-web:
    cd web; bun run check

dev-web:
    cd web; bun run dev

gen-api:
    cd web; bun run api

db-ui:
    cd web; bun run drizzle-kit studio
