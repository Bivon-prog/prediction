# 📡 BetSure Analytics API Documentation

Base URL: `http://localhost:8080` (Development)  
Production: `https://api.yourdomain.com`

## Authentication

Currently, the API uses user_id for identification. In production, implement JWT tokens.

## Endpoints

### 1. Health Check

Check if the API is running.

**Endpoint:** `GET /health`

**Response:**
```json
{
  "status": "healthy",
  "timestamp": "2024-01-15T10:30:00Z"
}
```

---

### 2. Get Today's Matches

Fetch all matches scheduled for today with predictions.

**Endpoint:** `GET /matches/today`

**Query Parameters:**
| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| user_id | String | Yes | MongoDB ObjectId of the user |

**Example Request:**
```bash
curl "http://localhost:8080/matches/today?user_id=507f1f77bcf86cd799439011"
```

**Response (200 OK):**
```json
[
  {
    "id": "65a1b2c3d4e5f6g7h8i9j0k1",
    "fixture_id": 12345,
    "home_team": {
      "team_id": 33,
      "team_name": "Manchester City",
      "goals_scored_avg": 2.5,
      "goals_conceded_avg": 0.8,
      "attack_strength": 0.93,
      "defense_strength": 0.30
    },
    "away_team": {
      "team_id": 42,
      "team_name": "Arsenal",
      "goals_scored_avg": 2.2,
      "goals_conceded_avg": 1.0,
      "attack_strength": 0.81,
      "defense_strength": 0.37
    },
    "match_date": "2024-01-15T15:00:00Z",
    "league": "Premier League",
    "prediction": {
      "home_win_prob": 48.0,
      "draw_prob": 26.0,
      "away_win_prob": 26.0,
      "over_2_5_prob": 65.0,
      "confidence": 48.0,
      "is_premium": false
    },
    "created_at": "2024-01-15T06:00:00Z"
  }
]
```

**Response (Premium Locked for Non-VIP):**
```json
{
  "prediction": {
    "home_win_prob": 0.0,
    "draw_prob": 0.0,
    "away_win_prob": 0.0,
    "over_2_5_prob": 0.0,
    "confidence": 82.0,
    "is_premium": true
  }
}
```

**Error Responses:**

`400 Bad Request` - Invalid user_id format
```json
{
  "error": "Invalid user_id"
}
```

`404 Not Found` - User not found
```json
{
  "error": "User not found"
}
```

`500 Internal Server Error` - Database error
```json
{
  "error": "Database error"
}
```

---

### 3. Initiate M-Pesa Payment

Start the M-Pesa STK Push process for VIP subscription.

**Endpoint:** `POST /pay/mpesa/initiate`

**Request Body:**
```json
{
  "user_id": "507f1f77bcf86cd799439011",
  "phone_number": "254712345678"
}
```

**Field Descriptions:**
| Field | Type | Required | Description |
|-------|------|----------|-------------|
| user_id | String | Yes | MongoDB ObjectId of the user |
| phone_number | String | Yes | Kenyan phone number (254XXXXXXXXX) |

**Example Request:**
```bash
curl -X POST http://localhost:8080/pay/mpesa/initiate \
  -H "Content-Type: application/json" \
  -d '{
    "user_id": "507f1f77bcf86cd799439011",
    "phone_number": "254712345678"
  }'
```

**Response (200 OK):**
```json
{
  "checkout_request_id": "ws_CO_15012024103045678901234567890",
  "message": "Check your phone for M-Pesa prompt"
}
```

**Error Responses:**

`400 Bad Request` - Invalid user_id
```json
{
  "error": "Invalid user_id"
}
```

`404 Not Found` - User doesn't exist
```json
{
  "error": "User not found"
}
```

`500 Internal Server Error` - M-Pesa API error
```json
{
  "error": "M-Pesa error: Connection timeout"
}
```

---

### 4. M-Pesa Callback

Webhook endpoint called by Safaricom after payment completion.

**Endpoint:** `POST /pay/mpesa/callback`

**Note:** This endpoint is called by Safaricom's servers, not by your frontend.

**Request Body (Success):**
```json
{
  "Body": {
    "stkCallback": {
      "MerchantRequestID": "29115-34620561-1",
      "CheckoutRequestID": "ws_CO_15012024103045678901234567890",
      "ResultCode": 0,
      "ResultDesc": "The service request is processed successfully.",
      "CallbackMetadata": {
        "Item": [
          {
            "Name": "Amount",
            "Value": 100
          },
          {
            "Name": "MpesaReceiptNumber",
            "Value": "NLJ7RT61SV"
          },
          {
            "Name": "TransactionDate",
            "Value": 20240115103045
          },
          {
            "Name": "PhoneNumber",
            "Value": 254712345678
          }
        ]
      }
    }
  }
}
```

**Request Body (Failed):**
```json
{
  "Body": {
    "stkCallback": {
      "MerchantRequestID": "29115-34620561-1",
      "CheckoutRequestID": "ws_CO_15012024103045678901234567890",
      "ResultCode": 1032,
      "ResultDesc": "Request cancelled by user"
    }
  }
}
```

**Response:**
```json
{
  "ResultCode": 0,
  "ResultDesc": "Accepted"
}
```

**What Happens:**
1. Payment record updated in database
2. If successful (ResultCode = 0):
   - User's `is_vip` set to `true`
   - `vip_expires_at` set to 24 hours from now
   - Payment status set to "Completed"
3. If failed:
   - Payment status set to "Failed"

---

## Data Models

### User
```typescript
{
  _id: ObjectId,
  phone_number: string,
  email?: string,
  is_vip: boolean,
  vip_expires_at?: Date,
  created_at: Date,
  updated_at: Date
}
```

### Match
```typescript
{
  _id: ObjectId,
  fixture_id: number,
  home_team: TeamStats,
  away_team: TeamStats,
  match_date: Date,
  league: string,
  prediction?: Prediction,
  created_at: Date
}
```

### TeamStats
```typescript
{
  team_id: number,
  team_name: string,
  goals_scored_avg: number,
  goals_conceded_avg: number,
  attack_strength: number,
  defense_strength: number
}
```

### Prediction
```typescript
{
  home_win_prob: number,    // Percentage (0-100)
  draw_prob: number,         // Percentage (0-100)
  away_win_prob: number,     // Percentage (0-100)
  over_2_5_prob: number,     // Percentage (0-100)
  confidence: number,        // Percentage (0-100)
  is_premium: boolean        // true if confidence > 75%
}
```

### Payment
```typescript
{
  _id: ObjectId,
  user_id: ObjectId,
  phone_number: string,
  amount: number,
  mpesa_receipt?: string,
  checkout_request_id: string,
  status: "Pending" | "Completed" | "Failed",
  created_at: Date,
  updated_at: Date
}
```

---

## Error Codes

| HTTP Status | Description |
|-------------|-------------|
| 200 | Success |
| 400 | Bad Request - Invalid parameters |
| 404 | Not Found - Resource doesn't exist |
| 500 | Internal Server Error - Server/database error |

---

## Rate Limiting

Currently no rate limiting is implemented. For production, consider:
- 100 requests per minute per IP
- 1000 requests per day per user

---

## CORS

CORS is configured to allow all origins in development. For production, restrict to your frontend domain:

```rust
let cors = Cors::default()
    .allowed_origin("https://yourdomain.com")
    .allowed_methods(vec!["GET", "POST"])
    .allowed_headers(vec![header::AUTHORIZATION, header::CONTENT_TYPE])
    .max_age(3600);
```

---

## Testing with cURL

### Get Matches
```bash
curl "http://localhost:8080/matches/today?user_id=507f1f77bcf86cd799439011"
```

### Initiate Payment
```bash
curl -X POST http://localhost:8080/pay/mpesa/initiate \
  -H "Content-Type: application/json" \
  -d '{
    "user_id": "507f1f77bcf86cd799439011",
    "phone_number": "254712345678"
  }'
```

### Test M-Pesa Callback (Simulate)
```bash
curl -X POST http://localhost:8080/pay/mpesa/callback \
  -H "Content-Type: application/json" \
  -d '{
    "Body": {
      "stkCallback": {
        "MerchantRequestID": "test-123",
        "CheckoutRequestID": "ws_CO_test123",
        "ResultCode": 0,
        "ResultDesc": "Success",
        "CallbackMetadata": {
          "Item": [
            {"Name": "MpesaReceiptNumber", "Value": "TEST123"}
          ]
        }
      }
    }
  }'
```

---

## Postman Collection

Import this JSON into Postman for easy testing:

```json
{
  "info": {
    "name": "BetSure Analytics API",
    "schema": "https://schema.getpostman.com/json/collection/v2.1.0/collection.json"
  },
  "item": [
    {
      "name": "Get Today's Matches",
      "request": {
        "method": "GET",
        "header": [],
        "url": {
          "raw": "http://localhost:8080/matches/today?user_id=507f1f77bcf86cd799439011",
          "protocol": "http",
          "host": ["localhost"],
          "port": "8080",
          "path": ["matches", "today"],
          "query": [
            {
              "key": "user_id",
              "value": "507f1f77bcf86cd799439011"
            }
          ]
        }
      }
    },
    {
      "name": "Initiate Payment",
      "request": {
        "method": "POST",
        "header": [
          {
            "key": "Content-Type",
            "value": "application/json"
          }
        ],
        "body": {
          "mode": "raw",
          "raw": "{\n  \"user_id\": \"507f1f77bcf86cd799439011\",\n  \"phone_number\": \"254712345678\"\n}"
        },
        "url": {
          "raw": "http://localhost:8080/pay/mpesa/initiate",
          "protocol": "http",
          "host": ["localhost"],
          "port": "8080",
          "path": ["pay", "mpesa", "initiate"]
        }
      }
    }
  ]
}
```

---

## WebSocket Support (Future)

For real-time updates, consider adding WebSocket support:

```rust
// Future endpoint
ws://localhost:8080/ws/matches
```

This would push live updates when:
- New predictions are generated
- Match results come in
- VIP status changes

---

## Versioning

Current version: `v1`

Future versions will be prefixed:
- `/api/v1/matches/today`
- `/api/v2/matches/today`

---

## Support

For API issues or questions:
- GitHub Issues: [github.com/yourusername/betsure-analytics/issues](https://github.com)
- Email: support@yourdomain.com
