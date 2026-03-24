/**
 * Performance Benchmarking Script
 * Measures: Response times, throughput, concurrent user capacity
 * Tools: autocannon for load testing
 */

const autocannon = require('autocannon');

const API_URL = process.env.API_URL || 'http://localhost:3001';
const FRONTEND_URL = process.env.FRONTEND_URL || 'http://localhost:5173';

// Test scenarios
const benchmarks = {
  auth: {
    title: 'Authentication Flow',
    duration: 10,
    connections: 50,
    requests: [
      {
        method: 'POST',
        path: '/auth/register',
        body: JSON.stringify({
          email: `test-${Date.now()}@example.com`,
          username: `testuser${Math.random().toString(36).substring(7)}`,
          password: 'TestPassword123!',
        }),
        headers: {
          'Content-Type': 'application/json',
        },
      },
    ],
  },
  search: {
    title: 'Search & Discovery',
    duration: 10,
    connections: 50,
    requests: [
      {
        method: 'GET',
        path: '/search/anime?query=naruto&limit=20&offset=0',
      },
      {
        method: 'GET',
        path: '/search/anime?query=anime&genre=Action&limit=20',
      },
    ],
  },
  recommendations: {
    title: 'Recommendations API',
    duration: 10,
    connections: 30,
    requests: [
      {
        method: 'GET',
        path: '/recommendations/1',
      },
    ],
  },
};

async function runBenchmark(name, config) {
  console.log(`\n${'='.repeat(60)}`);
  console.log(`📊 ${config.title}`);
  console.log(`${'='.repeat(60)}`);

  const instance = autocannon({
    url: API_URL,
    duration: config.duration,
    connections: config.connections,
    pipelining: 10,
    requests: config.requests,
    setupClient: (client) => {
      client.on('response', (statusCode, resBytes, responseTime) => {
        if (statusCode >= 400) {
          console.log(`⚠️  HTTP ${statusCode} - ${responseTime}ms`);
        }
      });
    },
  });

  instance.on('done', (results) => {
    console.log(`\n📈 Results for ${config.title}:`);
    console.log(`   Requests: ${results.requests.total}`);
    console.log(`   Throughput: ${Math.round(results.throughput.average)} req/s`);
    console.log(`   Latency (avg): ${Math.round(results.latency.mean)}ms`);
    console.log(`   Latency (p99): ${results.latency.p99}ms`);
    console.log(`   Errors: ${results.errors}`);
    console.log(`   Timeouts: ${results.timeouts}`);
  });

  return new Promise((resolve) => {
    instance.on('done', resolve);
  });
}

async function main() {
  console.log('🚀 Starting Performance Benchmarks...');
  console.log(`Target API: ${API_URL}`);
  console.log(`Test Duration: ~${Object.values(benchmarks).reduce((sum, b) => sum + b.duration, 0)}s`);

  try {
    for (const [key, config] of Object.entries(benchmarks)) {
      await runBenchmark(key, config);
    }

    console.log(`\n${'='.repeat(60)}`);
    console.log('✅ All benchmarks complete!');
    console.log(`${'='.repeat(60)}\n`);

    process.exit(0);
  } catch (error) {
    console.error('❌ Benchmark failed:', error.message);
    process.exit(1);
  }
}

main();
