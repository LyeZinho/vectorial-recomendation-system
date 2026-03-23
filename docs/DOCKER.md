# Docker Deployment Guide

## Quick Start

### Prerequisites
- Docker & Docker Compose installed
- 2GB disk space for Neo4j volume (persistent data)
- Ports 3001, 7687 available
- 40 seconds for full stack startup

### Start Stack

```bash
docker-compose up -d
```

**Wait for health checks** (view with `docker-compose ps`):
- `neo4j` → health status: `healthy`
- `api` → health status: `healthy`

Containers:
- **API Server**: http://localhost:3001
- **Neo4j Browser**: http://localhost:7474
- **Neo4j Bolt**: localhost:7687

### Health Check

```bash
curl http://localhost:3001/
```

Expected:
```json
{
  "service": "Anime2Vec Recommendation Engine",
  "status": "ready",
  "version": "2.0"
}
```

---

## Data Population

### Option 1: Use Pre-Built Cypher Script (Fastest)

```bash
cat data/import_neo4j.cypher | docker exec -i anime-neo4j cypher-shell -u neo4j -p password
```

This loads 10 test anime with relationships. Verify:

```bash
docker exec anime-neo4j cypher-shell -u neo4j -p password "MATCH (n:Anime) RETURN count(n);"
```

Expected: `10`

### Option 2: Run Full Data Import (From Phase 1 Harvest)

1. **Run harvest pipeline** (on host machine):

```bash
./target/release/anime-harvester harvest --datapool datapool --output data
```

Output: `data/harvester.db` (60KB SQLite)

2. **Export and import to Neo4j**:

```bash
./target/release/anime-harvester export-graph --db data/harvester.db --output data/graph.bin
```

3. **Create custom import script** for your dataset:

```bash
# Example: Create a Cypher script that parses graph.bin
# and batches CREATE statements
cat > data/import_custom.cypher << 'EOF'
// Your Cypher import logic here
EOF

# Load it:
cat data/import_custom.cypher | docker exec -i anime-neo4j cypher-shell -u neo4j -p password
```

---

## API Endpoints

### Health Check

```bash
GET http://localhost:3001/
```

Response:
```json
{
  "service": "Anime2Vec Recommendation Engine",
  "status": "ready",
  "version": "2.0"
}
```

### Get Recommendations

```bash
GET http://localhost:3001/api/recommendations/:anime_id
```

Example:
```bash
curl "http://localhost:3001/api/recommendations/1" | jq
```

Response:
```json
{
  "anime_id": "1",
  "recommendations": [
    {
      "anime_id": "rec1",
      "title": "Similar Anime 1",
      "score": 0.95,
      "layer": "technical_dna",
      "explanation": "Same director"
    }
  ]
}
```

### Search Anime

```bash
GET http://localhost:3001/api/search?q=<query>
```

Example:
```bash
curl "http://localhost:3001/api/search?q=Death" | jq
```

Response:
```json
{
  "query": "Death",
  "results": [
    {
      "anime_id": "death-note",
      "relevance": 1.0,
      "title": "Death Note"
    }
  ]
}
```

### Explain Recommendation

```bash
GET http://localhost:3001/api/explain/:rec_id
```

Example:
```bash
curl "http://localhost:3001/api/explain/rec1" | jq
```

---

## Monitoring

### View Container Status

```bash
docker-compose ps
```

Expected:
```
NAME                  IMAGE                                COMMAND                  STATUS
anime-neo4j           neo4j:5.19-community                 "tini -g -- /startup…"   Up XXs (healthy)
anime-harvester-api   vectorial-recomendation-system-api   "anime-harvester ser…"   Up XXs (healthy)
```

### View Logs

**API logs** (live tail):
```bash
docker-compose logs -f api
```

**Neo4j logs** (live tail):
```bash
docker-compose logs -f neo4j
```

**One-off logs** (last 50 lines):
```bash
docker-compose logs api --tail=50
```

### Enter Container Shell

**API container**:
```bash
docker exec -it anime-harvester-api bash
```

**Neo4j container**:
```bash
docker exec -it anime-neo4j bash
```

---

## Database Management

### Access Neo4j Directly

```bash
docker exec -it anime-neo4j cypher-shell -u neo4j -p password
```

Then run Cypher queries:
```cypher
MATCH (n:Anime) RETURN n LIMIT 5;
MATCH ()-[r]->() RETURN count(r);
MATCH (n) RETURN count(distinct n) as nodes;
```

### Backup Data

**Backup Neo4j volume**:
```bash
docker run --rm -v vectorial-recomendation-system_neo4j_data:/data \
  -v $(pwd):/backup ubuntu tar czf /backup/neo4j_backup.tar.gz /data
```

**Backup API data**:
```bash
tar czf api_data_backup.tar.gz data/
```

### Reset Database

```bash
# Delete all Neo4j data
docker-compose down -v

# Restart clean
docker-compose up -d

# Reload data
cat data/import_neo4j.cypher | docker exec -i anime-neo4j cypher-shell -u neo4j -p password
```

---

## Performance Tuning

### Docker Compose Resource Limits

Edit `docker-compose.yml` to add limits:

```yaml
services:
  neo4j:
    deploy:
      resources:
        limits:
          cpus: '2'
          memory: 2G
  api:
    deploy:
      resources:
        limits:
          cpus: '1'
          memory: 512M
```

Then restart:
```bash
docker-compose down
docker-compose up -d
```

### Enable Query Profiling (Neo4j)

```bash
docker exec anime-neo4j cypher-shell -u neo4j -p password \
  "PROFILE MATCH (a:Anime) RETURN a LIMIT 5;"
```

### Monitor Port Usage

```bash
lsof -i :3001
lsof -i :7687
```

---

## Troubleshooting

### API Won't Start

**Check logs**:
```bash
docker-compose logs api --tail=100
```

**Common issues**:
- Neo4j not healthy → wait 30+ seconds
- Port 3001 in use → change port in `docker-compose.yml` line 37
- Image not built → run `docker-compose build --no-cache`

### Neo4j Won't Start

**Check logs**:
```bash
docker-compose logs neo4j --tail=100
```

**Common issues**:
- Corrupted volume → `docker-compose down -v` then restart
- Insufficient disk space → check with `docker system df`

### Slow Queries

**Enable query logging**:
```bash
docker exec -it anime-neo4j bash
# Edit neo4j.conf to add:
# dbms.security.auth_enabled=false  # For testing only!
```

Then restart and monitor:
```bash
docker-compose restart neo4j
docker-compose logs -f neo4j | grep -i query
```

### Can't Connect to Neo4j from API

**Verify network**:
```bash
docker exec anime-harvester-api ping neo4j
```

**Expected**: Pings succeed (network is bridge-connected)

---

## Shutdown

### Graceful Shutdown

```bash
docker-compose down
```

Data persists in volumes (`neo4j_data`, `neo4j_logs`).

### Hard Shutdown (Clear Everything)

```bash
docker-compose down -v
docker rmi vectorial-recomendation-system-api:latest
```

---

## Production Deployment

For production:

1. **Use external Neo4j** (managed service)
2. **Add authentication**: Store credentials in `.env`
3. **Enable SSL/TLS**: Configure Axum with certificates
4. **Set resource limits**: CPU/memory constraints per container
5. **Use health checks**: Configure monitoring/alerts
6. **Backup strategy**: Automated Neo4j backups
7. **Logging**: Ship logs to centralized service (ELK, Datadog)

Example production `docker-compose.yml`:

```yaml
services:
  neo4j:
    environment:
      NEO4J_AUTH: ${NEO4J_USER}/${NEO4J_PASSWORD}
    volumes:
      - neo4j_data:/data
    restart: always
  api:
    environment:
      NEO4J_URI: ${NEO4J_URI}
    restart: always
    logging:
      driver: "json-file"
      options:
        max-size: "10m"
        max-file: "3"
```

---

## Support

For issues or questions, check:
- API logs: `docker-compose logs api`
- Neo4j logs: `docker-compose logs neo4j`
- Performance profile: `docs/PERFORMANCE_DOCKER.md`
- Phase 3 docs: `docs/PHASE3_COMPLETE.md`

---

Generated: 2026-03-23
Status: Docker deployment guide v1.0 ✅
