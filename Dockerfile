# Dockerfile for anime-harvester
# Uses pre-built binary for simplicity (faster builds)
FROM debian:bookworm-slim

WORKDIR /app

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libsqlite3-0 \
    curl \
    && rm -rf /var/lib/apt/lists/*

COPY target/release/anime-harvester /usr/local/bin/

EXPOSE 3000

HEALTHCHECK --interval=30s --timeout=10s --start-period=40s --retries=3 \
    CMD curl -f http://localhost:3000/ || exit 1

ENTRYPOINT ["anime-harvester"]
CMD ["serve", "--host", "0.0.0.0", "--port", "3000", "--neo4j-uri", "neo4j://neo4j:7687"]
