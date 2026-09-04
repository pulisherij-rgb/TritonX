Write-Host "==========================================" -ForegroundColor Cyan
Write-Host "🚀 Launching TritonX Engine Stack..." -ForegroundColor Green
Write-Host "==========================================" -ForegroundColor Cyan

# Step 1: Compile Rust shared library
Write-Host "⚙️ Compiling Rust Core DLL (Release Mode)..." -ForegroundColor Yellow
cargo build --lib --release

if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Rust Compilation Failed!" -ForegroundColor Red
    exit 1
}

# Step 2: Start API Server & Open Frontend
Write-Host "🌐 Launching FastAPI Backend on http://127.0.0.1:9000 ..." -ForegroundColor Green
Start-Process "http://127.0.0.1:9000/docs"
Start-Process "file:///$((Get-Location).Path)/frontend/index.html"

python -m uvicorn server2:app --port 9000