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
    Start-Process -FilePath "surreal" -ArgumentList "start", "--user", "{{db_user}}", "--pass", "{{db_pass}}" -NoNewWindow

stop-db:
    $p = Get-Process -Name "surreal"; if ($p) { Stop-Process -Id $p.Id -Force } else { Write-Host "SurrealDB process not found." }
