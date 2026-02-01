set shell := ["powershell.exe", "-Command"]


dev-backend:
    cd backend; cargo watch -x run
dev-web:
    cd web; bun run dev
