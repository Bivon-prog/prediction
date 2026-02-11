# ⚽ BetSure Analytics - Football Prediction SaaS

A high-performance football prediction platform built with Rust and Next.js, featuring AI-powered predictions using Poisson distribution and M-Pesa payment integration.

## 🏗️ Architecture

### Backend (Rust)
- **Framework**: Actix-web for REST API
- **Database**: MongoDB with async driver
- **Prediction Engine**: Statistical model using Poisson distribution
- **Payment**: Safaricom M-Pesa Daraja API integration
- **Background Jobs**: Tokio-based async cron jobs

### Frontend (Next.js 14)
- **Framework**: Next.js 14 with TypeScript
- **Styling**: Tailwind CSS
- **State Management**: React hooks
- **API Client**: Axios

## 📊 Database Schema

### User Collection
```javascript
{
  _id: ObjectId,
  phone_number: String,
  email: String?,
  is_vip: Boolean,
  vip_expires_at: DateTime?,
  created_at: DateTime,
  updated_at: DateTime
}
```

### Match Collection
```javascript
{
  _id: ObjectId,
  fixture_id: Int32,
  home_team: {
    team_id: Int32,
    team_name: String,
    goals_scored_avg: Float64,
    goals_conceded_avg: Float64,
    attack_strength: Float64,
    defense_strength: Float64
  },
  away_team: { /* same structure */ },
  match_date: DateTime,
  league: String,
  prediction: {
    home_win_prob: Float64,
    draw_prob: Float64,
    away_win_prob: Float64,
    over_2_5_prob: Float64,
    confidence: Float64,
    is_premium: Boolean
  },
  created_at: DateTime
}
```

### Payment Collection
```javascript
{
  _id: ObjectId,
  user_id: ObjectId,
  phone_number: String,
  amount: Float64,
  mpesa_receipt: String?,
  checkout_request_id: String,
  status: Enum["Pending", "Completed", "Failed"],
  created_at: DateTime,
  updated_at: DateTime
}
```

## 🧮 Prediction Algorithm

The prediction engine uses **Poisson Distribution** to calculate match outcomes:

### 1. Calculate Team Strengths
```
Attack Strength = Team Goals Scored Avg / League Avg Goals
Defense Strength = Team Goals Conceded Avg / League Avg Goals
```

### 2. Expected Goals
```
Home Expected Goals = Home Attack × Away Defense × League Avg
Away Expected Goals = Away Attack × Home Defense × League Avg
```

### 3. Poisson Probability
For each possible scoreline (0-0 to 5-5):
```
P(Home = h, Away = a) = Poisson(λ_home, h) × Poisson(λ_away, a)
```

### 4. Outcome Probabilities
- **Home Win**: Sum of P where home_goals > away_goals
- **Draw**: Sum of P where home_goals = away_goals
- **Away Win**: Sum of P where home_goals < away_goals
- **Over 2.5**: Sum of P where home_goals + away_goals > 2

### 5. Confidence Score
```
Confidence = max(Home Win %, Draw %, Away Win %)
```

Predictions with confidence > 75% are marked as premium.

## 🚀 API Endpoints

### GET /matches/today
Fetch today's match predictions.

**Query Parameters:**
- `user_id`: MongoDB ObjectId

**Response:**
```json
[
  {
    "id": "...",
    "fixture_id": 12345,
    "home_team": { "team_name": "Man City", ... },
    "away_team": { "team_name": "Arsenal", ... },
    "match_date": "2024-01-15T15:00:00Z",
    "league": "Premier League",
    "prediction": {
      "home_win_prob": 45.0,
      "draw_prob": 28.0,
      "away_win_prob": 27.0,
      "over_2_5_prob": 62.0,
      "confidence": 45.0,
      "is_premium": false
    }
  }
]
```

### POST /pay/mpesa/initiate
Initiate M-Pesa STK Push for VIP subscription.

**Request Body:**
```json
{
  "user_id": "507f1f77bcf86cd799439011",
  "phone_number": "254712345678"
}
```

**Response:**
```json
{
  "checkout_request_id": "ws_CO_...",
  "message": "Check your phone for M-Pesa prompt"
}
```

### POST /pay/mpesa/callback
M-Pesa callback endpoint (called by Safaricom).

**Request Body:** M-Pesa callback payload

**Response:**
```json
{
  "ResultCode": 0,
  "ResultDesc": "Accepted"
}
```

## 🔧 Setup Instructions

### Backend Setup

1. **Install Rust**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. **Configure Environment**
```bash
cd backend
cp .env.example .env
# Edit .env with your credentials
```

3. **Run the API Server**
```bash
cargo run --bin server
```

4. **Setup Cron Job** (Linux/macOS)
```bash
# Build the cron binary
cargo build --release --bin cron_job

# Add to crontab (runs daily at 6 AM)
crontab -e
# Add: 0 6 * * * /path/to/target/release/cron_job
```

### Frontend Setup

1. **Install Dependencies**
```bash
cd frontend
npm install
```

2. **Configure Environment**
```bash
cp .env.local.example .env.local
# Set NEXT_PUBLIC_API_URL
```

3. **Run Development Server**
```bash
npm run dev
```

4. **Build for Production**
```bash
npm run build
npm start
```

## 💰 Business Model

### Free Tier
- Access to predictions with <60% confidence
- Basic match statistics
- Limited daily views

### Premium Tier (KES 100 / 24 hours)
- Unlock predictions with >75% confidence
- Full access to all matches
- Advanced statistics
- Priority support

## 🔐 M-Pesa Integration

### Sandbox Testing
1. Get credentials from [Daraja Portal](https://developer.safaricom.co.ke/)
2. Use test credentials in `.env`
3. Test phone: `254708374149`

### Production
1. Apply for production credentials
2. Update `.env` with production URLs
3. Configure callback URL (must be HTTPS)
4. Whitelist your IP address

## 📈 Scaling Considerations

1. **Database Indexing**
```javascript
db.matches.createIndex({ "match_date": 1 })
db.users.createIndex({ "phone_number": 1 })
db.payments.createIndex({ "checkout_request_id": 1 })
```

2. **Caching** (Redis)
- Cache today's matches
- Cache team statistics
- TTL: 1 hour

3. **Rate Limiting**
- API-Football: 100 requests/day (free tier)
- Implement request queuing
- Cache API responses

4. **Horizontal Scaling**
- Deploy multiple API instances
- Use MongoDB replica sets
- Load balancer (Nginx/Caddy)

## 🧪 Testing

### Backend Tests
```bash
cd backend
cargo test
```

### Frontend Tests
```bash
cd frontend
npm test
```

## 📝 License

MIT License - See LICENSE file for details

## 🤝 Contributing

1. Fork the repository
2. Create feature branch
3. Commit changes
4. Push to branch
5. Open pull request

---

Built with ❤️ for the Kenyan market
