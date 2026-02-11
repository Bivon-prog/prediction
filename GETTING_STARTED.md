# 🚀 Getting Started with BetSure Analytics

Welcome! This guide will get you up and running in minutes.

## 📋 What You Need

Before starting, make sure you have:

1. **Rust** (1.70+) - [Install here](https://rustup.rs/)
2. **Node.js** (18+) - [Install here](https://nodejs.org/)
3. **MongoDB** (7.0+) - [Install here](https://www.mongodb.com/try/download/community)
4. **Git** - [Install here](https://git-scm.com/)

## ⚡ Quick Start (5 Minutes)

### Option 1: Automated Setup (Recommended)

**Windows:**
```cmd
start.bat
```

**macOS/Linux:**
```bash
chmod +x start.sh
./start.sh
```

The script will:
- Check prerequisites
- Setup backend and frontend
- Create configuration files
- Seed the database
- Show you next steps

### Option 2: Manual Setup

#### Step 1: Clone Repository
```bash
git clone https://github.com/yourusername/betsure-analytics.git
cd betsure-analytics
```

#### Step 2: Configure Backend
```bash
cd backend
cp .env.example .env
```

Edit `.env` and add your API keys:
- **API_FOOTBALL_KEY**: Get from [api-football.com](https://www.api-football.com/)
- **MPESA credentials**: Get from [Daraja Portal](https://developer.safaricom.co.ke/)

#### Step 3: Setup Database
```bash
# Make sure MongoDB is running
# macOS: brew services start mongodb-community
# Linux: sudo systemctl start mongod
# Windows: Start MongoDB service

# Seed database
cargo run --bin seed_db
```

Copy the generated `user_id` - you'll need it!

#### Step 4: Start Backend
```bash
cargo run --bin server
# Server starts on http://127.0.0.1:8080
```

#### Step 5: Setup Frontend (New Terminal)
```bash
cd frontend
npm install
echo "NEXT_PUBLIC_API_URL=http://localhost:8080" > .env.local
npm run dev
# Frontend starts on http://localhost:3000
```

#### Step 6: Open Browser
Visit `http://localhost:3000` and you should see the predictions page!

## 🎯 What's Next?

### 1. Fetch Real Data
```bash
cd backend
cargo run --bin cron_job
```

This fetches today's fixtures from API-Football and generates predictions.

### 2. Test Payment Flow
1. Click "Get VIP Access"
2. Enter phone: `254708374149` (sandbox test number)
3. Enter PIN: `1234`
4. Watch the magic happen!

### 3. Explore the Code

**Backend (Rust)**:
- `src/services/prediction_engine.rs` - The Poisson algorithm
- `src/services/mpesa.rs` - M-Pesa integration
- `src/routes/` - API endpoints

**Frontend (Next.js)**:
- `app/page.tsx` - Main page
- `components/MatchCard.tsx` - Match display
- `components/PaymentModal.tsx` - Payment UI

## 📚 Documentation

- **README.md** - Complete project documentation
- **QUICKSTART.md** - Detailed setup guide
- **API_DOCUMENTATION.md** - API reference
- **DEPLOYMENT.md** - Production deployment guide
- **ARCHITECTURE.md** - System architecture
- **PROJECT_SUMMARY.md** - Project overview
- **CHECKLIST.md** - Implementation checklist

## 🔧 Common Issues

### MongoDB Connection Error
```bash
# Check if MongoDB is running
mongosh
# If it fails, start MongoDB:
# macOS: brew services start mongodb-community
# Linux: sudo systemctl start mongod
# Windows: net start MongoDB
```

### Port Already in Use
```bash
# Backend (port 8080)
# Windows: netstat -ano | findstr :8080
# macOS/Linux: lsof -ti:8080 | xargs kill -9

# Frontend (port 3000)
# Windows: netstat -ano | findstr :3000
# macOS/Linux: lsof -ti:3000 | xargs kill -9
```

### API-Football Rate Limit
Free tier: 100 requests/day. If you hit the limit:
- Wait 24 hours, or
- Upgrade to Pro ($15/month)

### M-Pesa Sandbox Issues
- Use sandbox URL: `https://sandbox.safaricom.co.ke`
- Test phone: `254708374149`
- Test PIN: `1234`

## 🎓 Learning Path

### Day 1: Understand the Basics
1. Read README.md
2. Explore the code structure
3. Run the application locally
4. Test the prediction engine

### Day 2: Dive Deeper
1. Study the Poisson algorithm
2. Understand M-Pesa integration
3. Explore the database schema
4. Test API endpoints with Postman

### Day 3: Customize
1. Modify the prediction algorithm
2. Add new leagues
3. Customize the UI
4. Add new features

### Week 2: Deploy
1. Get production API keys
2. Setup a server
3. Deploy to production
4. Monitor and optimize

## 🤝 Get Help

### Documentation
- Check the docs in this repository
- Read inline code comments
- Review test files

### Community
- GitHub Issues: Report bugs or ask questions
- Stack Overflow: Tag with `rust`, `nextjs`, `mpesa`
- Discord: Join Rust and Next.js communities

### Professional Support
- Email: support@yourdomain.com
- Twitter: @BetSureAnalytics

## 🎉 You're Ready!

You now have a fully functional Football Prediction SaaS platform!

**What you can do:**
- ✅ View AI-powered match predictions
- ✅ Lock premium predictions behind paywall
- ✅ Accept M-Pesa payments
- ✅ Automatically fetch daily fixtures
- ✅ Scale to production

**Next steps:**
1. Customize the design
2. Add more features
3. Deploy to production
4. Start making money!

---

**Happy coding! ⚽🚀**

*Built with Rust, Next.js, and ❤️ for the Kenyan market*
