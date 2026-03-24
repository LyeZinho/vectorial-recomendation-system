#!/bin/bash

echo "=== Phase 4 API Performance Test ==="
echo ""

# Test recommendations endpoint
echo "Testing /api/recommendations/1 (10 times)..."
times=()
for i in {1..10}; do
    start=$(date +%s%N)
    curl -s "http://localhost:3001/api/recommendations/1" > /dev/null
    end=$(date +%s%N)
    elapsed=$((($end - $start) / 1000000))
    times+=($elapsed)
    echo "  Request $i: ${elapsed}ms"
done

total_time=0
for t in "${times[@]}"; do
    total_time=$((total_time + t))
done
avg_time=$((total_time / ${#times[@]}))
echo "Average: ${avg_time}ms (target <50ms)"
echo ""

# Test search endpoint
echo "Testing /api/search?q=Death (5 times)..."
for i in {1..5}; do
    start=$(date +%s%N)
    curl -s "http://localhost:3001/api/search?q=Death" > /dev/null
    end=$(date +%s%N)
    elapsed=$((($end - $start) / 1000000))
    echo "  Request $i: ${elapsed}ms"
done
echo ""

# Test explain endpoint
echo "Testing /api/explain/1/3 (5 times)..."
for i in {1..5}; do
    start=$(date +%s%N)
    curl -s "http://localhost:3001/api/explain/1/3" > /dev/null
    end=$(date +%s%N)
    elapsed=$((($end - $start) / 1000000))
    echo "  Request $i: ${elapsed}ms"
done

echo ""
echo "✅ Performance test complete"
