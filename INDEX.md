# 📚 BetSure Analytics - Documentation Index

Welcome to BetSure Analytics! This index will help you find exactly what you need.

## 🚀 Getting Started

**New to the project? Start here:**

1. **[GETTING_STARTED.md](GETTING_STARTED.md)** - 5-minute quick start guide
2. **[QUICKSTART.md](QUICKSTART.md)** - Detailed setup instructions
3. **[README.md](README.md)** - Complete project overview

**Setup Scripts:**
- `start.sh` (macOS/Linux) - Automated setup script
- `start.bat` (Windows) - Automated setup script

## 📖 Core Documentation

### Project Overview
- **[README.md](README.md)** - Main documentation with everything you need
- **[PROJECT_SUMMARY.md](PROJECT_SUMMARY.md)** - Executive summary and key features
- **[VISUAL_GUIDE.md](VISUAL_GUIDE.md)** - Visual diagrams and UI flows

### Technical Documentation
- **[ARCHITECTURE.md](ARCHITECTURE.md)** - System architecture and design
- **[API_DOCUMENTATION.md](API_DOCUMENTATION.md)** - Complete API reference
- **[DEPLOYMENT.md](DEPLOYMENT.md)** - Production deployment guide

### Implementation
- **[CHECKLIST.md](CHECKLIST.md)** - Implementation and launch checklist
- **[LICENSE](LICENSE)** - MIT License

## 🎯 By Use Case

### "I want to run this locally"
1. Read [GETTING_STARTED.md](GETTING_STARTED.md)
2. Run `./start.sh` (or `start.bat` on Windows)
3. Follow the on-screen instructions

### "I want to understand the code"
1. Read [PROJECT_SUMMARY.md](PROJECT_SUMMARY.md) - Overview
2. Read [ARCHITECTURE.md](ARCHITECTURE.md) - Technical details
3. Explore the code:
   - Backend: `backend/src/`
   - Frontend: `frontend/`

### "I want to deploy to production"
1. Read [DEPLOYMENT.md](DEPLOYMENT.md) - Complete deployment guide
2. Follow [CHECKLIST.md](CHECKLIST.md) - Pre-launch checklist
3. Setup monitoring and backups

### "I want to use the API"
1. Read [API_DOCUMENTATION.md](API_DOCUMENTATION.md) - API reference
2. Test with Postman collection (included in docs)
3. Integrate with your application

### "I want to understand the algorithm"
1. Read [PROJECT_SUMMARY.md](PROJECT_SUMMARY.md#the-prediction-algorithm)
2. Read [VISUAL_GUIDE.md](VISUAL_GUIDE.md#prediction-algorithm-visualization)
3. Study `backend/src/services/prediction_engine.rs`

### "I want to customize the project"
1. Read [ARCHITECTURE.md](ARCHITECTURE.md) - Understand the structure
2. Modify the code in:
   - Prediction: `backend/src/services/prediction_engine.rs`
   - UI: `frontend/components/`
   - API: `backend/src/routes/`

## 📂 Project Structure

```
betsure-analytics/
├── 📄 Documentation (You are here!)
│   ├── INDEX.md (this file)
│   ├── README.md
│   ├── GETTING_STARTED.md
│   ├── QUICKSTART.md
│   ├── PROJECT_SUMMARY.md
│   ├── ARCHITECTURE.md
│   ├── API_DOCUMENTATION.md
│   ├── DEPLOYMENT.md
│   ├── VISUAL_GUIDE.md
│   ├── CHECKLIST.md
│   └── LICENSE
│
├── 🦀 Backend (Rust)
│   ├── src/
│   │   ├── main.rs - API server
│   │   ├── lib.rs - Library exports
│   │   ├── config.rs - Configuration
│   │   ├── db.rs - Database connection
│   │   ├── models/ - Data structures
│   │   ├── routes/ - API endpoints
│   │   ├── services/ - Business logic
│   │   └── bin/ - Executables
│   ├── Cargo.toml - Dependencies
│   └── .env - Configuration
│
├── ⚛️ Frontend (Next.js)
│   ├── app/ - Pages
│   ├── components/ - React components
│   ├── lib/ - Utilities
│   ├── package.json - Dependencies
│   └── .env.local - Configuration
│
└── 🛠️ Scripts
    ├── start.sh - Setup script (Unix)
    └── start.bat - Setup script (Windows)
```

## 🔍 Quick Reference

### Key Files

**Backend:**
- `backend/src/services/prediction_engine.rs` - Poisson algorithm
- `backend/src/services/mpesa.rs` - M-Pesa integration
- `backend/src/routes/matches.rs` - Match endpoints
- `backend/src/routes/payments.rs` - Payment endpoints
- `backend/src/bin/cron_job.rs` - Daily data fetch

**Frontend:**
- `frontend/app/page.tsx` - Home page
- `frontend/components/MatchCard.tsx` - Match display
- `frontend/components/PaymentModal.tsx` - Payment UI
- `frontend/lib/api.ts` - API client

### Key Concepts

**Prediction Algorithm:**
- Uses Poisson Distribution
- Calculates attack/defense strengths
- Generates probabilities for outcomes
- Confidence-based premium classification

**Business Model:**
- Free: Predictions with <60% confidence
- Premium: Predictions with >75% confidence
- Price: KES 100 for 24 hours
- Payment: M-Pesa STK Push

**Tech Stack:**
- Backend: Rust + Actix-web
- Frontend: Next.js 14 + TypeScript
- Database: MongoDB
- Payments: M-Pesa Daraja API
- External Data: API-Football

## 📊 Documentation by Role

### For Developers
1. [GETTING_STARTED.md](GETTING_STARTED.md) - Setup
2. [ARCHITECTURE.md](ARCHITECTURE.md) - System design
3. [API_DOCUMENTATION.md](API_DOCUMENTATION.md) - API reference
4. Code comments in `backend/src/` and `frontend/`

### For DevOps Engineers
1. [DEPLOYMENT.md](DEPLOYMENT.md) - Deployment guide
2. [ARCHITECTURE.md](ARCHITECTURE.md#scalability-considerations) - Scaling
3. [CHECKLIST.md](CHECKLIST.md#production-deployment) - Launch checklist

### For Product Managers
1. [PROJECT_SUMMARY.md](PROJECT_SUMMARY.md) - Overview
2. [VISUAL_GUIDE.md](VISUAL_GUIDE.md) - UI flows
3. [PROJECT_SUMMARY.md](PROJECT_SUMMARY.md#business-metrics) - Metrics

### For Data Scientists
1. [PROJECT_SUMMARY.md](PROJECT_SUMMARY.md#the-prediction-algorithm) - Algorithm
2. [VISUAL_GUIDE.md](VISUAL_GUIDE.md#prediction-algorithm-visualization) - Visualization
3. `backend/src/services/prediction_engine.rs` - Implementation

### For Business Stakeholders
1. [PROJECT_SUMMARY.md](PROJECT_SUMMARY.md#executive-overview) - Summary
2. [PROJECT_SUMMARY.md](PROJECT_SUMMARY.md#business-metrics) - Revenue model
3. [CHECKLIST.md](CHECKLIST.md#feature-roadmap) - Roadmap

## 🎓 Learning Path

### Week 1: Basics
- [ ] Read [GETTING_STARTED.md](GETTING_STARTED.md)
- [ ] Run the application locally
- [ ] Explore [VISUAL_GUIDE.md](VISUAL_GUIDE.md)
- [ ] Test the prediction engine

### Week 2: Deep Dive
- [ ] Study [ARCHITECTURE.md](ARCHITECTURE.md)
- [ ] Read [API_DOCUMENTATION.md](API_DOCUMENTATION.md)
- [ ] Understand the Poisson algorithm
- [ ] Test M-Pesa integration

### Week 3: Customization
- [ ] Modify prediction algorithm
- [ ] Customize UI components
- [ ] Add new features
- [ ] Write tests

### Week 4: Deployment
- [ ] Read [DEPLOYMENT.md](DEPLOYMENT.md)
- [ ] Setup production server
- [ ] Deploy application
- [ ] Monitor and optimize

## 🔗 External Resources

### APIs
- [API-Football Documentation](https://www.api-football.com/documentation-v3)
- [M-Pesa Daraja API](https://developer.safaricom.co.ke/docs)

### Technologies
- [Rust Book](https://doc.rust-lang.org/book/)
- [Actix-web Guide](https://actix.rs/docs/)
- [Next.js Documentation](https://nextjs.org/docs)
- [MongoDB Manual](https://docs.mongodb.com/)

### Statistics
- [Poisson Distribution](https://en.wikipedia.org/wiki/Poisson_distribution)
- [Football Prediction Models](https://www.pinnacle.com/en/betting-articles/Soccer/how-to-calculate-poisson-distribution)

## 💡 Tips

### For Best Results
1. **Start with GETTING_STARTED.md** - Don't skip the basics
2. **Use the setup scripts** - They save time
3. **Read the code comments** - They explain the "why"
4. **Test locally first** - Before deploying
5. **Follow the checklist** - For production deployment

### Common Pitfalls
- ❌ Skipping environment configuration
- ❌ Not testing M-Pesa in sandbox first
- ❌ Deploying without SSL for M-Pesa callbacks
- ❌ Not setting up database indexes
- ❌ Ignoring rate limits on API-Football

### Pro Tips
- ✅ Use the automated setup scripts
- ✅ Test with sandbox credentials first
- ✅ Monitor API rate limits
- ✅ Setup backups before going live
- ✅ Use the checklist for deployment

## 🆘 Getting Help

### Documentation
- Check this index for the right document
- Read inline code comments
- Review test files for examples

### Community
- GitHub Issues - Report bugs
- Stack Overflow - Technical questions
- Discord - Rust and Next.js communities

### Professional Support
- Email: support@yourdomain.com
- Twitter: @BetSureAnalytics

## 📝 Contributing

Want to improve the documentation?

1. Fork the repository
2. Make your changes
3. Submit a pull request

See [README.md](README.md#contributing) for details.

## 🎉 Ready to Start?

Choose your path:

**Quick Start (5 minutes):**
→ [GETTING_STARTED.md](GETTING_STARTED.md)

**Detailed Setup:**
→ [QUICKSTART.md](QUICKSTART.md)

**Understand the System:**
→ [ARCHITECTURE.md](ARCHITECTURE.md)

**Deploy to Production:**
→ [DEPLOYMENT.md](DEPLOYMENT.md)

**Use the API:**
→ [API_DOCUMENTATION.md](API_DOCUMENTATION.md)

---

**Happy coding! ⚽🚀**

*Last Updated: January 2024*
