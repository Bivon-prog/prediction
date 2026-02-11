# 🏗️ System Architecture - BetSure Analytics

## High-Level Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                         CLIENT LAYER                             │
├─────────────────────────────────────────────────────────────────┤
│  Web Browser (Next.js 14)                                        │
│  - React Components                                              │
│  - Tailwind CSS                                                  │
│  - Axios HTTP Client                                             │
└────────────────────┬────────────────────────────────────────────┘
                     │ HTTPS/REST
                     │
┌────────────────────▼────────────────────────────────────────────┐
│                      API GATEWAY LAYER                           │
├─────────────────────────────────────────────────────────────────┤
│  Nginx/Caddy Reverse Proxy                                       │
│  - SSL Termination                                               │
│  - Load Balancing                                                │
│  - Rate Limiting                                                 │
└────────────────────┬────────────────────────────────────────────┘
                     │
        ┌────────────┴────────────┐
        │                         │
┌───────▼──────┐         ┌────────▼────────┐
│ APPLICATION  │         │   EXTERNAL      │
│    LAYER     │         │   SERVICES      │
├──────────────┤         ├─────────────────┤
│ Rust API     │◄────────┤ API-Football    │
│ (Actix-web)  │         │ (Fixtures/Stats)│
│              │         │                 │
│ - Routes     │         │ M-Pesa Daraja   │
│ - Services   │         │ (Payments)      │
│ - Models     │         └─────────────────┘
└──────┬───────┘
       │
       │ MongoDB Driver
       │
┌──────▼───────────────────────────────────────────────────────────┐
│                      DATA LAYER                                   │
├──────────────────────────────────────────────────────────────────┤
│  MongoDB Database                                                 │
│  - users collection                                               │
│  - matches collection                                             │
│  - payments collection                                            │
└──────────────────────────────────────────────────────────────────┘

┌──────────────────────────────────────────────────────────────────┐
│                    BACKGROUND JOBS                                │
├──────────────────────────────────────────────────────────────────┤
│  Cron Job (Tokio Runtime)                                         │
│  - Daily at 6 AM                                                  │
│  - Fetch fixtures                                                 │
│  - Generate predictions                                           │
│  - Save to database                                               │
└──────────────────────────────────────────────────────────────────┘
```

## Component Interaction Flow

### 1. User Views Predictions

```
┌──────┐     GET /matches/today?user_id=xxx     ┌──────────┐
│      ├────────────────────────────────────────►│          │
│ User │                                         │ Rust API │
│      │◄────────────────────────────────────────┤          │
└──────┘     JSON: [matches with predictions]   └────┬─────┘
                                                      │
                                                      │ Query
                                                      │
                                                 ┌────▼─────┐
                                                 │ MongoDB  │
                                                 │ matches  │
                                                 └──────────┘
```

**Steps**:
1. Frontend sends GET request with user_id
2. Backend checks if user is VIP
3. Backend queries matches for today
4. If user is not VIP, blur premium predictions
5. Return JSON response
6. Frontend renders MatchCard components

### 2. User Purchases VIP

```
┌──────┐  POST /pay/mpesa/initiate  ┌──────────┐  OAuth Token  ┌─────────┐
│      ├───────────────────────────►│          ├──────────────►│         │
│ User │                            │ Rust API │               │ M-Pesa  │
│      │◄───────────────────────────┤          │◄──────────────┤ Daraja  │
└──────┘  checkout_request_id       └────┬─────┘  Access Token └─────────┘
                                         │                            │
                                         │ Save Payment               │
                                         │ (Pending)                  │
                                    ┌────▼─────┐                     │
                                    │ MongoDB  │                     │
                                    │ payments │                     │
                                    └──────────┘                     │
                                                                     │
┌──────┐                                                             │
│ User │◄────────────────────────────────────────────────────────────┘
│Phone │  STK Push (Enter PIN)
└──────┘
    │
    │ User enters PIN
    │
    ▼
┌─────────┐  POST /pay/mpesa/callback  ┌──────────┐
│ M-Pesa  ├───────────────────────────►│ Rust API │
│ Daraja  │  {ResultCode: 0, Receipt}  └────┬─────┘
└─────────┘                                  │
                                             │ Update Payment
                                             │ Set is_vip=true
                                        ┌────▼─────┐
                                        │ MongoDB  │
                                        │ users &  │
                                        │ payments │
                                        └──────────┘
```

**Steps**:
1. User clicks "Unlock VIP"
2. Frontend calls `/pay/mpesa/initiate`
3. Backend gets OAuth token from M-Pesa
4. Backend initiates STK Push
5. Backend saves payment as "Pending"
6. User receives phone prompt
7. User enters M-Pesa PIN
8. M-Pesa calls `/pay/mpesa/callback`
9. Backend updates payment to "Completed"
10. Backend sets user.is_vip = true
11. User refreshes and sees premium predictions

### 3. Daily Data Ingestion

```
┌──────────┐  Cron Trigger (6 AM)  ┌──────────┐
│ Crontab  ├──────────────────────►│ cron_job │
└──────────┘                        └────┬─────┘
                                         │
                                         │ GET /fixtures?date=today
                                         │
                                    ┌────▼──────────┐
                                    │ API-Football  │
                                    └────┬──────────┘
                                         │
                                         │ Fixtures JSON
                                         │
                                    ┌────▼─────┐
                                    │ For each │
                                    │ fixture  │
                                    └────┬─────┘
                                         │
                    ┌────────────────────┼────────────────────┐
                    │                    │                    │
            ┌───────▼────────┐  ┌────────▼────────┐  ┌───────▼────────┐
            │ Fetch Home     │  │ Fetch Away      │  │ Calculate      │
            │ Team Stats     │  │ Team Stats      │  │ Strengths      │
            └───────┬────────┘  └────────┬────────┘  └───────┬────────┘
                    │                    │                    │
                    └────────────────────┼────────────────────┘
                                         │
                                    ┌────▼─────────────┐
                                    │ Poisson Engine   │
                                    │ - Calculate λ    │
                                    │ - Iterate 0-5    │
                                    │ - Sum probs      │
                                    └────┬─────────────┘
                                         │
                                         │ Prediction
                                         │
                                    ┌────▼─────┐
                                    │ MongoDB  │
                                    │ matches  │
                                    └──────────┘
```

**Steps**:
1. Cron triggers at 6 AM
2. Fetch today's fixtures from API-Football
3. For each fixture:
   - Fetch home team statistics
   - Fetch away team statistics
   - Calculate attack/defense strengths
   - Run Poisson prediction algorithm
   - Save match with prediction to MongoDB

## Data Flow Diagram

### Prediction Engine Detail

```
┌─────────────────────────────────────────────────────────────────┐
│                    PREDICTION ENGINE                             │
└─────────────────────────────────────────────────────────────────┘

Input:
┌──────────────────┐
│ Team Statistics  │
│ - Home Goals Avg │
│ - Away Goals Avg │
│ - Home Conc. Avg │
│ - Away Conc. Avg │
└────────┬─────────┘
         │
         ▼
┌────────────────────────────────────┐
│ Calculate Strengths                │
│ Attack = Goals / League Avg        │
│ Defense = Conceded / League Avg    │
└────────┬───────────────────────────┘
         │
         ▼
┌────────────────────────────────────┐
│ Expected Goals (λ)                 │
│ Home λ = Home Atk × Away Def × Avg │
│ Away λ = Away Atk × Home Def × Avg │
└────────┬───────────────────────────┘
         │
         ▼
┌────────────────────────────────────┐
│ Poisson Distribution               │
│ For each scoreline (0-0 to 5-5):  │
│   P(h,a) = Poisson(λh,h)×Poisson(λa,a) │
└────────┬───────────────────────────┘
         │
         ▼
┌────────────────────────────────────┐
│ Aggregate Probabilities            │
│ Home Win = Σ P(h>a)                │
│ Draw = Σ P(h=a)                    │
│ Away Win = Σ P(h<a)                │
│ Over 2.5 = Σ P(h+a>2)              │
└────────┬───────────────────────────┘
         │
         ▼
┌────────────────────────────────────┐
│ Calculate Confidence               │
│ Confidence = max(Home, Draw, Away) │
│ is_premium = Confidence > 75%      │
└────────┬───────────────────────────┘
         │
         ▼
Output:
┌──────────────────┐
│ Prediction       │
│ - home_win_prob  │
│ - draw_prob      │
│ - away_win_prob  │
│ - over_2_5_prob  │
│ - confidence     │
│ - is_premium     │
└──────────────────┘
```

## Technology Stack Details

### Backend (Rust)

```
┌─────────────────────────────────────────────────────────────────┐
│                         RUST BACKEND                             │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐          │
│  │  Actix-web   │  │    Tokio     │  │   MongoDB    │          │
│  │  (HTTP)      │  │  (Async)     │  │   Driver     │          │
│  └──────────────┘  └──────────────┘  └──────────────┘          │
│                                                                  │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐          │
│  │   Serde      │  │   Reqwest    │  │   Statrs     │          │
│  │   (JSON)     │  │  (HTTP Cli)  │  │  (Poisson)   │          │
│  └──────────────┘  └──────────────┘  └──────────────┘          │
│                                                                  │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐          │
│  │   Chrono     │  │   Dotenvy    │  │   Base64     │          │
│  │   (Time)     │  │   (.env)     │  │  (Encoding)  │          │
│  └──────────────┘  └──────────────┘  └──────────────┘          │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

**Key Libraries**:
- `actix-web`: High-performance HTTP server
- `tokio`: Async runtime for concurrent operations
- `mongodb`: Official MongoDB driver
- `serde`: Serialization/deserialization
- `reqwest`: HTTP client for external APIs
- `statrs`: Statistical functions (Poisson)
- `chrono`: Date/time handling
- `dotenvy`: Environment variable loading
- `base64`: Encoding for M-Pesa

### Frontend (Next.js)

```
┌─────────────────────────────────────────────────────────────────┐
│                       NEXT.JS FRONTEND                           │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐          │
│  │   React 18   │  │  TypeScript  │  │  Tailwind    │          │
│  │  (UI)        │  │  (Types)     │  │  (Styles)    │          │
│  └──────────────┘  └──────────────┘  └──────────────┘          │
│                                                                  │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐          │
│  │    Axios     │  │  date-fns    │  │   Next.js    │          │
│  │  (HTTP)      │  │  (Dates)     │  │   Router     │          │
│  └──────────────┘  └──────────────┘  └──────────────┘          │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

**Key Libraries**:
- `react`: UI component library
- `next`: React framework with SSR
- `typescript`: Type safety
- `tailwindcss`: Utility-first CSS
- `axios`: HTTP client
- `date-fns`: Date formatting

## Security Architecture

### Authentication Flow (Future)

```
┌──────┐  1. Login (phone + OTP)  ┌──────────┐
│ User ├─────────────────────────►│ Rust API │
└──────┘                           └────┬─────┘
   ▲                                    │
   │                                    │ 2. Verify OTP
   │                                    │
   │                              ┌─────▼────┐
   │                              │ MongoDB  │
   │                              │ users    │
   │                              └─────┬────┘
   │                                    │
   │  4. JWT Token                      │ 3. Generate JWT
   │  (expires in 7 days)               │
   └────────────────────────────────────┘

All subsequent requests:
┌──────┐  GET /matches/today         ┌──────────┐
│ User ├────────────────────────────►│ Rust API │
└──────┘  Header: Authorization:     └────┬─────┘
          Bearer <JWT>                     │
                                           │ Verify JWT
                                           │ Extract user_id
                                           │
                                      ┌────▼─────┐
                                      │ Process  │
                                      │ Request  │
                                      └──────────┘
```

### Data Security

```
┌─────────────────────────────────────────────────────────────────┐
│                      SECURITY LAYERS                             │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  Layer 1: Network                                                │
│  ┌────────────────────────────────────────────────────────┐     │
│  │ - HTTPS/TLS 1.3                                        │     │
│  │ - Firewall (UFW)                                       │     │
│  │ - DDoS Protection (Cloudflare)                         │     │
│  └────────────────────────────────────────────────────────┘     │
│                                                                  │
│  Layer 2: Application                                            │
│  ┌────────────────────────────────────────────────────────┐     │
│  │ - Input Validation                                     │     │
│  │ - Rate Limiting                                        │     │
│  │ - CORS Policy                                          │     │
│  │ - JWT Authentication (future)                          │     │
│  └────────────────────────────────────────────────────────┘     │
│                                                                  │
│  Layer 3: Data                                                   │
│  ┌────────────────────────────────────────────────────────┐     │
│  │ - MongoDB Authentication                               │     │
│  │ - Encryption at Rest                                   │     │
│  │ - Environment Variables                                │     │
│  │ - No PII in logs                                       │     │
│  └────────────────────────────────────────────────────────┘     │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

## Scalability Considerations

### Horizontal Scaling

```
                    ┌──────────────┐
                    │ Load Balancer│
                    │  (Nginx)     │
                    └──────┬───────┘
                           │
        ┌──────────────────┼──────────────────┐
        │                  │                  │
   ┌────▼────┐       ┌─────▼────┐      ┌─────▼────┐
   │ API     │       │ API      │      │ API      │
   │ Server 1│       │ Server 2 │      │ Server 3 │
   └────┬────┘       └─────┬────┘      └─────┬────┘
        │                  │                  │
        └──────────────────┼──────────────────┘
                           │
                    ┌──────▼───────┐
                    │   MongoDB    │
                    │ Replica Set  │
                    └──────────────┘
```

### Caching Strategy

```
┌──────┐  Request  ┌──────────┐  Cache Miss  ┌──────────┐
│ User ├──────────►│  Redis   ├─────────────►│ MongoDB  │
└──────┘           │  Cache   │              └──────────┘
   ▲               └────┬─────┘                    │
   │                    │                          │
   │  Response          │ Cache Hit                │
   │  (Fast)            │                          │
   └────────────────────┘                          │
                                                   │
   ┌────────────────────────────────────────────────┘
   │
   │  Store in Cache (TTL: 1 hour)
   │
   ▼
┌──────────┐
│  Redis   │
│  Cache   │
└──────────┘
```

**Cache Keys**:
- `matches:today:{date}` - Today's matches
- `user:{user_id}` - User data
- `team_stats:{team_id}` - Team statistics

## Monitoring & Observability

```
┌─────────────────────────────────────────────────────────────────┐
│                      MONITORING STACK                            │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  Application Logs                                                │
│  ┌────────────────────────────────────────────────────────┐     │
│  │ - Rust: env_logger → stdout                            │     │
│  │ - Next.js: console → stdout                            │     │
│  │ - Nginx: access.log, error.log                         │     │
│  └────────────────────────────────────────────────────────┘     │
│                                                                  │
│  Metrics (Future)                                                │
│  ┌────────────────────────────────────────────────────────┐     │
│  │ - Prometheus: Scrape /metrics endpoint                 │     │
│  │ - Grafana: Visualize metrics                           │     │
│  │ - Alerts: PagerDuty/Slack                              │     │
│  └────────────────────────────────────────────────────────┘     │
│                                                                  │
│  Health Checks                                                   │
│  ┌────────────────────────────────────────────────────────┐     │
│  │ - GET /health → 200 OK                                 │     │
│  │ - UptimeRobot: Check every 5 minutes                   │     │
│  │ - Alert if down > 2 minutes                            │     │
│  └────────────────────────────────────────────────────────┘     │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

## Deployment Pipeline (Future)

```
┌──────────┐  git push  ┌──────────┐  webhook  ┌──────────┐
│Developer ├───────────►│  GitHub  ├──────────►│ CI/CD    │
└──────────┘            └──────────┘           │ (Actions)│
                                               └────┬─────┘
                                                    │
                                    ┌───────────────┼───────────────┐
                                    │               │               │
                              ┌─────▼────┐    ┌────▼────┐    ┌─────▼────┐
                              │  Build   │    │  Test   │    │  Deploy  │
                              │  Rust    │    │  cargo  │    │  to      │
                              │  Next.js │    │  test   │    │  Server  │
                              └──────────┘    └─────────┘    └──────────┘
```

---

This architecture is designed for:
- **Performance**: Rust backend, async operations
- **Scalability**: Horizontal scaling ready
- **Reliability**: Error handling, retries
- **Security**: Multiple layers of protection
- **Maintainability**: Clean separation of concerns
