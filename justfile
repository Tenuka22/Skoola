set shell := ["bash", "-cu"]
set windows-shell := ["powershell", "-Command"]

dev-backend:
    cd backend; cargo run

run-backend:
    cd backend; cargo run --release

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
