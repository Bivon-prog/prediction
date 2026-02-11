# ⚡ Quick Start Guide

Get BetSure Analytics running locally in 5 minutes.

## Prerequisites

- Rust (1.70+)
- Node.js (18+)
- MongoDB (7.0+)
- Git

## Step 1: Clone Repository

```bash
git clone https://github.com/yourusername/betsure-analytics.git
cd betsure-analytics
```

## Step 2: Setup MongoDB

### macOS (Homebrew)
```bash
brew tap mongodb/brew
brew install mongodb-community@7.0
brew services start mongodb-community@7.0
```

### Ubuntu/Debian
```bash
wget -qO - https://www.mongodb.org/static/pgp/server-7.0.asc | sudo apt-key add -
echo "deb [ arch=amd64,arm64 ] https://repo.mongodb.org/apt/ubuntu jammy/mongodb-org/7.0 multiverse" | sudo tee /etc/apt/sources.list.d/mongodb-org-7.0.list
sudo apt update
sudo apt install -y mongodb-org
sudo systemctl start mongod
```

### Windows
Download and install from [MongoDB Download Center](https://www.mongodb.com/try/download/community)

## Step 3: Configure Backend

```bash
cd backend

# Copy environment file
cp .env.example .env

# Edit .env with your API keys
nano .env  # or use your favorite editor
```

**Required API Keys:**
1. **API-Football**: Get free key at [api-football.com](https://www.api-football.com/)
2. **M-Pesa Sandbox**: Get credentials at [Daraja Portal](https://developer.safaricom.co.ke/)

## Step 4: Seed Database

```bash
# Create test user
cargo run --bin seed_db
```

Copy the generated `user_id` - you'll need it for the frontend.

## Step 5: Start Backend

```bash
# Run API server
cargo run --bin server

# Server will start on http://127.0.0.1:8080
```

## Step 6: Setup Frontend

Open a new terminal:

```bash
cd frontend

# Install dependencies
npm install

# Configure API URL
echo "NEXT_PUBLIC_API_URL=http://localhost:8080" > .env.local

# Start development server
npm run dev

# Frontend will start on http://localhost:3000
```

## Step 7: Test the Application

1. Open browser to `http://localhost:3000`
2. You should see "Today's Predictions" page
3. Click "Get VIP Access" to test payment flow
4. Use M-Pesa sandbox test number: `254708374149`

## Step 8: Fetch Real Data (Optional)

```bash
# Run cron job manually to fetch today's fixtures
cd backend
cargo run --bin cron_job
```

This will:
- Fetch fixtures from API-Football
- Calculate predictions using Poisson distribution
- Save to MongoDB

## Troubleshooting

### MongoDB Connection Error
```bash
# Check if MongoDB is running
mongosh
# If it fails, start MongoDB:
# macOS: brew services start mongodb-community
# Linux: sudo systemctl start mongod
```

### API-Football Rate Limit
Free tier allows 100 requests/day. If you hit the limit:
- Wait 24 hours, or
- Upgrade to Pro plan ($15/month)

### M-Pesa Sandbox Issues
- Ensure you're using sandbox URL: `https://sandbox.safaricom.co.ke`
- Test phone: `254708374149`
- Test PIN: `1234`

### Port Already in Use
```bash
# Backend (port 8080)
lsof -ti:8080 | xargs kill -9

# Frontend (port 3000)
lsof -ti:3000 | xargs kill -9
```

## Development Workflow

### Backend Changes
```bash
cd backend
cargo watch -x 'run --bin server'  # Auto-reload on changes
```

### Frontend Changes
```bash
cd frontend
npm run dev  # Auto-reload enabled by default
```

### Run Tests
```bash
# Backend
cd backend
cargo test

# Frontend
cd frontend
npm test
```

## Project Structure

```
betsure-analytics/
├── backend/
│   ├── src/
│   │   ├── main.rs              # API server entry point
│   │   ├── bin/
│   │   │   ├── cron_job.rs      # Daily data ingestion
│   │   │   └── seed_db.rs       # Database seeding
│   │   ├── models/              # Data structures
│   │   ├── routes/              # API endpoints
│   │   ├── services/            # Business logic
│   │   │   ├── prediction_engine.rs  # Poisson algorithm
│   │   │   ├── data_ingestion.rs     # API-Football client
│   │   │   └── mpesa.rs              # M-Pesa integration
│   │   ├── config.rs            # Configuration
│   │   └── db.rs                # MongoDB connection
│   └── Cargo.toml
├── frontend/
│   ├── app/
│   │   ├── page.tsx             # Home page
│   │   └── layout.tsx           # Root layout
│   ├── components/
│   │   ├── MatchCard.tsx        # Match display
│   │   └── PaymentModal.tsx     # M-Pesa payment
│   └── lib/
│       └── api.ts               # API client
└── README.md
```

## Next Steps

1. **Customize Prediction Algorithm**: Edit `backend/src/services/prediction_engine.rs`
2. **Add More Leagues**: Modify API-Football queries in `data_ingestion.rs`
3. **Improve UI**: Customize Tailwind styles in `frontend/components/`
4. **Add Authentication**: Implement JWT in backend and frontend
5. **Deploy**: Follow `DEPLOYMENT.md` for production setup

## Useful Commands

```bash
# Backend
cargo build --release          # Production build
cargo test                     # Run tests
cargo run --bin cron_job       # Manual data fetch

# Frontend
npm run build                  # Production build
npm run start                  # Production server
npm run lint                   # Lint code

# Database
mongosh                        # MongoDB shell
mongosh --eval "use betsure; db.matches.find().pretty()"  # View matches
```

## Support

- **Documentation**: See `README.md` for detailed info
- **Deployment**: See `DEPLOYMENT.md` for production setup
- **Issues**: Open an issue on GitHub

---

Happy coding! ⚽🚀
