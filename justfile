set shell := ["bash", "-cu"]
set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]

db_user := "root"
db_pass := "secret"

dev-backend-hr:
    cd backend; $env:RUST_LOG='info'; cargo watch -x run

dev-backend:
    cd backend; $env:RUST_LOG='info'; cargo run

run-backend:
    cd backend; $env:RUST_LOG='info'; cargo run --release

check-backend:
    cd backend; cargo check

dev-web:
    cd web; bun run dev
