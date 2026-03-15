param(
    [switch]$FailOnMissing
)

$ErrorActionPreference = "Stop"

$schemaPath = Join-Path $PSScriptRoot "..\\src\\schema.rs"
if (-not (Test-Path $schemaPath)) {
    Write-Error "schema.rs not found at: $schemaPath"
}

$schema = Get-Content $schemaPath -Raw

# Extract table names from `diesel::table! { <table_name> ( ... ) { ... } }`
$tableMatches = [regex]::Matches(
    $schema,
    "diesel::table!\s*\{\s*\r?\n\s*([A-Za-z0-9_]+)\s*\("
)

$tables = $tableMatches | ForEach-Object { $_.Groups[1].Value } | Select-Object -Unique | Sort-Object

# Extract tables referenced by admin CRUD service macros.
$srcRoot = Join-Path $PSScriptRoot "..\\src"
$rsFiles =
    Get-ChildItem $srcRoot -Recurse -Filter *.rs |
    Where-Object { $_.FullName -notlike "*\\schema.rs" }

$code = ($rsFiles | ForEach-Object { Get-Content $_.FullName -Raw }) -join "`n"

$serviceTables = @()
$serviceTables += [regex]::Matches($code, "impl_admin_entity_service!\s*\(\s*[A-Za-z0-9_]+\s*,\s*(?:[A-Za-z0-9_]+::)*([A-Za-z0-9_]+)::table") |
    ForEach-Object { $_.Groups[1].Value }
$serviceTables += [regex]::Matches($code, "impl_admin_entity_service_i32!\s*\(\s*[A-Za-z0-9_]+\s*,\s*(?:[A-Za-z0-9_]+::)*([A-Za-z0-9_]+)::table") |
    ForEach-Object { $_.Groups[1].Value }
$serviceTables += [regex]::Matches($code, "impl_admin_entity_service_id!\s*\(\s*[A-Za-z0-9_]+\s*,\s*(?:[A-Za-z0-9_]+::)*([A-Za-z0-9_]+)::table") |
    ForEach-Object { $_.Groups[1].Value }

$serviceTables = $serviceTables | Select-Object -Unique | Sort-Object

$missing = $tables | Where-Object { $_ -notin $serviceTables }

Write-Host "tables=$($tables.Count) admin_service_tables=$($serviceTables.Count) missing=$($missing.Count)"

if ($missing.Count -gt 0) {
    Write-Host ""
    Write-Host "Tables without an `impl_admin_entity_service*` admin CRUD service:"
    $missing | ForEach-Object { Write-Host " - $_" }

    if ($FailOnMissing) {
        exit 1
    }
}

exit 0
