set shell := ["powershell.exe", "-Command"]

db_user := "root"
db_pass := "secret"

dev-backend:
    cd backend; $env:RUST_LOG='info'; cargo watch -x run

run-server:
    cd backend; $env:RUST_LOG='info'; cargo run --release

check-backend:
    cd backend; cargo check

dev-web:
    cd web; bun run dev
