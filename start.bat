@echo off
REM BetSure Analytics - Quick Start Script (Windows)
REM This script helps you get the project running quickly

echo ⚽ BetSure Analytics - Quick Start
echo ==================================
echo.

REM Check prerequisites
echo 📋 Checking prerequisites...

REM Check Rust
where cargo >nul 2>nul
if %ERRORLEVEL% NEQ 0 (
    echo ❌ Rust not found. Install from: https://rustup.rs/
    exit /b 1
)
echo ✅ Rust installed

REM Check Node.js
where node >nul 2>nul
if %ERRORLEVEL% NEQ 0 (
    echo ❌ Node.js not found. Install from: https://nodejs.org/
    exit /b 1
)
echo ✅ Node.js installed

REM Check MongoDB
where mongosh >nul 2>nul
if %ERRORLEVEL% NEQ 0 (
    where mongo >nul 2>nul
    if %ERRORLEVEL% NEQ 0 (
        echo ⚠️  MongoDB CLI not found. Make sure MongoDB is running.
    ) else (
        echo ✅ MongoDB CLI installed
    )
) else (
    echo ✅ MongoDB CLI installed
)

echo.
echo 🔧 Setting up backend...
cd backend

REM Check if .env exists
if not exist .env (
    echo 📝 Creating .env file from .env.example...
    copy .env.example .env
    echo ⚠️  Please edit backend\.env with your API keys before continuing!
    echo    - API_FOOTBALL_KEY: Get from https://www.api-football.com/
    echo    - MPESA credentials: Get from https://developer.safaricom.co.ke/
    echo.
    pause
)

REM Build backend
echo 🔨 Building Rust backend...
cargo build --release

REM Seed database
echo 🌱 Seeding database...
cargo run --bin seed_db

echo.
echo ✅ Backend setup complete!
echo.

REM Setup frontend
echo 🔧 Setting up frontend...
cd ..\frontend

REM Install dependencies
echo 📦 Installing npm packages...
call npm install

REM Create .env.local if it doesn't exist
if not exist .env.local (
    echo NEXT_PUBLIC_API_URL=http://localhost:8080 > .env.local
    echo ✅ Created .env.local
)

echo.
echo ✅ Frontend setup complete!
echo.

REM Instructions
echo 🚀 Setup Complete! Next steps:
echo.
echo 1. Start the backend (in one terminal):
echo    cd backend
echo    cargo run --bin server
echo.
echo 2. Start the frontend (in another terminal):
echo    cd frontend
echo    npm run dev
echo.
echo 3. Open your browser to: http://localhost:3000
echo.
echo 4. (Optional) Fetch real data:
echo    cd backend
echo    cargo run --bin cron_job
echo.
echo 📚 Documentation:
echo    - README.md - Full documentation
echo    - QUICKSTART.md - Quick start guide
echo    - API_DOCUMENTATION.md - API reference
echo    - DEPLOYMENT.md - Production deployment
echo.
echo Happy coding! ⚽🚀
pause
