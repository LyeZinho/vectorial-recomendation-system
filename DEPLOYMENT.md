# Deployment Guide

## Production Checklist

- [ ] All environment variables configured
- [ ] Database backups scheduled
- [ ] SSL/TLS certificates installed
- [ ] Rate limiting enabled
- [ ] CORS properly configured
- [ ] Logging and monitoring set up
- [ ] Error tracking (Sentry/similar) configured
- [ ] Performance testing completed
- [ ] Security audit passed

---

## Docker Deployment

### Prerequisites
- Docker & Docker Compose on target server
- Minimum 2GB RAM, 10GB disk
- Port 80/443 for HTTPS
- Reverse proxy (nginx) recommended

### Environment Setup

1. **Create production .env file**

**NestJS Bridge (.env.production):**
```
PORT=3001
NODE_ENV=production
DATABASE_HOST=postgres
DATABASE_PORT=5432
DATABASE_USER=postgres_user
DATABASE_PASSWORD=<STRONG_PASSWORD>
DATABASE_NAME=anime_bridge_prod
JWT_SECRET=<GENERATE_32_CHAR_RANDOM_STRING>
JWT_EXPIRATION=3600
RUST_API_URL=http://rust-engine:3000
REDIS_URL=redis://redis:6379
FRONTEND_URL=https://yourdomain.com
LOG_LEVEL=warn
```

**SvelteKit Frontend (.env.production):**
```
PUBLIC_API_URL=https://api.yourdomain.com
```

2. **Update docker-compose for production**

```yaml
version: '3.8'

services:
  postgres:
    image: postgres:15-alpine
    restart: always
    environment:
      POSTGRES_PASSWORD: ${DATABASE_PASSWORD}
      POSTGRES_DB: ${DATABASE_NAME}
    volumes:
      - postgres_data:/var/lib/postgresql/data
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U postgres"]
      interval: 10s
      retries: 5

  redis:
    image: redis:7-alpine
    restart: always
    command: redis-server --requirepass ${REDIS_PASSWORD}
    healthcheck:
      test: ["CMD", "redis-cli", "ping"]
      interval: 10s
      retries: 5

  neo4j:
    image: neo4j:5.19-community
    restart: always
    environment:
      NEO4J_AUTH: neo4j/${NEO4J_PASSWORD}
      NEO4J_ACCEPT_LICENSE_AGREEMENT: "yes"
    volumes:
      - neo4j_data:/var/lib/neo4j/data

  rust-engine:
    image: anime-rust-engine:latest
    restart: always
    depends_on:
      neo4j:
        condition: service_healthy
    environment:
      NEO4J_URI: neo4j://neo4j:7687
      RUST_LOG: warn

  nestjs-bridge:
    image: anime-nestjs-bridge:latest
    restart: always
    depends_on:
      postgres:
        condition: service_healthy
    environment:
      NODE_ENV: production
      DATABASE_HOST: postgres
      DATABASE_PASSWORD: ${DATABASE_PASSWORD}
      JWT_SECRET: ${JWT_SECRET}
    ports:
      - "3001:3001"

  sveltekit-frontend:
    image: anime-sveltekit-frontend:latest
    restart: always
    depends_on:
      nestjs-bridge:
        condition: service_healthy
    ports:
      - "5173:5173"

  nginx:
    image: nginx:alpine
    restart: always
    ports:
      - "80:80"
      - "443:443"
    volumes:
      - ./nginx.conf:/etc/nginx/nginx.conf:ro
      - ./certs:/etc/nginx/certs:ro
    depends_on:
      - sveltekit-frontend
      - nestjs-bridge

volumes:
  postgres_data:
  neo4j_data:
```

3. **Setup nginx reverse proxy**

Create `nginx.conf`:
```nginx
upstream frontend {
    server sveltekit-frontend:5173;
}

upstream api {
    server nestjs-bridge:3001;
}

server {
    listen 80;
    server_name yourdomain.com;
    return 301 https://$server_name$request_uri;
}

server {
    listen 443 ssl http2;
    server_name yourdomain.com;

    ssl_certificate /etc/nginx/certs/cert.pem;
    ssl_certificate_key /etc/nginx/certs/key.pem;
    ssl_protocols TLSv1.2 TLSv1.3;
    ssl_ciphers HIGH:!aNULL:!MD5;

    # Frontend
    location / {
        proxy_pass http://frontend;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }

    # API
    location /api/ {
        proxy_pass http://api/;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_http_version 1.1;
    }

    # Rate limiting
    limit_req_zone $binary_remote_addr zone=api_limit:10m rate=100r/m;
    location /api/ {
        limit_req zone=api_limit burst=20;
        proxy_pass http://api/;
    }
}
```

### Deployment Steps

1. **Build images**
```bash
docker build -t anime-nestjs-bridge:latest services/nestjs-bridge
docker build -t anime-sveltekit-frontend:latest services/sveltekit-frontend
```

2. **Start services**
```bash
docker-compose -f docker-compose.yml up -d
```

3. **Verify health**
```bash
docker-compose ps
curl https://yourdomain.com
```

4. **Check logs**
```bash
docker-compose logs -f nestjs-bridge
docker-compose logs -f sveltekit-frontend
```

---

## Database Backups

### PostgreSQL Backup (daily)

```bash
#!/bin/bash
# backup.sh
BACKUP_DIR="/backups/postgres"
DATE=$(date +%Y%m%d_%H%M%S)

docker exec anime-postgres pg_dump -U postgres anime_bridge_prod > \
  ${BACKUP_DIR}/backup_${DATE}.sql

# Keep only last 30 days
find ${BACKUP_DIR} -name "backup_*.sql" -mtime +30 -delete
```

Schedule with cron:
```bash
0 2 * * * /scripts/backup.sh
```

### Neo4j Backup

```bash
docker exec anime-neo4j neo4j-admin backup --backup-dir=/backups/neo4j --database=neo4j
```

---

## Monitoring & Logging

### Health Checks

```bash
# Check all services
curl -s http://localhost:3001/health || echo "API down"
curl -s http://localhost:5173/ || echo "Frontend down"
```

### Logs

```bash
# View real-time logs
docker-compose logs -f

# View specific service
docker-compose logs -f nestjs-bridge

# Export logs
docker-compose logs > logs_$(date +%Y%m%d).txt
```

### Monitoring Setup (Optional)

Consider setting up:
- **Prometheus** - Metrics collection
- **Grafana** - Visualization
- **Sentry** - Error tracking
- **ELK Stack** - Log aggregation

---

## Performance Optimization

### Database
- Enable PostgreSQL connection pooling (pgBouncer)
- Add indexes on frequently queried columns
- Monitor slow queries

### Caching
- Redis caching enabled by default
- Cache recommendations for 1 hour
- Cache user data for 30 minutes

### Frontend
- SvelteKit builds to static + server
- Enable gzip compression in nginx
- Use CDN for static assets

### Backend
- Node cluster mode for multi-core
- Load balancing via nginx
- Connection pooling for databases

---

## Security Hardening

1. **Environment Variables**
   - Never commit .env files
   - Use strong passwords (32+ chars)
   - Rotate JWT secret monthly

2. **Network**
   - Only expose ports 80/443
   - Use HTTPS everywhere
   - Enable rate limiting

3. **Database**
   - Regular backups
   - User role restrictions
   - Monitor unusual queries

4. **Application**
   - Keep dependencies updated
   - Enable CORS only for trusted domains
   - Input validation on all endpoints
   - SQL injection prevention (TypeORM handles this)

5. **SSL/TLS**
   - Use Let's Encrypt for free certificates
   - Enable HSTS headers
   - Certificate renewal automation

---

## Scaling

### Horizontal Scaling
- Run multiple NestJS instances behind nginx load balancer
- Use managed database services (AWS RDS, etc.)
- Shared Redis instance for session management

### Vertical Scaling
- Increase server RAM/CPU
- Optimize database queries
- Use CDN for static assets

### Example Multi-Instance Setup
```yaml
nestjs-bridge-1:
  image: anime-nestjs-bridge:latest
nestjs-bridge-2:
  image: anime-nestjs-bridge:latest

upstream api_backend {
    server nestjs-bridge-1:3001;
    server nestjs-bridge-2:3001;
}
```

---

## Rollback Procedure

1. **Keep previous image versions**
```bash
docker tag anime-nestjs-bridge:latest anime-nestjs-bridge:v1.0.0
docker tag anime-nestjs-bridge:v1.1.0 anime-nestjs-bridge:latest
```

2. **Rollback if needed**
```bash
docker-compose down
docker-compose up -d  # Uses "latest" which now points to v1.0.0
```

3. **Database migration rollback** (keep migration scripts)
```bash
npm run migration:revert
```

---

## Support & Debugging

### Common Issues

**502 Bad Gateway**
- Check if NestJS is running: `docker logs anime-nestjs-api`
- Verify database connection: `curl http://localhost:3001/health`

**Slow performance**
- Check Redis: `docker exec anime-redis redis-cli INFO`
- Monitor database: `docker logs anime-postgres`

**JWT errors**
- Verify JWT_SECRET matches across services
- Check token expiration: `JWT_EXPIRATION=3600` (1 hour)

### Useful Commands
```bash
docker-compose exec postgres psql -U postgres -d anime_bridge_prod
docker-compose exec redis redis-cli
docker-compose logs --tail=100 nestjs-bridge
docker stats  # View resource usage
```

---

## Maintenance Schedule

| Task | Frequency | Command |
|------|-----------|---------|
| Database backup | Daily | `backup.sh` |
| Certificate renewal | 90 days | Let's Encrypt auto-renewal |
| Dependency updates | Monthly | `npm update` |
| Security audit | Quarterly | Manual review |
| Performance review | Quarterly | Check metrics |

---

For production support, contact your DevOps team or consulting partner.
