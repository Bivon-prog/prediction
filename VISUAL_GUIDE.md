# 📊 BetSure Analytics - Visual Guide

## 🎨 User Interface Flow

```
┌─────────────────────────────────────────────────────────────────┐
│                         HOME PAGE                                │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │  ⚽ BetSure Analytics                    [Get VIP Access]  │  │
│  │  AI-Powered Football Predictions                          │  │
│  └───────────────────────────────────────────────────────────┘  │
│                                                                  │
│  Today's Predictions                                             │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │  Premier League                              15:00         │  │
│  │  ┌──────────────┐    VS    ┌──────────────┐              │  │
│  │  │ Man City     │          │   Arsenal    │              │  │
│  │  │ Avg: 2.5     │          │   Avg: 2.2   │              │  │
│  │  └──────────────┘          └──────────────┘              │  │
│  │  ─────────────────────────────────────────────────────    │  │
│  │  Home Win: 48%   Draw: 26%   Away Win: 26%               │  │
│  │  Over 2.5: 65%                    Confidence: 48%         │  │
│  └───────────────────────────────────────────────────────────┘  │
│                                                                  │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │  Premier League                              17:30         │  │
│  │  ┌──────────────┐    VS    ┌──────────────┐              │  │
│  │  │ Liverpool    │          │   Chelsea    │              │  │
│  │  │ Avg: 2.8     │          │   Avg: 1.9   │              │  │
│  │  └──────────────┘          └──────────────┘              │  │
│  │  ─────────────────────────────────────────────────────    │  │
│  │  ╔═══════════════════════════════════════════════════╗    │  │
│  │  ║  🔒 PREMIUM PREDICTION - UNLOCK VIP (KES 100)   ║    │  │
│  │  ║  Confidence: 82%                                 ║    │  │
│  │  ╚═══════════════════════════════════════════════════╝    │  │
│  └───────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
```

## 💳 Payment Flow

```
Step 1: User Clicks "Unlock VIP"
┌─────────────────────────────────────┐
│  Unlock VIP Access                  │
│  ─────────────────────────────────  │
│  Get 24-hour access to premium      │
│  predictions with 75%+ confidence   │
│  for only KES 100.                  │
│                                     │
│  M-Pesa Phone Number                │
│  ┌─────────────────────────────┐   │
│  │ 254712345678                │   │
│  └─────────────────────────────┘   │
│                                     │
│  [Pay KES 100]  [Cancel]            │
└─────────────────────────────────────┘

Step 2: M-Pesa Prompt on Phone
┌─────────────────────────────────────┐
│  📱 M-Pesa                          │
│  ─────────────────────────────────  │
│  BetSure Analytics                  │
│  Amount: KES 100                    │
│                                     │
│  Enter M-Pesa PIN:                  │
│  ┌───┬───┬───┬───┐                 │
│  │ * │ * │ * │ * │                 │
│  └───┴───┴───┴───┘                 │
│                                     │
│  [Confirm]  [Cancel]                │
└─────────────────────────────────────┘

Step 3: Success Message
┌─────────────────────────────────────┐
│  ✅ Payment Successful!             │
│  ─────────────────────────────────  │
│  You now have VIP access for 24hrs  │
│  Receipt: NLJ7RT61SV                │
│                                     │
│  Refreshing predictions...          │
└─────────────────────────────────────┘

Step 4: Premium Unlocked
┌───────────────────────────────────────────────────────────┐
│  Premier League                              17:30         │
│  ┌──────────────┐    VS    ┌──────────────┐              │
│  │ Liverpool    │          │   Chelsea    │              │
│  │ Avg: 2.8     │          │   Avg: 1.9   │              │
│  └──────────────┘          └──────────────┘              │
│  ─────────────────────────────────────────────────────    │
│  Home Win: 68%   Draw: 22%   Away Win: 10%               │
│  Over 2.5: 72%                    Confidence: 82% 🔓      │
└───────────────────────────────────────────────────────────┘
```

## 🔄 Data Flow Diagram

```
┌──────────────────────────────────────────────────────────────────┐
│                    DAILY DATA INGESTION                           │
└──────────────────────────────────────────────────────────────────┘

06:00 AM - Cron Job Triggers
         │
         ▼
┌─────────────────┐
│  Fetch Fixtures │  GET /fixtures?date=2024-01-15
│  from API       │  ────────────────────────────────►
└────────┬────────┘
         │
         │ Response: [
         │   {fixture_id: 12345, home: "Man City", away: "Arsenal"},
         │   {fixture_id: 12346, home: "Liverpool", away: "Chelsea"}
         │ ]
         │
         ▼
┌─────────────────┐
│  For Each       │
│  Fixture        │
└────────┬────────┘
         │
    ┌────┴────┐
    │         │
    ▼         ▼
┌────────┐  ┌────────┐
│ Fetch  │  │ Fetch  │  GET /teams/statistics?team=33
│ Home   │  │ Away   │  ────────────────────────────────►
│ Stats  │  │ Stats  │
└───┬────┘  └───┬────┘
    │           │
    │ Goals: 2.5│ Goals: 2.2
    │ Conc: 0.8 │ Conc: 1.0
    │           │
    └─────┬─────┘
          │
          ▼
┌──────────────────────┐
│ Calculate Strengths  │
│ Home Attack: 0.93    │
│ Home Defense: 0.30   │
│ Away Attack: 0.81    │
│ Away Defense: 0.37   │
└──────────┬───────────┘
           │
           ▼
┌──────────────────────┐
│ Poisson Prediction   │
│ Expected Goals:      │
│ Home: 1.86           │
│ Away: 0.89           │
└──────────┬───────────┘
           │
           ▼
┌──────────────────────┐
│ Calculate Probs      │
│ For 0-0 to 5-5:      │
│ P(2-1) = 0.18        │
│ P(1-0) = 0.15        │
│ P(3-1) = 0.12        │
│ ...                  │
└──────────┬───────────┘
           │
           ▼
┌──────────────────────┐
│ Aggregate Results    │
│ Home Win: 68%        │
│ Draw: 22%            │
│ Away Win: 10%        │
│ Over 2.5: 45%        │
│ Confidence: 68%      │
└──────────┬───────────┘
           │
           ▼
┌──────────────────────┐
│ Save to MongoDB      │
│ matches collection   │
└──────────────────────┘
```

## 🧮 Prediction Algorithm Visualization

```
┌──────────────────────────────────────────────────────────────────┐
│              POISSON DISTRIBUTION PREDICTION                      │
└──────────────────────────────────────────────────────────────────┘

Input Data:
┌─────────────────────────────────────────────────────────────────┐
│  Man City (Home)              Arsenal (Away)                    │
│  ─────────────────            ─────────────────                 │
│  Goals Scored: 2.5/game       Goals Scored: 2.2/game           │
│  Goals Conceded: 0.8/game     Goals Conceded: 1.0/game         │
│                                                                 │
│  League Average: 2.7 goals/game                                 │
└─────────────────────────────────────────────────────────────────┘

Step 1: Calculate Strengths
┌─────────────────────────────────────────────────────────────────┐
│  Home Attack = 2.5 / 2.7 = 0.93  ⚡ (Strong)                    │
│  Home Defense = 0.8 / 2.7 = 0.30 🛡️ (Excellent)                │
│  Away Attack = 2.2 / 2.7 = 0.81  ⚡ (Good)                      │
│  Away Defense = 1.0 / 2.7 = 0.37 🛡️ (Good)                     │
└─────────────────────────────────────────────────────────────────┘

Step 2: Expected Goals (λ)
┌─────────────────────────────────────────────────────────────────┐
│  Home λ = 0.93 × 0.37 × 2.7 = 0.93 goals                        │
│  Away λ = 0.81 × 0.30 × 2.7 = 0.66 goals                        │
└─────────────────────────────────────────────────────────────────┘

Step 3: Probability Matrix (Poisson)
┌─────────────────────────────────────────────────────────────────┐
│        Away Goals                                               │
│      0     1     2     3     4     5                            │
│  0  0.19  0.13  0.04  0.01  0.00  0.00  ← Home Goals           │
│  1  0.18  0.12  0.04  0.01  0.00  0.00                         │
│  2  0.17  0.11  0.04  0.01  0.00  0.00                         │
│  3  0.05  0.04  0.01  0.00  0.00  0.00                         │
│  4  0.01  0.01  0.00  0.00  0.00  0.00                         │
│  5  0.00  0.00  0.00  0.00  0.00  0.00                         │
│                                                                 │
│  Green = Home Win | Yellow = Draw | Red = Away Win             │
└─────────────────────────────────────────────────────────────────┘

Step 4: Sum Probabilities
┌─────────────────────────────────────────────────────────────────┐
│  Home Win (h > a): 0.68 = 68% 🟢                                │
│  Draw (h = a):     0.22 = 22% 🟡                                │
│  Away Win (h < a): 0.10 = 10% 🔴                                │
│  Over 2.5 Goals:   0.45 = 45% ⚽                                │
│                                                                 │
│  Confidence: max(68%, 22%, 10%) = 68%                           │
│  Premium: 68% > 75%? NO ❌ (Free Tier)                          │
└─────────────────────────────────────────────────────────────────┘

Final Prediction:
┌─────────────────────────────────────────────────────────────────┐
│  🏆 MOST LIKELY: Man City Win (68%)                             │
│  📊 Confidence: Medium (68%)                                    │
│  🎯 Recommendation: Back Home Win                               │
│  💰 Tier: Free (confidence < 75%)                               │
└─────────────────────────────────────────────────────────────────┘
```

## 🗄️ Database Schema Visualization

```
┌──────────────────────────────────────────────────────────────────┐
│                         MONGODB DATABASE                          │
└──────────────────────────────────────────────────────────────────┘

Collection: users
┌─────────────────────────────────────────────────────────────────┐
│  {                                                               │
│    _id: ObjectId("507f1f77bcf86cd799439011"),                   │
│    phone_number: "254712345678",                                │
│    email: null,                                                 │
│    is_vip: true,                                                │
│    vip_expires_at: ISODate("2024-01-16T10:30:00Z"),            │
│    created_at: ISODate("2024-01-15T10:30:00Z"),                │
│    updated_at: ISODate("2024-01-15T10:30:00Z")                 │
│  }                                                               │
└─────────────────────────────────────────────────────────────────┘
         │
         │ References
         ▼
Collection: payments
┌─────────────────────────────────────────────────────────────────┐
│  {                                                               │
│    _id: ObjectId("65a1b2c3d4e5f6g7h8i9j0k2"),                   │
│    user_id: ObjectId("507f1f77bcf86cd799439011"), ◄─────────┐   │
│    phone_number: "254712345678",                           │   │
│    amount: 100.0,                                          │   │
│    mpesa_receipt: "NLJ7RT61SV",                            │   │
│    checkout_request_id: "ws_CO_150120241030...",           │   │
│    status: "Completed",                                    │   │
│    created_at: ISODate("2024-01-15T10:30:00Z"),           │   │
│    updated_at: ISODate("2024-01-15T10:31:00Z")            │   │
│  }                                                         │   │
└────────────────────────────────────────────────────────────┘   │
                                                                 │
Collection: matches                                              │
┌─────────────────────────────────────────────────────────────┐  │
│  {                                                           │  │
│    _id: ObjectId("65a1b2c3d4e5f6g7h8i9j0k1"),               │  │
│    fixture_id: 12345,                                       │  │
│    home_team: {                                             │  │
│      team_id: 33,                                           │  │
│      team_name: "Manchester City",                          │  │
│      goals_scored_avg: 2.5,                                 │  │
│      goals_conceded_avg: 0.8,                               │  │
│      attack_strength: 0.93,                                 │  │
│      defense_strength: 0.30                                 │  │
│    },                                                        │  │
│    away_team: { ... },                                      │  │
│    match_date: ISODate("2024-01-15T15:00:00Z"),            │  │
│    league: "Premier League",                                │  │
│    prediction: {                                            │  │
│      home_win_prob: 68.0,                                   │  │
│      draw_prob: 22.0,                                       │  │
│      away_win_prob: 10.0,                                   │  │
│      over_2_5_prob: 45.0,                                   │  │
│      confidence: 68.0,                                      │  │
│      is_premium: false                                      │  │
│    },                                                        │  │
│    created_at: ISODate("2024-01-15T06:00:00Z")             │  │
│  }                                                           │  │
└─────────────────────────────────────────────────────────────┘  │
                                                                 │
Indexes:                                                         │
┌─────────────────────────────────────────────────────────────┐  │
│  users:                                                      │  │
│    - phone_number (unique)                                  │  │
│                                                              │  │
│  matches:                                                    │  │
│    - match_date                                             │  │
│    - fixture_id (unique)                                    │  │
│                                                              │  │
│  payments:                                                   │  │
│    - user_id                                                │──┘
│    - checkout_request_id                                    │
└─────────────────────────────────────────────────────────────┘
```

## 🏗️ System Architecture

```
┌──────────────────────────────────────────────────────────────────┐
│                      PRODUCTION ARCHITECTURE                      │
└──────────────────────────────────────────────────────────────────┘

Internet
   │
   ▼
┌─────────────┐
│ Cloudflare  │  CDN + DDoS Protection
└──────┬──────┘
       │
       ▼
┌─────────────┐
│ Nginx       │  Reverse Proxy + SSL
│ Port 80/443 │
└──────┬──────┘
       │
   ┌───┴───┐
   │       │
   ▼       ▼
┌──────┐ ┌──────┐
│ Rust │ │Next.js│
│ API  │ │ SSR  │
│:8080 │ │:3000 │
└───┬──┘ └──────┘
    │
    ▼
┌──────────┐
│ MongoDB  │
│ :27017   │
└──────────┘

External Services:
┌─────────────────┐
│ API-Football    │  Fixtures & Stats
└─────────────────┘

┌─────────────────┐
│ M-Pesa Daraja   │  Payments
└─────────────────┘

Background Jobs:
┌─────────────────┐
│ Cron (6 AM)     │  Daily Data Fetch
└─────────────────┘
```

## 📱 Mobile Responsive Design

```
Desktop (1920x1080)
┌────────────────────────────────────────────────────────────┐
│  ⚽ BetSure Analytics              [Get VIP Access]        │
│  ──────────────────────────────────────────────────────    │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐    │
│  │  Match 1     │  │  Match 2     │  │  Match 3     │    │
│  │  Man City vs │  │  Liverpool vs│  │  Chelsea vs  │    │
│  │  Arsenal     │  │  Tottenham   │  │  Man Utd     │    │
│  └──────────────┘  └──────────────┘  └──────────────┘    │
└────────────────────────────────────────────────────────────┘

Tablet (768x1024)
┌──────────────────────────────────┐
│  ⚽ BetSure Analytics             │
│  [Get VIP Access]                │
│  ────────────────────────────    │
│  ┌──────────────┐                │
│  │  Match 1     │                │
│  │  Man City vs │                │
│  │  Arsenal     │                │
│  └──────────────┘                │
│  ┌──────────────┐                │
│  │  Match 2     │                │
│  │  Liverpool vs│                │
│  │  Tottenham   │                │
│  └──────────────┘                │
└──────────────────────────────────┘

Mobile (375x667)
┌──────────────────┐
│  ⚽ BetSure       │
│  [VIP]           │
│  ────────────    │
│  ┌────────────┐  │
│  │ Match 1    │  │
│  │ Man City   │  │
│  │ vs Arsenal │  │
│  └────────────┘  │
│  ┌────────────┐  │
│  │ Match 2    │  │
│  │ Liverpool  │  │
│  │ vs Spurs   │  │
│  └────────────┘  │
└──────────────────┘
```

---

**This visual guide helps you understand the system at a glance!**

For more details, see:
- ARCHITECTURE.md - Technical architecture
- README.md - Complete documentation
- API_DOCUMENTATION.md - API reference
