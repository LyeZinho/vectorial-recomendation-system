# Vectorial Recommendation System

A production-ready three-tier anime recommendation engine combining **SvelteKit** (frontend), **NestJS** (API bridge), and **Rust** (ML engine) with PostgreSQL, Neo4j, and Redis.

## ✨ Features

| Feature | Status | Details |
|---------|--------|---------|
| User Authentication | ✅ | JWT-based auth with refresh tokens |
| Anime Recommendations | ✅ | ML-powered suggestions from Rust engine |
| Search & Discovery | ✅ | Full-text search with genre/year filters |
| Watchlist Management | ✅ | Track watching/completed/dropped/planned |
| User Profiles | ✅ | View watchlist history and stats |
| Admin Dashboard | ✅ | User management and system statistics |
| E2E Tests | ✅ | Playwright test suite for critical flows |
| Docker Deployment | ✅ | Full stack orchestration with health checks |

## 🏗️ Architecture

### Three-Tier Design

```
┌─────────────────────────────────────────────────────────────┐
│ SvelteKit Frontend (Port 5173)                              │
│ - Neobrutalist UI (Yellow #FFFF00, Purple #9D4EDD)         │
│ - Svelte stores for state management                        │
│ - Playwright E2E tests                                       │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│ NestJS Bridge API (Port 3001)                               │
│ - Backend-for-Frontend (BFF) pattern                        │
│ - 7 modules: auth, users, recommendations, watchlist,      │
│   search, admin, app                                         │
│ - JWT-based route protection                                │
│ - Caching layer (Redis)                                     │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│ Rust ML Engine (Port 3000)                                  │
│ - Collaborative filtering recommendations                    │
│ - Anime metadata and search indexing                        │
│ - Neo4j graph database interface                            │
└─────────────────────────────────────────────────────────────┘
```

### Data Layer

- **PostgreSQL** (5432): User accounts, watchlist, auth tokens
- **Neo4j** (7474): Recommendation graph, anime relationships
- **Redis** (6379): Request caching, session store

## 🚀 Quick Start

### Prerequisites

- Docker & Docker Compose
- Node.js 18+ (for local development)
- Rust toolchain (for backend modifications)

### Local Development

```bash
# Clone repository
git clone <repo-url>
cd vectorial-recomendation-system

# Start all services (PostgreSQL, Neo4j, Redis, APIs, Frontend)
docker-compose up -d

# Wait for services to initialize
sleep 20

# Verify health
docker-compose ps

# Access frontend
open http://localhost:5173
```

**Expected Output:**
```
CONTAINER ID   STATUS              PORTS
postgres       healthy             5432
redis          healthy             6379
neo4j          healthy             7474
rust-engine    healthy             3000
nestjs-bridge  healthy             3001
sveltekit      healthy             5173
```

### Local Development (Without Docker)

**NestJS Backend:**
```bash
cd services/nestjs-bridge
npm install
npm run dev
# Server runs on http://localhost:3001
```

**SvelteKit Frontend:**
```bash
cd services/sveltekit-frontend
npm install
npm run dev
# App runs on http://localhost:5173
```

**Rust Engine:** Requires local Rust setup (see DEVELOPMENT.md)

## 📖 Documentation

| Document | Purpose |
|----------|---------|
| [DEVELOPMENT.md](./DEVELOPMENT.md) | Local development setup, API reference, troubleshooting |
| [DEPLOYMENT.md](./DEPLOYMENT.md) | Production deployment, scaling, security hardening |
| [SECURITY-AUDIT.md](./docs/SECURITY-AUDIT.md) | OWASP Top 10 security review and recommendations |
| [Frontend Architecture](./docs/plans/2026-03-24-frontend-architecture-design.md) | SvelteKit/NestJS design decisions |

## 🧪 Testing

### E2E Tests (Playwright)

Test critical user flows: registration → login → search → watchlist → profile

```bash
cd services/sveltekit-frontend
npm run test:e2e

# Run specific test
npx playwright test e2e/auth-flow.spec.ts
```

### Integration Tests (NestJS)

Test API endpoints with real database

```bash
cd services/nestjs-bridge
npm run test

# Watch mode
npm run test:watch
```

### Performance Benchmarking

```bash
cd services/nestjs-bridge

# Install dependencies first
npm install

# Run benchmarks (requires running API on port 3001)
npm run benchmark
```

## 🔌 API Reference

### Authentication

```bash
# Register new user
curl -X POST http://localhost:3001/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "email": "user@example.com",
    "username": "username",
    "password": "SecurePass123!"
  }'

# Login
curl -X POST http://localhost:3001/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "user@example.com",
    "password": "SecurePass123!"
  }'
# Response: { "access_token": "...", "refresh_token": "..." }
```

### Recommendations

```bash
# Get recommendations for user
curl -X GET http://localhost:3001/recommendations/1 \
  -H "Authorization: Bearer <access_token>"
```

### Search

```bash
# Search anime
curl -X GET "http://localhost:3001/search/anime?query=naruto&limit=20" \
  -H "Authorization: Bearer <access_token>"

# Search with filters
curl -X GET "http://localhost:3001/search/anime?query=anime&genre=Action&year=2023&limit=20" \
  -H "Authorization: Bearer <access_token>"
```

### Watchlist

```bash
# Add to watchlist
curl -X POST http://localhost:3001/watchlist \
  -H "Authorization: Bearer <access_token>" \
  -H "Content-Type: application/json" \
  -d '{ "anime_id": 1, "status": "watching" }'

# Update status
curl -X PATCH http://localhost:3001/watchlist/1 \
  -H "Authorization: Bearer <access_token>" \
  -H "Content-Type: application/json" \
  -d '{ "status": "completed" }'
```

### Admin

```bash
# Get system stats (admin only)
curl -X GET http://localhost:3001/admin/stats \
  -H "Authorization: Bearer <admin_token>"

# List all users (admin only)
curl -X GET http://localhost:3001/admin/users \
  -H "Authorization: Bearer <admin_token>"
```

## 🎨 Frontend Features

### Pages

- **Login** (`/login`) — Registration and authentication
- **Recommendations** (`/recommendations`) — ML-powered suggestions
- **Discovery** (`/discovery`) — Full-text search with filters
- **Profile** (`/profile`) — User watchlist and statistics
- **Admin** (`/admin`) — User management and analytics

### Neobrutalist Design

- **Colors:** Yellow (#FFFF00), Purple (#9D4EDD), Black (#000000)
- **Typography:** Bold sans-serif, high contrast
- **Layout:** Minimal, geometric, anti-smooth aesthetic
- **Interactions:** Direct, no animations, instant feedback

## 📊 Performance

### Caching Strategy

- **Redis:** 10-minute TTL on search results and recommendations
- **Database:** Indexed queries on user_id, anime_id, genre
- **Frontend:** Svelte stores for local state management

### Expected Throughput

- Search: ~1000 req/s (see performance-benchmark.js)
- Recommendations: ~500 req/s
- Auth: ~300 req/s

## 🔐 Security

### Implemented

✅ JWT authentication (access + refresh tokens)  
✅ Bcrypt password hashing (10 salt rounds)  
✅ SQL injection prevention (TypeORM parametrized queries)  
✅ Role-based access control (user/admin)  
✅ Protected routes (NestJS guards + SvelteKit hooks)

### Recommended for Production

⚠️ Rate limiting (100 req/minute per IP)  
⚠️ HTTPS/TLS (reverse proxy with SSL)  
⚠️ httpOnly cookies for JWT (replace localStorage)  
⚠️ CORS hardening (restrict to known origins)  
⚠️ Security headers (Helmet.js)

See [SECURITY-AUDIT.md](./docs/SECURITY-AUDIT.md) for full details.

## 🤝 Contributing

Contributions welcome! Follow these guidelines:

1. Create feature branch: `git checkout -b feature/your-feature`
2. Write tests for new functionality
3. Ensure all tests pass: `npm run test`
4. Run linter: `npm run lint`
5. Submit PR with clear description

### Code Style

- TypeScript with strict mode
- Self-documenting code (minimal comments)
- 2-space indentation
- No magic numbers/strings

## 📝 Project Structure

```
vectorial-recomendation-system/
├── services/
│   ├── rust-engine/          # Recommendation ML engine
│   ├── nestjs-bridge/        # Backend-for-Frontend API
│   │   ├── src/
│   │   │   ├── auth/
│   │   │   ├── users/
│   │   │   ├── recommendations/
│   │   │   ├── watchlist/
│   │   │   ├── search/
│   │   │   ├── admin/
│   │   │   └── app.module.ts
│   │   └── test/
│   └── sveltekit-frontend/   # User interface
│       ├── src/
│       │   ├── routes/       # Pages
│       │   ├── components/   # UI components
│       │   ├── stores/       # Svelte stores
│       │   └── lib/api.ts    # API client
│       └── e2e/             # E2E tests
├── docker-compose.yml
├── DEVELOPMENT.md
├── DEPLOYMENT.md
├── README.md (this file)
└── docs/
    ├── SECURITY-AUDIT.md
    └── plans/
```

## 🐛 Troubleshooting

### Services won't start

```bash
# Check logs
docker-compose logs -f

# Restart all
docker-compose restart

# Full reset (WARNING: deletes data)
docker-compose down -v && docker-compose up -d
```

### Port conflicts

If ports 5173, 3001, 3000, 5432, 6379, or 7474 are in use:

Edit `docker-compose.yml` to map different ports:
```yaml
services:
  sveltekit:
    ports:
      - "5174:5173"  # Change left side to your desired port
```

### Database not ready

Services wait 30 seconds for databases. If still failing:

```bash
# Wait longer
sleep 30

# Check health
docker-compose ps

# Recreate services
docker-compose up -d --force-recreate
```

See [DEVELOPMENT.md](./DEVELOPMENT.md#troubleshooting) for more help.

## 📄 License

[LICENSE file here - add as needed]

## 👥 Team

- **Architecture:** AI Agent (Sisyphus)
- **Frontend:** SvelteKit + Vite
- **Backend:** NestJS + TypeORM
- **Engine:** Rust (collaborative filtering)

---

**Last Updated:** March 24, 2026  
**Status:** ✅ Production-Ready (Phase 7 Complete)
