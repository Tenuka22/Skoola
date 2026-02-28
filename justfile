set shell := ["bash", "-cu"]
set windows-shell := ["powershell", "-Command"]

dev-backend:
    clear && cd backend && cargo run --bin backend

run-backend:
    clear && cd backend && cargo run --bin backend --release

fetch-run-backend:
    clear && cd backend && bash build_and_fetch.sh && ./backend

cloud-run-backend:
    clear && cd backend && ./backend


dev-backend-hr:
    clear && cd backend && cargo watch -x run

check-backend:
    clear && cd backend && cargo check

check-web:
    clear && cd web && bun run tsc

lint-web:
    cd web && bun run check

dev-web:
    cd web && bun run dev

gen-api:
    cd web && bun run api

db-ui:
    cd web && bun run drizzle-kit studio

db-seed:
    cd backend && cargo run --bin seed

db-migrate:
    cd backend && diesel migration run
    cd backend && diesel print-schema > src/schema.rs
