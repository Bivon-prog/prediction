# 🚀 Deployment Guide - BetSure Analytics

## Production Deployment Checklist

### 1. Server Requirements

**Minimum Specs:**
- 2 CPU cores
- 4GB RAM
- 40GB SSD
- Ubuntu 22.04 LTS

**Recommended Providers:**
- DigitalOcean (Droplet $24/month)
- Linode (Nanode 4GB)
- AWS EC2 (t3.medium)
- Hetzner Cloud (CX21)

### 2. Backend Deployment (Rust API)

#### Install Dependencies
```bash
# Update system
sudo apt update && sudo apt upgrade -y

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Install MongoDB
wget -qO - https://www.mongodb.org/static/pgp/server-7.0.asc | sudo apt-key add -
echo "deb [ arch=amd64,arm64 ] https://repo.mongodb.org/apt/ubuntu jammy/mongodb-org/7.0 multiverse" | sudo tee /etc/apt/sources.list.d/mongodb-org-7.0.list
sudo apt update
sudo apt install -y mongodb-org
sudo systemctl start mongod
sudo systemctl enable mongod
```

#### Build and Deploy
```bash
# Clone repository
git clone https://github.com/yourusername/betsure-analytics.git
cd betsure-analytics/backend

# Configure environment
cp .env.example .env
nano .env  # Edit with production values

# Build release binary
cargo build --release

# Create systemd service
sudo nano /etc/systemd/system/betsure-api.service
```

**Service File:**
```ini
[Unit]
Description=BetSure Analytics API
After=network.target mongod.service

[Service]
Type=simple
User=ubuntu
WorkingDirectory=/home/ubuntu/betsure-analytics/backend
Environment="RUST_LOG=info"
ExecStart=/home/ubuntu/betsure-analytics/backend/target/release/server
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
```

```bash
# Start service
sudo systemctl daemon-reload
sudo systemctl start betsure-api
sudo systemctl enable betsure-api
sudo systemctl status betsure-api
```

#### Setup Cron Job
```bash
# Add to crontab
crontab -e

# Add this line (runs daily at 6 AM EAT)
0 3 * * * /home/ubuntu/betsure-analytics/backend/target/release/cron_job >> /var/log/betsure-cron.log 2>&1
```

### 3. Frontend Deployment (Next.js)

#### Option A: Vercel (Recommended - Free Tier)
```bash
cd frontend
npm install -g vercel
vercel login
vercel --prod
```

Set environment variable in Vercel dashboard:
- `NEXT_PUBLIC_API_URL=https://api.yourdomain.com`

#### Option B: Self-Hosted with PM2
```bash
# Install Node.js
curl -fsSL https://deb.nodesource.com/setup_20.x | sudo -E bash -
sudo apt install -y nodejs

# Install PM2
sudo npm install -g pm2

# Build and start
cd frontend
npm install
npm run build
pm2 start npm --name "betsure-frontend" -- start
pm2 startup
pm2 save
```

### 4. Nginx Reverse Proxy

```bash
sudo apt install -y nginx certbot python3-certbot-nginx

# Create Nginx config
sudo nano /etc/nginx/sites-available/betsure
```

**Nginx Configuration:**
```nginx
# API Server
server {
    listen 80;
    server_name api.yourdomain.com;

    location / {
        proxy_pass http://127.0.0.1:8080;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host $host;
        proxy_cache_bypass $http_upgrade;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
    }
}

# Frontend (if self-hosted)
server {
    listen 80;
    server_name yourdomain.com www.yourdomain.com;

    location / {
        proxy_pass http://127.0.0.1:3000;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host $host;
        proxy_cache_bypass $http_upgrade;
    }
}
```

```bash
# Enable site
sudo ln -s /etc/nginx/sites-available/betsure /etc/nginx/sites-enabled/
sudo nginx -t
sudo systemctl restart nginx

# Setup SSL
sudo certbot --nginx -d api.yourdomain.com -d yourdomain.com -d www.yourdomain.com
```

### 5. M-Pesa Production Setup

1. **Apply for Production Credentials**
   - Go to [Daraja Portal](https://developer.safaricom.co.ke/)
   - Create production app
   - Submit for approval (takes 2-3 days)

2. **Configure Callback URL**
   - Must be HTTPS
   - Example: `https://api.yourdomain.com/pay/mpesa/callback`
   - Whitelist your server IP in Daraja portal

3. **Update Environment**
```bash
MPESA_BASE_URL=https://api.safaricom.co.ke
MPESA_CONSUMER_KEY=<production_key>
MPESA_CONSUMER_SECRET=<production_secret>
MPESA_SHORTCODE=<your_paybill>
MPESA_PASSKEY=<production_passkey>
MPESA_CALLBACK_URL=https://api.yourdomain.com/pay/mpesa/callback
```

### 6. API-Football Setup

1. **Get API Key**
   - Sign up at [API-Football](https://www.api-football.com/)
   - Free tier: 100 requests/day
   - Pro tier: $15/month (unlimited)

2. **Configure**
```bash
API_FOOTBALL_BASE_URL=https://v3.football.api-sports.io
API_FOOTBALL_KEY=<your_api_key>
```

### 7. Database Optimization

```javascript
// Connect to MongoDB
mongosh

use betsure

// Create indexes
db.matches.createIndex({ "match_date": 1 })
db.matches.createIndex({ "fixture_id": 1 }, { unique: true })
db.users.createIndex({ "phone_number": 1 }, { unique: true })
db.payments.createIndex({ "checkout_request_id": 1 })
db.payments.createIndex({ "user_id": 1 })

// Enable authentication (production)
db.createUser({
  user: "betsure_admin",
  pwd: "strong_password_here",
  roles: [{ role: "readWrite", db: "betsure" }]
})
```

Update MongoDB URI:
```bash
MONGODB_URI=mongodb://betsure_admin:strong_password_here@localhost:27017/betsure?authSource=betsure
```

### 8. Monitoring & Logging

#### Setup Log Rotation
```bash
sudo nano /etc/logrotate.d/betsure
```

```
/var/log/betsure-cron.log {
    daily
    rotate 7
    compress
    delaycompress
    missingok
    notifempty
}
```

#### Monitor Services
```bash
# Check API status
sudo systemctl status betsure-api

# View logs
sudo journalctl -u betsure-api -f

# Check cron logs
tail -f /var/log/betsure-cron.log

# Monitor PM2 (if using)
pm2 logs betsure-frontend
pm2 monit
```

### 9. Firewall Configuration

```bash
# Enable UFW
sudo ufw allow 22/tcp
sudo ufw allow 80/tcp
sudo ufw allow 443/tcp
sudo ufw enable

# MongoDB should only be accessible locally
sudo ufw deny 27017
```

### 10. Backup Strategy

```bash
# Create backup script
nano ~/backup.sh
```

```bash
#!/bin/bash
DATE=$(date +%Y%m%d_%H%M%S)
BACKUP_DIR="/home/ubuntu/backups"

# Create backup directory
mkdir -p $BACKUP_DIR

# Backup MongoDB
mongodump --db betsure --out $BACKUP_DIR/mongo_$DATE

# Compress
tar -czf $BACKUP_DIR/betsure_backup_$DATE.tar.gz $BACKUP_DIR/mongo_$DATE

# Remove old backups (keep last 7 days)
find $BACKUP_DIR -name "betsure_backup_*.tar.gz" -mtime +7 -delete

# Remove uncompressed dump
rm -rf $BACKUP_DIR/mongo_$DATE
```

```bash
chmod +x ~/backup.sh

# Add to crontab (daily at 2 AM)
crontab -e
0 2 * * * /home/ubuntu/backup.sh
```

### 11. Performance Optimization

#### Enable Gzip in Nginx
```nginx
gzip on;
gzip_vary on;
gzip_min_length 1024;
gzip_types text/plain text/css text/xml text/javascript application/json application/javascript application/xml+rss;
```

#### Add Caching Headers
```nginx
location ~* \.(jpg|jpeg|png|gif|ico|css|js)$ {
    expires 1y;
    add_header Cache-Control "public, immutable";
}
```

### 12. Security Hardening

```bash
# Install fail2ban
sudo apt install -y fail2ban

# Configure SSH
sudo nano /etc/ssh/sshd_config
# Set: PermitRootLogin no
# Set: PasswordAuthentication no

# Restart SSH
sudo systemctl restart sshd

# Setup automatic security updates
sudo apt install -y unattended-upgrades
sudo dpkg-reconfigure --priority=low unattended-upgrades
```

### 13. Domain Setup

1. **Purchase Domain** (Namecheap, GoDaddy, etc.)

2. **Configure DNS Records:**
```
Type    Name    Value                   TTL
A       @       <your_server_ip>        3600
A       www     <your_server_ip>        3600
A       api     <your_server_ip>        3600
```

### 14. Health Checks

Create a health endpoint in `backend/src/routes/mod.rs`:

```rust
use actix_web::{get, HttpResponse, Responder};

#[get("/health")]
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now()
    }))
}
```

Setup monitoring with UptimeRobot or similar service.

### 15. Cost Estimation (Monthly)

| Service | Cost |
|---------|------|
| DigitalOcean Droplet (4GB) | $24 |
| Domain Name | $1 |
| API-Football Pro | $15 |
| SSL Certificate | Free (Let's Encrypt) |
| **Total** | **~$40/month** |

### 16. Launch Checklist

- [ ] Backend API running and accessible
- [ ] Frontend deployed and accessible
- [ ] MongoDB secured with authentication
- [ ] SSL certificates installed
- [ ] M-Pesa production credentials configured
- [ ] Cron job running daily
- [ ] Backups configured
- [ ] Monitoring setup
- [ ] Firewall configured
- [ ] DNS records propagated
- [ ] Test payment flow end-to-end
- [ ] Test prediction generation
- [ ] Load testing completed

---

## Quick Start Commands

```bash
# Check all services
sudo systemctl status betsure-api mongod nginx

# Restart everything
sudo systemctl restart betsure-api nginx
pm2 restart betsure-frontend

# View logs
sudo journalctl -u betsure-api -f
tail -f /var/log/betsure-cron.log
pm2 logs betsure-frontend

# Test API
curl https://api.yourdomain.com/health

# Manual cron run
/home/ubuntu/betsure-analytics/backend/target/release/cron_job
```

Good luck with your launch! 🚀
