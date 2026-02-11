#!/bin/bash

# BetSure Analytics - Quick Start Script
# This script helps you get the project running quickly

set -e

echo "⚽ BetSure Analytics - Quick Start"
echo "=================================="
echo ""

# Check prerequisites
echo "📋 Checking prerequisites..."

# Check Rust
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust not found. Install from: https://rustup.rs/"
    exit 1
fi
echo "✅ Rust installed"

# Check Node.js
if ! command -v node &> /dev/null; then
    echo "❌ Node.js not found. Install from: https://nodejs.org/"
    exit 1
fi
echo "✅ Node.js installed"

# Check MongoDB
if ! command -v mongosh &> /dev/null && ! command -v mongo &> /dev/null; then
    echo "⚠️  MongoDB CLI not found. Make sure MongoDB is running."
else
    echo "✅ MongoDB CLI installed"
fi

echo ""
echo "🔧 Setting up backend..."
cd backend

# Check if .env exists
if [ ! -f .env ]; then
    echo "📝 Creating .env file from .env.example..."
    cp .env.example .env
    echo "⚠️  Please edit backend/.env with your API keys before continuing!"
    echo "   - API_FOOTBALL_KEY: Get from https://www.api-football.com/"
    echo "   - MPESA credentials: Get from https://developer.safaricom.co.ke/"
    echo ""
    read -p "Press Enter after you've configured .env..."
fi

# Build backend
echo "🔨 Building Rust backend..."
cargo build --release

# Seed database
echo "🌱 Seeding database..."
cargo run --bin seed_db

echo ""
echo "✅ Backend setup complete!"
echo ""

# Setup frontend
echo "🔧 Setting up frontend..."
cd ../frontend

# Install dependencies
echo "📦 Installing npm packages..."
npm install

# Create .env.local if it doesn't exist
if [ ! -f .env.local ]; then
    echo "NEXT_PUBLIC_API_URL=http://localhost:8080" > .env.local
    echo "✅ Created .env.local"
fi

echo ""
echo "✅ Frontend setup complete!"
echo ""

# Instructions
echo "🚀 Setup Complete! Next steps:"
echo ""
echo "1. Start the backend (in one terminal):"
echo "   cd backend"
echo "   cargo run --bin server"
echo ""
echo "2. Start the frontend (in another terminal):"
echo "   cd frontend"
echo "   npm run dev"
echo ""
echo "3. Open your browser to: http://localhost:3000"
echo ""
echo "4. (Optional) Fetch real data:"
echo "   cd backend"
echo "   cargo run --bin cron_job"
echo ""
echo "📚 Documentation:"
echo "   - README.md - Full documentation"
echo "   - QUICKSTART.md - Quick start guide"
echo "   - API_DOCUMENTATION.md - API reference"
echo "   - DEPLOYMENT.md - Production deployment"
echo ""
echo "Happy coding! ⚽🚀"
