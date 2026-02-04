set shell := ["bash", "-cu"]
set windows-shell := ["powershell", "-Command"]

dev-backend:
    cd backend; $env:RUST_LOG="info"; cargo run

run-backend:
    cd backend; $env:RUST_LOG="info"; cargo run --release

dev-backend-hr:
    cd backend; $env:RUST_LOG="info"; cargo watch -x run

check-backend:
    cd backend; cargo check

dev-web:
    cd web; bun run dev
