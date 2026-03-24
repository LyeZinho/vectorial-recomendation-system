# Development Guide

## Quick Start

### Prerequisites
- Node.js 20+
- Docker & Docker Compose
- Git

### Setup

1. **Clone and install dependencies**
```bash
git clone https://github.com/your-org/vectorial-recommendation-system.git
cd vectorial-recommendation-system
npm install
```

2. **Start all services with Docker Compose**
```bash
docker-compose up -d
```

This starts:
- PostgreSQL (port 5432) - user data
- Redis (port 6379) - caching
- Neo4j (port 7474, 7687) - recommendation graph
- Rust API (port 3000) - recommendation engine
- NestJS Bridge (port 3001) - BFF layer
- SvelteKit Frontend (port 5173) - UI

3. **Verify services are healthy**
```bash
docker-compose ps
```

All services should show `healthy` status.

4. **Access the application**
- Frontend: http://localhost:5173
- API Docs: http://localhost:3001/api (if configured)
- Admin Dashboard: http://localhost:3001/admin (requires admin role)

---

## Development Workflow

### Working on Frontend

```bash
cd services/sveltekit-frontend
npm run dev
```

Frontend will hot-reload at http://localhost:5173

**Key files:**
- `src/routes/` - Pages (login, recommendations, discovery, profile, admin)
- `src/stores/` - State management (auth, recommendations)
- `src/components/` - Reusable UI (AnimeCard, SearchBar)
- `src/lib/api.ts` - API client with JWT auto-inject

### Working on Backend (NestJS)

```bash
cd services/nestjs-bridge
npm run dev
```

Backend will hot-reload on file changes.

**Key modules:**
- `auth/` - Authentication (JWT, register, login)
- `users/` - User management and profiles
- `recommendations/` - Fetch recommendations from Rust API
- `watchlist/` - User watchlist CRUD
- `search/` - Search with filters and pagination
- `admin/` - System stats and user management

### Database

**PostgreSQL (User Data):**
```bash
docker exec -it anime-postgres psql -U postgres -d anime_bridge
```

**Neo4j (Recommendation Graph):**
- UI: http://localhost:7474
- Username: neo4j
- Password: password

**Redis (Cache):**
```bash
docker exec -it anime-redis redis-cli
```

---

## API Endpoints

### Authentication
- `POST /auth/register` - Create account
- `POST /auth/login` - Login
- `POST /auth/refresh` - Refresh JWT
- `GET /auth/me` - Get current user (requires JWT)

### Users
- `GET /users/me` - Get current user profile
- `GET /users/:id` - Get user by ID
- `PUT /users/:id` - Update user profile
- `DELETE /users/:id` - Delete user

### Recommendations
- `GET /recommendations/:id` - Get recommendations for user (from Rust API)

### Watchlist
- `POST /watchlist` - Add to watchlist
- `GET /watchlist` - Get user watchlist (supports ?status=watching|completed|dropped|planned)
- `PUT /watchlist/:id` - Update watchlist status
- `DELETE /watchlist/:id` - Remove from watchlist

### Search
- `GET /search?q=query&genre=Action&year=2023&offset=0&limit=20` - Search anime
- `GET /search/genres` - Get available genres
- `GET /search/years` - Get available years

### Admin (requires admin role)
- `GET /admin/stats` - System statistics
- `GET /admin/users?limit=50&offset=0` - List users
- `PUT /admin/users/:id/role` - Update user role
- `DELETE /admin/users/:id` - Delete user

---

## Environment Variables

### SvelteKit Frontend (.env)
```
PUBLIC_API_URL=http://localhost:3001
```

### NestJS Backend (.env)
```
PORT=3001
NODE_ENV=development
DATABASE_HOST=localhost
DATABASE_PORT=5432
DATABASE_USER=postgres
DATABASE_PASSWORD=password
DATABASE_NAME=anime_bridge
JWT_SECRET=nestjs-dev-secret-key-min-32-chars-long
JWT_EXPIRATION=3600
RUST_API_URL=http://localhost:3000
REDIS_URL=redis://localhost:6379
FRONTEND_URL=http://localhost:5173
```

---

## Testing

### NestJS Unit Tests
```bash
cd services/nestjs-bridge
npm run test
```

### NestJS Integration Tests
```bash
npm run test:e2e
```

### SvelteKit Tests
```bash
cd services/sveltekit-frontend
npm run test
```

---

## Build for Production

```bash
# Build all services
npm run build

# Or individually
cd services/nestjs-bridge && npm run build
cd services/sveltekit-frontend && npm run build
```

---

## Troubleshooting

### Services won't start
```bash
# Clean up old containers
docker-compose down -v
docker-compose up -d
```

### Database connection errors
```bash
# Check PostgreSQL logs
docker logs anime-postgres

# Verify it's healthy
docker-compose ps | grep postgres
```

### API returns 401
- Check your JWT token is in localStorage
- Verify token hasn't expired (1 hour default)
- Try refreshing the token: `POST /auth/refresh`

### Frontend can't connect to API
- Verify NestJS is running: `curl http://localhost:3001/`
- Check `PUBLIC_API_URL` in frontend .env
- Check CORS configuration in NestJS

---

## Architecture

```
User Browser
    ↓
SvelteKit Frontend (5173)
    ↓
NestJS Bridge (3001) ← PostgreSQL (5432)
    ↓
Rust Engine (3000) ← Neo4j (7474)
                   ← Redis (6379)
```

**Data Flow:**
1. User logs in → NestJS creates JWT → stored in localStorage
2. Frontend sends JWT in Authorization header
3. NestJS validates JWT, fetches user from PostgreSQL
4. For recommendations/search: NestJS calls Rust Engine, caches in Redis
5. User watchlist: NestJS stores in PostgreSQL

---

## Code Style

- **TypeScript**: strict mode (relaxed nullChecks for NestJS compat)
- **Svelte**: reactive stores, scoped styling
- **Tailwind CSS**: utility-first, neobrutalist theme (yellow/purple/black)
- **Naming**: camelCase for vars/functions, PascalCase for classes/components

---

## Next Steps

- See DEPLOYMENT.md for production setup
- Check docs/plans/ for feature specifications
- Run tests before committing: `npm run test`
