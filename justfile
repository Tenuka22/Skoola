set shell := ["powershell.exe", "-Command"]

db_user := "root"
db_pass := "secret"

dev-backend:
    cd backend; cargo watch -x run

check-backend:
    cd backend; cargo check

dev-web:
    cd web; bun run dev

start-db:
    surreal start --user {{db_user}} --pass {{db_pass}}
