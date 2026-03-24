# API Endpoints Documentation

## Recommendations Endpoint

**Request:**
```
GET /api/recommendations/:anime_id
```

**Parameters:**
- `anime_id` (integer, required): ID of the anime to get recommendations for

**Response (200 OK):**
```json
{
  "anime_id": 1,
  "anime_name": "Attack on Titan",
  "recommendations": [
    {
      "anime_id": 3,
      "name": "Demon Slayer",
      "genre": "Action,Adventure",
      "rating": 4.6,
      "score": 0.82,
      "weight": 0.85,
      "layer": "technical_dna"
    }
  ],
  "total": 1
}
```

**Error (404 Not Found):**
```json
{
  "error": {
    "code": "NOT_FOUND",
    "message": "Anime with ID 99999 not found"
  }
}
```

**Performance:** <50ms average

---

## Search Endpoint

**Request:**
```
GET /api/search?q=query
```

**Parameters:**
- `q` (string, required): Search query (title substring match)

**Response (200 OK):**
```json
{
  "query": "Death",
  "results": [
    {
      "anime_id": 2,
      "name": "Death Note",
      "genre": "Thriller,Supernatural",
      "relevance": 1.0
    }
  ],
  "suggestions": [
    {
      "anime_id": 8,
      "name": "Death Parade"
    }
  ]
}
```

**Error (400 Bad Request):**
```json
{
  "error": {
    "code": "BAD_REQUEST",
    "message": "Search query cannot be empty"
  }
}
```

**Performance:** <100ms average

---

## Explain Endpoint

**Request:**
```
GET /api/explain/:anime_id/:rec_id
```

**Parameters:**
- `anime_id` (integer, required): Source anime ID
- `rec_id` (integer, required): Recommended anime ID

**Response (200 OK):**
```json
{
  "source_id": 1,
  "source_name": "Attack on Titan",
  "recommendation_id": 3,
  "recommendation_name": "Demon Slayer",
  "relationship": "RELATED_TO",
  "layer": "technical_dna",
  "weight": 0.85,
  "explanation": "These anime share similar themes and narrative structure (relationship weight: 0.85)"
}
```

**Error (404 Not Found):**
```json
{
  "error": {
    "code": "NOT_FOUND",
    "message": "Relationship between anime 1 and 99 not found"
  }
}
```

**Performance:** <30ms average

---

## Error Codes

| Code | HTTP Status | Meaning |
|------|-------------|---------|
| NOT_FOUND | 404 | Anime or relationship not found |
| BAD_REQUEST | 400 | Invalid parameters (e.g., empty query) |
| INTERNAL_ERROR | 500 | Server error (query execution failed) |
| SERVICE_UNAVAILABLE | 503 | Neo4j is unreachable |
