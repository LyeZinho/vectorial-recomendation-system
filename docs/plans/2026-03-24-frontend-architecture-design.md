# Frontend Architecture Design - 2026-03-24

## Executive Summary

Build a three-tier system: **SvelteKit frontend** + **NestJS bridge** + **Rust engine**. The NestJS bridge acts as a smart adapter layer that transforms Rust API responses, manages user state (PostgreSQL), and handles business logic specific to the frontend experience.

---

## Overall System Architecture

```
SvelteKit Frontend (Port 5173)
    ↓ HTTP/REST API calls
NestJS Bridge (Port 3001)
    ├→ PostgreSQL (User data, profiles, preferences, watchlist)
    └→ Rust Axum API (Port 3000 - recommendations, search, explain)
        └→ Neo4j (Anime graph data)
    └→ Redis (Caching layer)
```

### Design Principles

1. **Separation of Concerns** - Rust handles graph analytics, NestJS handles user context, SvelteKit handles UI
2. **No Rust Modifications** - Backend-for-Frontend pattern keeps Rust API untouched
3. **Smart Caching** - Responses cached at multiple levels (Redis, browser)
4. **Type Safety** - TypeScript throughout NestJS and SvelteKit
5. **Scalability** - Each service independently deployable in Docker

---

## Project Structure (Nested Workspaces)

```
vectorial-recommendation-system/
├── services/
│   ├── rust-engine/
│   │   ├── src/
│   │   ├── Cargo.toml
│   │   ├── docker-compose.yml
│   │   └── Dockerfile
│   │
│   ├── nestjs-bridge/              (NEW)
│   │   ├── src/
│   │   │   ├── main.ts             (Entry point, port 3001)
│   │   │   ├── app.module.ts
│   │   │   ├── auth/               (JWT, session, login/register)
│   │   │   │   ├── auth.module.ts
│   │   │   │   ├── auth.service.ts
│   │   │   │   ├── jwt.strategy.ts
│   │   │   │   ├── auth.controller.ts
│   │   │   │   └── entities/
│   │   │   ├── users/              (User profiles, preferences)
│   │   │   │   ├── users.module.ts
│   │   │   │   ├── users.service.ts
│   │   │   │   ├── users.controller.ts
│   │   │   │   └── entities/user.entity.ts
│   │   │   ├── recommendations/    (Rust API aggregation)
│   │   │   │   ├── recommendations.module.ts
│   │   │   │   ├── recommendations.service.ts
│   │   │   │   ├── recommendations.controller.ts
│   │   │   │   └── dtos/
│   │   │   ├── search/             (Search enrichment)
│   │   │   │   ├── search.module.ts
│   │   │   │   ├── search.service.ts
│   │   │   │   └── search.controller.ts
│   │   │   ├── watchlist/          (Saved anime)
│   │   │   │   ├── watchlist.module.ts
│   │   │   │   ├── watchlist.service.ts
│   │   │   │   └── watchlist.controller.ts
│   │   │   ├── admin/              (Proxy admin endpoints)
│   │   │   │   ├── admin.module.ts
│   │   │   │   ├── admin.service.ts
│   │   │   │   └── admin.controller.ts
│   │   │   ├── config/             (Environment, database)
│   │   │   └── common/             (Guards, filters, interceptors)
│   │   ├── package.json
│   │   ├── tsconfig.json
│   │   ├── .env.example
│   │   ├── docker-compose.yml
│   │   └── Dockerfile
│   │
│   └── sveltekit-frontend/         (NEW)
│       ├── src/
│       │   ├── app.html
│       │   ├── routes/
│       │   │   ├── +page.svelte    (Home)
│       │   │   ├── login/
│       │   │   │   └── +page.svelte
│       │   │   ├── discovery/
│       │   │   │   └── +page.svelte
│       │   │   ├── recommendations/
│       │   │   │   └── +page.svelte
│       │   │   ├── profile/
│       │   │   │   └── +page.svelte
│       │   │   ├── admin/          (Admin dashboard)
│       │   │   │   └── +page.svelte
│       │   │   └── +layout.svelte  (Navigation layout)
│       │   ├── components/
│       │   │   ├── Header.svelte
│       │   │   ├── Footer.svelte
│       │   │   ├── SearchBar.svelte
│       │   │   ├── AnimeCard.svelte
│       │   │   ├── RecommendationCard.svelte
│       │   │   └── WatchlistButton.svelte
│       │   ├── stores/             (Svelte stores)
│       │   │   ├── auth.ts         (Auth state)
│       │   │   ├── user.ts         (User profile)
│       │   │   └── watchlist.ts    (Saved anime)
│       │   ├── lib/
│       │   │   ├── api.ts          (NestJS API client)
│       │   │   ├── auth.ts         (Auth helpers)
│       │   │   └── types.ts        (TypeScript interfaces)
│       │   └── app.css             (Tailwind + neobrutalist design)
│       ├── static/
│       ├── package.json
│       ├── svelte.config.js
│       ├── vite.config.ts
│       ├── tsconfig.json
│       ├── tailwind.config.js
│       ├── .env.example
│       ├── Dockerfile
│       └── docker-compose.yml
│
├── docs/
│   └── plans/
│       └── 2026-03-24-frontend-architecture-design.md (THIS FILE)
└── docker-compose.yml             (Root orchestration - all services)
```

---

## Frontend Pages & Features (SvelteKit)

### Page Structure

| Route | Purpose | Auth Required | Features |
|-------|---------|---------------|----------|
| `/` | Home page | No | Hero, trending anime, featured |
| `/login` | Authentication | No | Login/register form, JWT handling |
| `/discovery` | Search & browse | Yes | Search, filters, fuzzy matching, pagination |
| `/recommendations` | Personalized recs | Yes | ML-based recommendations, refresh |
| `/profile` | User settings | Yes | Preferences, saved items, watchlist |
| `/admin` | Admin dashboard | Admin only | System stats, cache management |

### Design System

**Colors (Neobrutalist):**
- Primary: `#FFFF00` (Bright yellow)
- Secondary: `#9D4EDD` (Deep purple)
- Background: `#000000` (Pure black)
- Text: `#FFFFFF` (White)

**Typography:**
- Bold, geometric fonts
- High contrast
- Minimal decoration

**Layout:**
- Full-width containers
- Strong grid system
- Stark whitespace

---

## NestJS Bridge Architecture

### Core Modules

#### 1. **Auth Module**
- JWT token generation/validation
- Login/register endpoints
- Session management
- Password hashing (bcrypt)
- Role-based access control (user/admin)

**Endpoints:**
```
POST   /auth/login          → Generate JWT tokens
POST   /auth/register       → Create new user
POST   /auth/refresh        → Refresh access token
GET    /auth/me             → Get current user
POST   /auth/logout         → Invalidate session
```

#### 2. **Users Module**
- User profile CRUD
- Preferences (theme, language, etc.)
- Settings management
- Database: PostgreSQL via TypeORM

**Endpoints:**
```
GET    /users/:id           → Get user profile
PUT    /users/:id           → Update profile
GET    /users/:id/preferences → Get user prefs
PUT    /users/:id/preferences → Update prefs
DELETE /users/:id           → Delete account
```

#### 3. **Recommendations Module**
- Proxy calls to Rust API: `GET /api/recommendations/:id`
- Enrich response with user context (watchlist status, saved items)
- Cache aggregated results (Redis, 10min TTL)
- Transform response shape for frontend

**Endpoints:**
```
GET    /recommendations/:id → Get recommendations (enriched)
POST   /recommendations/:id/refresh → Clear cache, fetch fresh
```

**Data Flow:**
```
Frontend: GET /recommendations/123
NestJS:
  1. Load user from PostgreSQL
  2. Load user's watchlist
  3. Call Rust: GET /api/recommendations/123
  4. Merge: Mark items in watchlist
  5. Cache result (10min)
  6. Return transformed response
```

#### 4. **Search Module**
- Proxy to Rust: `GET /api/search?q=query`
- Add user-specific filters (watched status)
- Cache results (Redis, 5min TTL)

**Endpoints:**
```
GET    /search?q=query     → Search anime (fuzzy, cached)
GET    /search/suggestions?q=prefix → Autocomplete suggestions
```

#### 5. **Watchlist Module**
- CRUD operations for saved anime
- Track watch status (watching, completed, dropped, planned)
- Database: PostgreSQL

**Endpoints:**
```
GET    /watchlist                      → Get user's watchlist
POST   /watchlist/:animeId             → Add to watchlist
PUT    /watchlist/:animeId             → Update status
DELETE /watchlist/:animeId             → Remove from watchlist
GET    /watchlist/:animeId/status      → Check if saved
```

#### 6. **Admin Module**
- Proxy admin endpoints from Rust with auth check
- System statistics dashboard
- Cache invalidation control

**Endpoints:**
```
GET    /admin/stats                    → System statistics (admin-only)
POST   /admin/cache/invalidate         → Clear cache patterns (admin-only)
POST   /admin/anime/update             → Update anime data (admin-only)
GET    /admin/users                    → List users (admin-only)
```

---

## Data Models (PostgreSQL via TypeORM)

### User Entity
```typescript
{
  id: UUID (PK)
  email: string (unique)
  username: string (unique)
  password_hash: string (bcrypt)
  role: 'user' | 'admin'
  profile: {
    avatar_url?: string
    bio?: string
    favorite_genre?: string
  }
  preferences: {
    theme: 'light' | 'dark'
    language: string
    notifications_enabled: boolean
  }
  created_at: Date
  updated_at: Date
  is_deleted: boolean
}
```

### Watchlist Entity
```typescript
{
  id: UUID (PK)
  user_id: UUID (FK)
  anime_id: number (from Neo4j)
  status: 'watching' | 'completed' | 'dropped' | 'planned'
  rating?: number (1-10)
  notes?: string
  added_at: Date
  updated_at: Date
}
```

---

## Authentication Flow

### Login Flow (SvelteKit Frontend → NestJS → Database)

```
1. User submits login form (email, password)
2. SvelteKit sends: POST /auth/login
3. NestJS:
   - Lookup user in PostgreSQL
   - Verify password (bcrypt)
   - Generate JWT tokens (access + refresh)
   - Return tokens to frontend
4. SvelteKit:
   - Store access token in memory
   - Store refresh token in httpOnly cookie
   - Set auth state in store
   - Redirect to /recommendations
5. Subsequent requests include Bearer token
6. NestJS validates JWT before processing
```

### Protected Endpoints

All endpoints except `/auth/login`, `/auth/register`, `/` are protected:
- Require valid JWT in `Authorization: Bearer <token>` header
- NestJS guard validates token before route handler executes
- Expired tokens → 401 Unauthorized → Frontend redirects to login
- Refresh token used to get new access token → 403 triggers refresh flow

---

## Integration with Rust API

### Call Pattern

NestJS doesn't reimplement Rust functionality. Instead:
1. **Recommendations** - Call Rust, enrich with watchlist status
2. **Search** - Call Rust, add user filters
3. **Admin** - Proxy Rust admin endpoints with NestJS auth check

### Example: Get Recommendations

```typescript
// NestJS RecommendationsService
async getRecommendations(userId: string, animeId: number) {
  // Load user and watchlist
  const user = await this.usersService.findById(userId);
  const watchlist = await this.watchlistService.findByUser(userId);
  
  // Call Rust API
  const response = await this.httpClient.get(
    `http://rust-api:3000/api/recommendations/${animeId}`
  ).toPromise();
  
  // Enrich with user context
  const enriched = response.results.map(anime => ({
    ...anime,
    isInWatchlist: watchlist.some(w => w.anime_id === anime.id),
    userRating: watchlist.find(w => w.anime_id === anime.id)?.rating,
  }));
  
  // Cache and return
  await this.cacheManager.set(`rec:${animeId}:${userId}`, enriched, 600);
  return enriched;
}
```

---

## Caching Strategy

### Multi-Level Caching

1. **Browser Cache** - HTTP caching headers (recommendations: 5min)
2. **Redis Cache** (NestJS) - Aggregated responses (10min for recommendations, 5min for search)
3. **Database Queries** - Indexed lookups for users/watchlist

### Cache Invalidation

- Search: Invalidated after 5 minutes
- Recommendations: Invalidated after 10 minutes OR when user adds to watchlist
- Admin endpoint: Manual invalidation via `/admin/cache/invalidate`

---

## Error Handling

### Frontend Error States

1. **Network Error** - Retry button, fallback UI
2. **Auth Error (401)** - Redirect to login
3. **Forbidden (403)** - Admin-only content message
4. **Not Found (404)** - "Anime not found" message
5. **Server Error (5xx)** - Retry banner with contact support link

### NestJS Error Handling

- Custom exception filters for consistent error responses
- Validation errors return 400 with field-level details
- Unhandled exceptions return 500 with error ID for debugging
- All errors logged with context (userId, endpoint, timestamp)

---

## Testing Strategy

### Frontend (SvelteKit)

- **Unit Tests:** Component logic, stores, API client
- **E2E Tests:** User flows (login → search → add watchlist)
- **Visual Tests:** Neobrutalist design consistency

### NestJS Backend

- **Unit Tests:** Services, guards, filters
- **Integration Tests:** Database queries, Rust API calls
- **Auth Tests:** JWT validation, role-based access

### System Tests

- All three services running in Docker
- Frontend → NestJS → Rust → Neo4j full flow
- Cache invalidation works end-to-end

---

## Deployment Architecture

### Development (Docker Compose)

```yaml
services:
  rust-engine:
    build: services/rust-engine
    ports: 3000:3000
    depends_on: neo4j, redis
  
  nestjs-bridge:
    build: services/nestjs-bridge
    ports: 3001:3000
    depends_on: rust-engine, postgres, redis
    env:
      RUST_API_URL: http://rust-engine:3000
      DATABASE_URL: postgres://...
      REDIS_URL: redis://redis:6379
  
  sveltekit-frontend:
    build: services/sveltekit-frontend
    ports: 5173:5173
    depends_on: nestjs-bridge
    env:
      PUBLIC_API_URL: http://nestjs-bridge:3001
  
  postgres:
    image: postgres:15
    volumes: postgres_data:/var/lib/postgresql/data
  
  redis:
    image: redis:7-alpine
  
  neo4j:
    image: neo4j:5.19
```

### Production Considerations

- Use managed PostgreSQL (AWS RDS, Google Cloud SQL)
- Use managed Redis (AWS ElastiCache)
- Use managed Neo4j (Neo4j Cloud)
- Frontend deployed to CDN (Vercel, Netlify)
- NestJS deployed to container registry (Docker Hub, ECR)
- Rust API remains on current infrastructure
- API Gateway or reverse proxy in front (nginx, Traefik)

---

## Success Criteria

✅ **Phase 1:** SvelteKit scaffold + NestJS scaffold running together  
✅ **Phase 2:** User authentication flow working (register → login → protected pages)  
✅ **Phase 3:** Search and recommendations connected (frontend calls NestJS → Rust → results)  
✅ **Phase 4:** Watchlist feature working (save/unsave anime, persist in PostgreSQL)  
✅ **Phase 5:** Admin dashboard accessible (stats, cache management)  
✅ **Phase 6:** All three services running in Docker Compose with proper health checks  
✅ **Phase 7:** Full E2E test coverage for critical user flows  

---

## Known Limitations & Future Work

### Phase 6 Scope
- User authentication (JWT, bcrypt) ✓
- Basic watchlist ✓
- Search integration ✓
- Recommendations enrichment ✓

### Phase 7+ Scope (Not in Phase 6)
- Social features (follow users, share lists)
- Advanced user preferences (genre weights, seasonal preferences)
- Real-time notifications (WebSocket)
- Recommendation algorithm tuning
- Analytics dashboard
- CORS configuration for production
- Request logging & distributed tracing
- OpenAPI/Swagger documentation

---

**Status: Design Complete - Ready for Implementation Planning**

Generated: 2026-03-24
Architecture: Three-tier (SvelteKit + NestJS + Rust)
