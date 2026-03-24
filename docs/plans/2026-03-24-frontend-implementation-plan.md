# Frontend Implementation Plan - SvelteKit + NestJS Bridge

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task with checkpoint reviews between phases.

**Goal:** Build a production-ready three-tier system (SvelteKit frontend + NestJS bridge backend + Rust engine) with authentication, recommendations, search, and watchlist features.

**Architecture:** 
- **SvelteKit Frontend** (Port 5173) - Neobrutalist UI with reactive stores
- **NestJS Bridge** (Port 3001) - Smart adapter layer with PostgreSQL user persistence
- **Rust API** (Port 3000) - Existing recommendation engine (unchanged)
- All services orchestrated with Docker Compose at project root

**Tech Stack:**
- Frontend: SvelteKit, Svelte 5, Vite, Tailwind CSS, TypeScript
- Backend: NestJS, TypeORM, PostgreSQL, bcrypt, JWT
- Build: npm workspaces, Docker Compose
- Testing: Vitest (frontend), Jest (NestJS)

**Design Reference:** `/home/pedro/repo/vectorial-recomendation-system/docs/plans/2026-03-24-frontend-architecture-design.md`

---

## Phase 1: Project Scaffolding & Setup (5 tasks)

### Task 1: Create Workspace Structure

**Files:**
- Create: `services/` directory
- Create: `services/nestjs-bridge/package.json`
- Create: `services/sveltekit-frontend/package.json`
- Modify: Root `package.json` (add workspaces)

**Step 1: Create services directory and structure**

```bash
mkdir -p services/{nestjs-bridge,sveltekit-frontend}
cd /home/pedro/repo/vectorial-recomendation-system
```

**Step 2: Create root package.json with workspaces**

Update root `package.json`:
```json
{
  "name": "vectorial-recommendation-system",
  "private": true,
  "workspaces": [
    "services/nestjs-bridge",
    "services/sveltekit-frontend"
  ],
  "scripts": {
    "dev": "npm run dev --workspaces",
    "build": "npm run build --workspaces",
    "test": "npm run test --workspaces"
  }
}
```

**Step 3: Initialize NestJS project structure**

Create `services/nestjs-bridge/package.json`:
```json
{
  "name": "nestjs-bridge",
  "version": "0.1.0",
  "private": true,
  "scripts": {
    "dev": "nest start --watch",
    "build": "nest build",
    "start": "node dist/main",
    "test": "jest",
    "test:watch": "jest --watch",
    "lint": "eslint src"
  },
  "dependencies": {
    "@nestjs/common": "^10.0.0",
    "@nestjs/core": "^10.0.0",
    "@nestjs/jwt": "^12.0.0",
    "@nestjs/passport": "^10.0.0",
    "@nestjs/typeorm": "^9.0.0",
    "@types/bcrypt": "^5.0.0",
    "@types/passport-jwt": "^3.0.8",
    "bcrypt": "^5.1.0",
    "class-transformer": "^0.5.1",
    "class-validator": "^0.14.0",
    "passport": "^0.6.0",
    "passport-jwt": "^4.0.1",
    "pg": "^8.10.0",
    "reflect-metadata": "^0.1.13",
    "rxjs": "^7.8.1",
    "typeorm": "^0.3.16",
    "axios": "^1.6.0"
  },
  "devDependencies": {
    "@nestjs/cli": "^10.0.0",
    "@types/express": "^4.17.17",
    "@types/jest": "^29.5.0",
    "@types/node": "^18.15.0",
    "jest": "^29.5.0",
    "ts-jest": "^29.1.0",
    "ts-node": "^10.9.0",
    "typescript": "^5.0.0",
    "eslint": "^8.36.0"
  }
}
```

**Step 4: Initialize SvelteKit project structure**

Create `services/sveltekit-frontend/package.json`:
```json
{
  "name": "sveltekit-frontend",
  "version": "0.1.0",
  "private": true,
  "scripts": {
    "dev": "vite",
    "build": "vite build",
    "preview": "vite preview",
    "test": "vitest",
    "lint": "eslint src"
  },
  "dependencies": {
    "svelte": "^5.0.0"
  },
  "devDependencies": {
    "@sveltejs/vite-plugin-svelte": "^3.0.0",
    "@tailwindcss/vite": "^4.1.14",
    "tailwindcss": "^4.1.14",
    "typescript": "^5.0.0",
    "vite": "^6.2.0",
    "vitest": "^1.0.0",
    "autoprefixer": "^10.4.21"
  }
}
```

**Step 5: Commit**

```bash
git add services/ package.json
git commit -m "chore: scaffold workspace structure for NestJS bridge and SvelteKit frontend"
```

**Expected:** Both service directories created with valid package.json files.

---

### Task 2: Initialize NestJS Backend

**Files:**
- Create: `services/nestjs-bridge/src/main.ts`
- Create: `services/nestjs-bridge/src/app.module.ts`
- Create: `services/nestjs-bridge/nest-cli.json`
- Create: `services/nestjs-bridge/tsconfig.json`
- Create: `services/nestjs-bridge/.env.example`

**Step 1: Create NestJS entry point**

Create `services/nestjs-bridge/src/main.ts`:
```typescript
import { NestFactory } from '@nestjs/core';
import { AppModule } from './app.module';

async function bootstrap() {
  const app = await NestFactory.create(AppModule);

  app.enableCors({
    origin: process.env.FRONTEND_URL || 'http://localhost:5173',
    credentials: true,
  });

  const port = process.env.PORT || 3001;
  await app.listen(port);
  console.log(`🚀 NestJS Bridge running on http://localhost:${port}`);
}

bootstrap();
```

**Step 2: Create AppModule**

Create `services/nestjs-bridge/src/app.module.ts`:
```typescript
import { Module } from '@nestjs/common';
import { TypeOrmModule } from '@nestjs/typeorm';
import { ConfigModule } from '@nestjs/config';
import { AuthModule } from './auth/auth.module';
import { UsersModule } from './users/users.module';

@Module({
  imports: [
    ConfigModule.forRoot({
      envFilePath: '.env',
      isGlobal: true,
    }),
    TypeOrmModule.forRoot({
      type: 'postgres',
      host: process.env.DATABASE_HOST || 'localhost',
      port: parseInt(process.env.DATABASE_PORT || '5432'),
      username: process.env.DATABASE_USER || 'postgres',
      password: process.env.DATABASE_PASSWORD || 'password',
      database: process.env.DATABASE_NAME || 'anime_bridge',
      entities: ['src/**/*.entity.ts'],
      synchronize: process.env.NODE_ENV !== 'production',
      logging: process.env.NODE_ENV === 'development',
    }),
    AuthModule,
    UsersModule,
  ],
})
export class AppModule {}
```

**Step 3: Create TypeScript configuration**

Create `services/nestjs-bridge/tsconfig.json`:
```json
{
  "compilerOptions": {
    "module": "commonjs",
    "target": "ES2021",
    "lib": ["ES2021"],
    "outDir": "./dist",
    "rootDir": "./src",
    "strict": true,
    "esModuleInterop": true,
    "skipLibCheck": true,
    "forceConsistentCasingInFileNames": true,
    "resolveJsonModule": true,
    "declaration": true,
    "decorators": true,
    "experimentalDecorators": true,
    "emitDecoratorMetadata": true,
    "moduleResolution": "node"
  },
  "include": ["src"],
  "exclude": ["node_modules", "dist"]
}
```

**Step 4: Create NestJS CLI config**

Create `services/nestjs-bridge/nest-cli.json`:
```json
{
  "collection": "@nestjs/schematics",
  "sourceRoot": "src",
  "compilerOptions": {
    "deleteOutDir": true
  }
}
```

**Step 5: Create .env.example**

Create `services/nestjs-bridge/.env.example`:
```
PORT=3001
NODE_ENV=development

DATABASE_HOST=localhost
DATABASE_PORT=5432
DATABASE_USER=postgres
DATABASE_PASSWORD=password
DATABASE_NAME=anime_bridge

JWT_SECRET=your-super-secret-jwt-key-min-32-chars-long-for-development
JWT_EXPIRATION=3600

RUST_API_URL=http://localhost:3000
REDIS_URL=redis://localhost:6379

FRONTEND_URL=http://localhost:5173
```

**Step 6: Commit**

```bash
git add services/nestjs-bridge/
git commit -m "feat(nestjs): initialize NestJS backend with core configuration"
```

**Expected:** NestJS project configured and ready for module development.

---

### Task 3: Initialize SvelteKit Frontend

**Files:**
- Create: `services/sveltekit-frontend/src/app.html`
- Create: `services/sveltekit-frontend/src/routes/+page.svelte`
- Create: `services/sveltekit-frontend/src/app.css`
- Create: `services/sveltekit-frontend/vite.config.ts`
- Create: `services/sveltekit-frontend/svelte.config.js`
- Create: `services/sveltekit-frontend/tsconfig.json`
- Create: `services/sveltekit-frontend/.env.example`

**Step 1: Create Svelte configuration**

Create `services/sveltekit-frontend/svelte.config.js`:
```javascript
import adapter from '@sveltejs/adapter-node';

export default {
  kit: {
    adapter: adapter(),
  },
};
```

**Step 2: Create Vite configuration**

Create `services/sveltekit-frontend/vite.config.ts`:
```typescript
import { defineConfig } from 'vite';
import { svelte } from '@sveltejs/vite-plugin-svelte';

export default defineConfig({
  plugins: [svelte()],
  server: {
    port: 5173,
    host: '0.0.0.0',
  },
  preview: {
    port: 5173,
    host: '0.0.0.0',
  },
});
```

**Step 3: Create TypeScript configuration**

Create `services/sveltekit-frontend/tsconfig.json`:
```json
{
  "extends": "./.svelte-kit/tsconfig.json",
  "compilerOptions": {
    "strict": true,
    "esModuleInterop": true,
    "skipLibCheck": true,
    "forceConsistentCasingInFileNames": true,
    "resolveJsonModule": true,
    "moduleResolution": "node",
    "target": "ES2020",
    "module": "ES2020",
    "lib": ["ES2020", "DOM", "DOM.Iterable"]
  }
}
```

**Step 4: Create app layout**

Create `services/sveltekit-frontend/src/app.html`:
```html
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width" />
    <title>Anime Recommendation Engine</title>
    %sveltekit.head%
  </head>
  <body data-sveltekit-preload-data="hover">
    <div>%sveltekit.body%</div>
  </body>
</html>
```

**Step 5: Create home page**

Create `services/sveltekit-frontend/src/routes/+page.svelte`:
```svelte
<script>
  export let data;
</script>

<div class="min-h-screen bg-black text-white flex flex-col items-center justify-center">
  <h1 class="text-6xl font-black mb-4 text-yellow-300">ANIME.VEC</h1>
  <p class="text-2xl mb-8 text-purple-400">Vectorial Recommendation Engine</p>
  <a href="/login" class="bg-yellow-300 text-black px-8 py-3 font-bold text-lg hover:bg-yellow-200">
    GET STARTED
  </a>
</div>
```

**Step 6: Create Tailwind CSS**

Create `services/sveltekit-frontend/src/app.css`:
```css
@tailwind base;
@tailwind components;
@tailwind utilities;

@layer base {
  body {
    @apply bg-black text-white font-sans;
  }

  h1, h2, h3, h4, h5, h6 {
    @apply font-black tracking-tight;
  }
}

@layer components {
  .btn-primary {
    @apply bg-yellow-300 text-black px-6 py-3 font-bold hover:bg-yellow-200 transition;
  }

  .btn-secondary {
    @apply bg-purple-600 text-white px-6 py-3 font-bold hover:bg-purple-500 transition;
  }

  .card {
    @apply bg-gray-900 border-2 border-yellow-300 p-4;
  }
}
```

**Step 7: Create .env.example**

Create `services/sveltekit-frontend/.env.example`:
```
PUBLIC_API_URL=http://localhost:3001
```

**Step 8: Commit**

```bash
git add services/sveltekit-frontend/
git commit -m "feat(sveltekit): initialize SvelteKit frontend with neobrutalist design"
```

**Expected:** SvelteKit project configured with Tailwind and home page visible.

---

### Task 4: Create Docker Setup

**Files:**
- Create: `services/nestjs-bridge/Dockerfile`
- Create: `services/nestjs-bridge/docker-compose.yml`
- Create: `services/sveltekit-frontend/Dockerfile`
- Modify: Root `docker-compose.yml` (orchestrate all services)

**Step 1: Create NestJS Dockerfile**

Create `services/nestjs-bridge/Dockerfile`:
```dockerfile
FROM node:20-alpine AS builder
WORKDIR /app
COPY package*.json ./
RUN npm ci
COPY . .
RUN npm run build

FROM node:20-alpine
WORKDIR /app
RUN npm install -g @nestjs/cli
COPY package*.json ./
RUN npm ci --only=production
COPY --from=builder /app/dist ./dist
EXPOSE 3001
CMD ["node", "dist/main"]
```

**Step 2: Create SvelteKit Dockerfile**

Create `services/sveltekit-frontend/Dockerfile`:
```dockerfile
FROM node:20-alpine AS builder
WORKDIR /app
COPY package*.json ./
RUN npm ci
COPY . .
RUN npm run build

FROM node:20-alpine
WORKDIR /app
COPY package*.json ./
RUN npm ci --only=production
COPY --from=builder /app/build ./build
EXPOSE 5173
CMD ["node", "-r", "dotenv/config", "build"]
```

**Step 3: Create root Docker Compose**

Create/Modify root `docker-compose.yml`:
```yaml
version: '3.8'

services:
  postgres:
    image: postgres:15-alpine
    container_name: anime-postgres
    environment:
      POSTGRES_USER: postgres
      POSTGRES_PASSWORD: password
      POSTGRES_DB: anime_bridge
    ports:
      - "5432:5432"
    volumes:
      - postgres_data:/var/lib/postgresql/data
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U postgres"]
      interval: 10s
      timeout: 5s
      retries: 5
    networks:
      - anime-network

  redis:
    image: redis:7-alpine
    container_name: anime-redis
    ports:
      - "6379:6379"
    healthcheck:
      test: ["CMD", "redis-cli", "ping"]
      interval: 10s
      timeout: 5s
      retries: 5
    networks:
      - anime-network

  neo4j:
    image: neo4j:5.19-community
    container_name: anime-neo4j
    environment:
      NEO4J_AUTH: neo4j/password
      NEO4J_ACCEPT_LICENSE_AGREEMENT: "yes"
    ports:
      - "7687:7687"
      - "7474:7474"
    volumes:
      - neo4j_data:/var/lib/neo4j/data
    healthcheck:
      test: ["CMD-SHELL", "cypher-shell -u neo4j -p password 'RETURN 1' || exit 1"]
      interval: 10s
      timeout: 5s
      retries: 5
    networks:
      - anime-network

  rust-engine:
    build: services/rust-engine
    container_name: anime-rust-api
    depends_on:
      neo4j:
        condition: service_healthy
      redis:
        condition: service_healthy
    environment:
      NEO4J_URI: neo4j://neo4j:7687
      NEO4J_USER: neo4j
      NEO4J_PASSWORD: password
      REDIS_URL: redis://redis:6379/
      JWT_SECRET: dev-phase5-secret-key-min-32-chars
      RUST_LOG: info
    ports:
      - "3000:3000"
    networks:
      - anime-network
    healthcheck:
      test: ["CMD-SHELL", "curl -f http://localhost:3000/ || exit 1"]
      interval: 30s
      timeout: 10s
      retries: 3

  nestjs-bridge:
    build: services/nestjs-bridge
    container_name: anime-nestjs-api
    depends_on:
      postgres:
        condition: service_healthy
      redis:
        condition: service_healthy
      rust-engine:
        condition: service_healthy
    environment:
      PORT: 3001
      NODE_ENV: development
      DATABASE_HOST: postgres
      DATABASE_PORT: 5432
      DATABASE_USER: postgres
      DATABASE_PASSWORD: password
      DATABASE_NAME: anime_bridge
      JWT_SECRET: nestjs-dev-secret-key-min-32-chars-long
      JWT_EXPIRATION: 3600
      RUST_API_URL: http://rust-engine:3000
      REDIS_URL: redis://redis:6379
      FRONTEND_URL: http://localhost:5173
    ports:
      - "3001:3001"
    networks:
      - anime-network
    healthcheck:
      test: ["CMD-SHELL", "curl -f http://localhost:3001/ || exit 1"]
      interval: 30s
      timeout: 10s
      retries: 3

  sveltekit-frontend:
    build: services/sveltekit-frontend
    container_name: anime-sveltekit-frontend
    depends_on:
      nestjs-bridge:
        condition: service_healthy
    environment:
      PUBLIC_API_URL: http://nestjs-bridge:3001
    ports:
      - "5173:5173"
    networks:
      - anime-network
    healthcheck:
      test: ["CMD-SHELL", "curl -f http://localhost:5173/ || exit 1"]
      interval: 30s
      timeout: 10s
      retries: 3

volumes:
  postgres_data:
    driver: local
  neo4j_data:
    driver: local

networks:
  anime-network:
    driver: bridge
```

**Step 4: Commit**

```bash
git add services/nestjs-bridge/Dockerfile services/sveltekit-frontend/Dockerfile docker-compose.yml
git commit -m "chore: add Docker configuration for all services"
```

**Expected:** All services defined in Docker Compose with proper health checks and dependencies.

---

### Task 5: Test Scaffolding & Build Verification

**Files:**
- Create: `services/nestjs-bridge/jest.config.js`
- Create: `services/sveltekit-frontend/vitest.config.ts`
- Modify: Both `package.json` scripts to include build

**Step 1: Create Jest config for NestJS**

Create `services/nestjs-bridge/jest.config.js`:
```javascript
module.exports = {
  moduleFileExtensions: ['js', 'json', 'ts'],
  rootDir: 'src',
  testRegex: '.*\\.spec\\.ts$',
  transform: {
    '^.+\\.(t|j)s$': 'ts-jest',
  },
  collectCoverageFrom: [
    '**/*.(t|j)s',
  ],
  coverageDirectory: '../coverage',
  testEnvironment: 'node',
};
```

**Step 2: Create Vitest config for SvelteKit**

Create `services/sveltekit-frontend/vitest.config.ts`:
```typescript
import { defineConfig } from 'vitest/config';
import { svelte } from '@sveltejs/vite-plugin-svelte';

export default defineConfig({
  plugins: [svelte()],
  test: {
    environment: 'jsdom',
    globals: true,
  },
});
```

**Step 3: Verify builds locally (dev environment)**

```bash
cd /home/pedro/repo/vectorial-recomendation-system

# Install all dependencies
npm install

# Build NestJS
cd services/nestjs-bridge
npm run build
# Expected: dist/ directory created

# Build SvelteKit
cd ../sveltekit-frontend
npm run build
# Expected: .svelte-kit/ and build/ directories created
```

**Step 4: Commit**

```bash
git add services/nestjs-bridge/jest.config.js services/sveltekit-frontend/vitest.config.ts
git commit -m "chore: configure testing frameworks (Jest, Vitest)"
```

**Expected:** Both projects build successfully with no errors.

---

## Phase 2: Authentication System (6 tasks)

### Task 6: Create User Entity & Database Schema

**Files:**
- Create: `services/nestjs-bridge/src/users/entities/user.entity.ts`
- Create: `services/nestjs-bridge/src/users/users.module.ts`
- Create: `services/nestjs-bridge/src/users/users.service.ts`
- Create: `services/nestjs-bridge/src/auth/auth.module.ts`

**Step 1: Create User entity with TypeORM**

Create `services/nestjs-bridge/src/users/entities/user.entity.ts`:
```typescript
import { Entity, PrimaryGeneratedColumn, Column, CreateDateColumn, UpdateDateColumn } from 'typeorm';

@Entity('users')
export class User {
  @PrimaryGeneratedColumn('uuid')
  id: string;

  @Column({ unique: true })
  email: string;

  @Column({ unique: true })
  username: string;

  @Column()
  password_hash: string;

  @Column({ default: 'user' })
  role: 'user' | 'admin';

  @Column({ type: 'json', nullable: true })
  profile: {
    avatar_url?: string;
    bio?: string;
    favorite_genre?: string;
  };

  @Column({ type: 'json', nullable: true })
  preferences: {
    theme: 'light' | 'dark';
    language: string;
    notifications_enabled: boolean;
  };

  @Column({ default: false })
  is_deleted: boolean;

  @CreateDateColumn()
  created_at: Date;

  @UpdateDateColumn()
  updated_at: Date;
}
```

**Step 2: Create Users service**

Create `services/nestjs-bridge/src/users/users.service.ts`:
```typescript
import { Injectable, ConflictException, NotFoundException } from '@nestjs/common';
import { InjectRepository } from '@nestjs/typeorm';
import { Repository } from 'typeorm';
import { User } from './entities/user.entity';
import * as bcrypt from 'bcrypt';

@Injectable()
export class UsersService {
  constructor(
    @InjectRepository(User)
    private usersRepository: Repository<User>,
  ) {}

  async create(email: string, username: string, password: string) {
    const existing = await this.usersRepository.findOne({
      where: [{ email }, { username }],
    });

    if (existing) {
      throw new ConflictException('Email or username already exists');
    }

    const password_hash = await bcrypt.hash(password, 10);

    const user = this.usersRepository.create({
      email,
      username,
      password_hash,
      role: 'user',
      preferences: {
        theme: 'dark',
        language: 'en',
        notifications_enabled: true,
      },
    });

    return this.usersRepository.save(user);
  }

  async findById(id: string) {
    const user = await this.usersRepository.findOne({ where: { id } });
    if (!user) {
      throw new NotFoundException('User not found');
    }
    return user;
  }

  async findByEmail(email: string) {
    return this.usersRepository.findOne({ where: { email } });
  }

  async update(id: string, updateData: Partial<User>) {
    await this.findById(id);
    await this.usersRepository.update(id, updateData);
    return this.findById(id);
  }

  async verifyPassword(user: User, password: string): Promise<boolean> {
    return bcrypt.compare(password, user.password_hash);
  }
}
```

**Step 3: Create Users module**

Create `services/nestjs-bridge/src/users/users.module.ts`:
```typescript
import { Module } from '@nestjs/common';
import { TypeOrmModule } from '@nestjs/typeorm';
import { User } from './entities/user.entity';
import { UsersService } from './users.service';

@Module({
  imports: [TypeOrmModule.forFeature([User])],
  providers: [UsersService],
  exports: [UsersService],
})
export class UsersModule {}
```

**Step 4: Create Auth module (basic structure)**

Create `services/nestjs-bridge/src/auth/auth.module.ts`:
```typescript
import { Module } from '@nestjs/common';
import { JwtModule } from '@nestjs/jwt';
import { PassportModule } from '@nestjs/passport';
import { AuthService } from './auth.service';
import { AuthController } from './auth.controller';
import { JwtStrategy } from './jwt.strategy';
import { UsersModule } from '../users/users.module';

@Module({
  imports: [
    UsersModule,
    PassportModule,
    JwtModule.register({
      secret: process.env.JWT_SECRET || 'dev-secret',
      signOptions: { expiresIn: process.env.JWT_EXPIRATION || '3600s' },
    }),
  ],
  providers: [AuthService, JwtStrategy],
  controllers: [AuthController],
})
export class AuthModule {}
```

**Step 5: Update AppModule to include Auth**

Modify `services/nestjs-bridge/src/app.module.ts` to import `AuthModule`:
```typescript
// Add to imports:
import { AuthModule } from './auth/auth.module';

// Update the Module decorator:
@Module({
  imports: [
    ConfigModule.forRoot({...}),
    TypeOrmModule.forRoot({...}),
    UsersModule,
    AuthModule,  // Add this
  ],
})
```

**Step 6: Commit**

```bash
git add services/nestjs-bridge/src/users/ services/nestjs-bridge/src/auth/auth.module.ts services/nestjs-bridge/src/app.module.ts
git commit -m "feat(auth): add User entity and authentication module structure"
```

**Expected:** User table schema defined, bcrypt hashing configured, services ready for auth endpoints.

---

### Task 7: Implement JWT Authentication Service

**Files:**
- Create: `services/nestjs-bridge/src/auth/auth.service.ts`
- Create: `services/nestjs-bridge/src/auth/jwt.strategy.ts`
- Create: `services/nestjs-bridge/src/auth/jwt-auth.guard.ts`

**Step 1: Create Auth service**

Create `services/nestjs-bridge/src/auth/auth.service.ts`:
```typescript
import { Injectable, UnauthorizedException } from '@nestjs/common';
import { JwtService } from '@nestjs/jwt';
import { UsersService } from '../users/users.service';

@Injectable()
export class AuthService {
  constructor(
    private jwtService: JwtService,
    private usersService: UsersService,
  ) {}

  async validateUser(email: string, password: string) {
    const user = await this.usersService.findByEmail(email);
    if (!user) {
      throw new UnauthorizedException('Invalid credentials');
    }

    const isPasswordValid = await this.usersService.verifyPassword(user, password);
    if (!isPasswordValid) {
      throw new UnauthorizedException('Invalid credentials');
    }

    return user;
  }

  async login(user: any) {
    const payload = {
      sub: user.id,
      email: user.email,
      username: user.username,
      role: user.role,
    };

    return {
      access_token: this.jwtService.sign(payload),
      refresh_token: this.jwtService.sign(payload, { expiresIn: '7d' }),
      expires_in: parseInt(process.env.JWT_EXPIRATION || '3600'),
      user: {
        id: user.id,
        email: user.email,
        username: user.username,
        role: user.role,
      },
    };
  }

  async register(email: string, username: string, password: string) {
    const user = await this.usersService.create(email, username, password);
    return this.login(user);
  }

  async refresh(user: any) {
    return this.login(user);
  }
}
```

**Step 2: Create JWT Strategy**

Create `services/nestjs-bridge/src/auth/jwt.strategy.ts`:
```typescript
import { Injectable } from '@nestjs/common';
import { PassportStrategy } from '@nestjs/passport';
import { ExtractJwt, Strategy } from 'passport-jwt';
import { UsersService } from '../users/users.service';

@Injectable()
export class JwtStrategy extends PassportStrategy(Strategy) {
  constructor(private usersService: UsersService) {
    super({
      jwtFromRequest: ExtractJwt.fromAuthHeaderAsBearerToken(),
      ignoreExpiration: false,
      secretOrKey: process.env.JWT_SECRET || 'dev-secret',
    });
  }

  async validate(payload: any) {
    const user = await this.usersService.findById(payload.sub);
    return user;
  }
}
```

**Step 3: Create JWT Auth Guard**

Create `services/nestjs-bridge/src/auth/jwt-auth.guard.ts`:
```typescript
import { Injectable } from '@nestjs/common';
import { AuthGuard } from '@nestjs/passport';

@Injectable()
export class JwtAuthGuard extends AuthGuard('jwt') {}

@Injectable()
export class AdminGuard extends AuthGuard('jwt') {
  handleRequest(err: any, user: any) {
    if (err || !user || user.role !== 'admin') {
      throw new Error('Unauthorized: Admin access required');
    }
    return user;
  }
}
```

**Step 4: Commit**

```bash
git add services/nestjs-bridge/src/auth/auth.service.ts services/nestjs-bridge/src/auth/jwt.strategy.ts services/nestjs-bridge/src/auth/jwt-auth.guard.ts
git commit -m "feat(auth): implement JWT strategy and authentication guards"
```

**Expected:** JWT token generation and validation working.

---

### Task 8: Create Auth Endpoints (Login, Register, Refresh)

**Files:**
- Create: `services/nestjs-bridge/src/auth/auth.controller.ts`
- Create: `services/nestjs-bridge/src/auth/dtos/login.dto.ts`
- Create: `services/nestjs-bridge/src/auth/dtos/register.dto.ts`

**Step 1: Create DTOs**

Create `services/nestjs-bridge/src/auth/dtos/login.dto.ts`:
```typescript
import { IsEmail, IsNotEmpty, MinLength } from 'class-validator';

export class LoginDto {
  @IsEmail()
  email: string;

  @IsNotEmpty()
  @MinLength(6)
  password: string;
}
```

Create `services/nestjs-bridge/src/auth/dtos/register.dto.ts`:
```typescript
import { IsEmail, IsNotEmpty, MinLength } from 'class-validator';

export class RegisterDto {
  @IsEmail()
  email: string;

  @IsNotEmpty()
  @MinLength(3)
  username: string;

  @IsNotEmpty()
  @MinLength(6)
  password: string;
}
```

**Step 2: Create Auth controller**

Create `services/nestjs-bridge/src/auth/auth.controller.ts`:
```typescript
import { Controller, Post, Body, UseGuards, Request, Get } from '@nestjs/common';
import { AuthService } from './auth.service';
import { LoginDto } from './dtos/login.dto';
import { RegisterDto } from './dtos/register.dto';
import { JwtAuthGuard } from './jwt-auth.guard';

@Controller('auth')
export class AuthController {
  constructor(private authService: AuthService) {}

  @Post('register')
  async register(@Body() registerDto: RegisterDto) {
    return this.authService.register(
      registerDto.email,
      registerDto.username,
      registerDto.password,
    );
  }

  @Post('login')
  async login(@Body() loginDto: LoginDto) {
    const user = await this.authService.validateUser(loginDto.email, loginDto.password);
    return this.authService.login(user);
  }

  @Post('refresh')
  @UseGuards(JwtAuthGuard)
  async refresh(@Request() req) {
    return this.authService.refresh(req.user);
  }

  @Get('me')
  @UseGuards(JwtAuthGuard)
  me(@Request() req) {
    return {
      id: req.user.id,
      email: req.user.email,
      username: req.user.username,
      role: req.user.role,
    };
  }
}
```

**Step 3: Create test for auth controller**

Create `services/nestjs-bridge/src/auth/auth.controller.spec.ts`:
```typescript
import { Test, TestingModule } from '@nestjs/testing';
import { AuthController } from './auth.controller';
import { AuthService } from './auth.service';

describe('AuthController', () => {
  let controller: AuthController;
  let authService: AuthService;

  beforeEach(async () => {
    const module: TestingModule = await Test.createTestingModule({
      controllers: [AuthController],
      providers: [
        {
          provide: AuthService,
          useValue: {
            register: jest.fn(),
            login: jest.fn(),
            refresh: jest.fn(),
          },
        },
      ],
    }).compile();

    controller = module.get<AuthController>(AuthController);
    authService = module.get<AuthService>(AuthService);
  });

  it('should be defined', () => {
    expect(controller).toBeDefined();
  });

  it('should call register on AuthService', async () => {
    const registerDto = { email: 'test@test.com', username: 'testuser', password: 'password123' };
    jest.spyOn(authService, 'register').mockResolvedValue({} as any);

    await controller.register(registerDto);
    expect(authService.register).toHaveBeenCalledWith('test@test.com', 'testuser', 'password123');
  });
});
```

**Step 4: Update AppModule to add ValidationPipe**

Modify `services/nestjs-bridge/src/main.ts`:
```typescript
import { ValidationPipe } from '@nestjs/common';

async function bootstrap() {
  const app = await NestFactory.create(AppModule);

  app.useGlobalPipes(
    new ValidationPipe({
      whitelist: true,
      forbidNonWhitelisted: true,
      transform: true,
    }),
  );

  app.enableCors({
    origin: process.env.FRONTEND_URL || 'http://localhost:5173',
    credentials: true,
  });

  const port = process.env.PORT || 3001;
  await app.listen(port);
  console.log(`🚀 NestJS Bridge running on http://localhost:${port}`);
}

bootstrap();
```

**Step 5: Run tests**

```bash
cd services/nestjs-bridge
npm run test -- auth.controller.spec
# Expected: Test passes
```

**Step 6: Commit**

```bash
git add services/nestjs-bridge/src/auth/auth.controller.ts services/nestjs-bridge/src/auth/dtos/ services/nestjs-bridge/src/main.ts services/nestjs-bridge/src/auth/auth.controller.spec.ts
git commit -m "feat(auth): add login, register, and refresh endpoints with validation"
```

**Expected:** Auth endpoints ready and tested.

---

### Task 9: Create Users Endpoints (Profile CRUD)

**Files:**
- Create: `services/nestjs-bridge/src/users/users.controller.ts`
- Create: `services/nestjs-bridge/src/users/dtos/update-user.dto.ts`

**Step 1: Create update user DTO**

Create `services/nestjs-bridge/src/users/dtos/update-user.dto.ts`:
```typescript
import { IsOptional, IsString, MinLength } from 'class-validator';

export class UpdateUserDto {
  @IsOptional()
  @IsString()
  @MinLength(3)
  username?: string;

  @IsOptional()
  @IsString()
  bio?: string;

  @IsOptional()
  @IsString()
  favorite_genre?: string;

  @IsOptional()
  theme?: 'light' | 'dark';
}
```

**Step 2: Create users controller**

Create `services/nestjs-bridge/src/users/users.controller.ts`:
```typescript
import { Controller, Get, Put, Delete, UseGuards, Request, Body, Param } from '@nestjs/common';
import { UsersService } from './users.service';
import { JwtAuthGuard } from '../auth/jwt-auth.guard';
import { UpdateUserDto } from './dtos/update-user.dto';

@Controller('users')
export class UsersController {
  constructor(private usersService: UsersService) {}

  @Get('me')
  @UseGuards(JwtAuthGuard)
  async getMe(@Request() req) {
    return {
      id: req.user.id,
      email: req.user.email,
      username: req.user.username,
      profile: req.user.profile,
      preferences: req.user.preferences,
    };
  }

  @Get(':id')
  @UseGuards(JwtAuthGuard)
  async getUser(@Param('id') id: string) {
    const user = await this.usersService.findById(id);
    return {
      id: user.id,
      username: user.username,
      profile: user.profile,
    };
  }

  @Put(':id')
  @UseGuards(JwtAuthGuard)
  async updateUser(@Param('id') id: string, @Body() updateUserDto: UpdateUserDto, @Request() req) {
    if (req.user.id !== id && req.user.role !== 'admin') {
      throw new Error('Forbidden: Cannot update other users');
    }

    const updateData: any = {};
    if (updateUserDto.username) updateData.username = updateUserDto.username;
    if (updateUserDto.bio || updateUserDto.favorite_genre) {
      updateData.profile = {
        ...req.user.profile,
        ...(updateUserDto.bio && { bio: updateUserDto.bio }),
        ...(updateUserDto.favorite_genre && { favorite_genre: updateUserDto.favorite_genre }),
      };
    }
    if (updateUserDto.theme) {
      updateData.preferences = {
        ...req.user.preferences,
        theme: updateUserDto.theme,
      };
    }

    const updated = await this.usersService.update(id, updateData);
    return {
      id: updated.id,
      email: updated.email,
      username: updated.username,
      profile: updated.profile,
      preferences: updated.preferences,
    };
  }

  @Delete(':id')
  @UseGuards(JwtAuthGuard)
  async deleteUser(@Param('id') id: string, @Request() req) {
    if (req.user.id !== id && req.user.role !== 'admin') {
      throw new Error('Forbidden: Cannot delete other users');
    }

    await this.usersService.update(id, { is_deleted: true });
    return { message: 'User deleted successfully' };
  }
}
```

**Step 3: Update users module to export controller**

Modify `services/nestjs-bridge/src/users/users.module.ts`:
```typescript
import { Module } from '@nestjs/common';
import { TypeOrmModule } from '@nestjs/typeorm';
import { User } from './entities/user.entity';
import { UsersService } from './users.service';
import { UsersController } from './users.controller';

@Module({
  imports: [TypeOrmModule.forFeature([User])],
  providers: [UsersService],
  controllers: [UsersController],
  exports: [UsersService],
})
export class UsersModule {}
```

**Step 4: Commit**

```bash
git add services/nestjs-bridge/src/users/users.controller.ts services/nestjs-bridge/src/users/dtos/ services/nestjs-bridge/src/users/users.module.ts
git commit -m "feat(users): add profile CRUD endpoints with role-based access"
```

**Expected:** User profile endpoints working with authorization.

---

### Task 10: Create Login Page (SvelteKit Frontend)

**Files:**
- Create: `services/sveltekit-frontend/src/routes/login/+page.svelte`
- Create: `services/sveltekit-frontend/src/lib/api.ts`
- Create: `services/sveltekit-frontend/src/stores/auth.ts`

**Step 1: Create API client**

Create `services/sveltekit-frontend/src/lib/api.ts`:
```typescript
const API_URL = process.env.PUBLIC_API_URL || 'http://localhost:3001';

export async function apiCall(endpoint: string, options: any = {}) {
  const token = typeof window !== 'undefined' ? localStorage.getItem('access_token') : null;

  const headers: any = {
    'Content-Type': 'application/json',
    ...options.headers,
  };

  if (token) {
    headers['Authorization'] = `Bearer ${token}`;
  }

  const response = await fetch(`${API_URL}${endpoint}`, {
    ...options,
    headers,
  });

  if (!response.ok) {
    throw new Error(`API error: ${response.status}`);
  }

  return response.json();
}

export async function login(email: string, password: string) {
  return apiCall('/auth/login', {
    method: 'POST',
    body: JSON.stringify({ email, password }),
  });
}

export async function register(email: string, username: string, password: string) {
  return apiCall('/auth/register', {
    method: 'POST',
    body: JSON.stringify({ email, username, password }),
  });
}

export async function getMe() {
  return apiCall('/users/me');
}
```

**Step 2: Create auth store**

Create `services/sveltekit-frontend/src/stores/auth.ts`:
```typescript
import { writable } from 'svelte/store';

interface AuthState {
  user: any | null;
  isLoggedIn: boolean;
  token: string | null;
}

function createAuthStore() {
  const { subscribe, set, update } = writable<AuthState>({
    user: null,
    isLoggedIn: false,
    token: null,
  });

  return {
    subscribe,
    login: (user: any, token: string) => {
      localStorage.setItem('access_token', token);
      set({ user, isLoggedIn: true, token });
    },
    logout: () => {
      localStorage.removeItem('access_token');
      set({ user: null, isLoggedIn: false, token: null });
    },
    setUser: (user: any) => update(state => ({ ...state, user })),
  };
}

export const authStore = createAuthStore();
```

**Step 3: Create login page**

Create `services/sveltekit-frontend/src/routes/login/+page.svelte`:
```svelte
<script lang="ts">
  import { goto } from '$app/navigation';
  import { login, register } from '$lib/api';
  import { authStore } from '../../stores/auth';

  let isLogin = true;
  let email = '';
  let username = '';
  let password = '';
  let error = '';
  let loading = false;

  async function handleSubmit() {
    loading = true;
    error = '';

    try {
      let result;
      if (isLogin) {
        result = await login(email, password);
      } else {
        result = await register(email, username, password);
      }

      authStore.login(result.user, result.access_token);
      await goto('/recommendations');
    } catch (err: any) {
      error = err.message || 'Authentication failed';
    } finally {
      loading = false;
    }
  }

  function toggleMode() {
    isLogin = !isLogin;
    error = '';
  }
</script>

<div class="min-h-screen bg-black text-white flex items-center justify-center p-4">
  <div class="w-full max-w-md">
    <h1 class="text-4xl font-black text-center mb-8 text-yellow-300">
      ANIME.VEC
    </h1>

    <form on:submit|preventDefault={handleSubmit} class="space-y-4 border-2 border-yellow-300 p-8">
      {#if !isLogin}
        <div>
          <label class="block text-sm font-bold mb-2">Username</label>
          <input
            type="text"
            bind:value={username}
            required
            class="w-full bg-gray-900 border-2 border-purple-400 p-3 text-white font-bold"
          />
        </div>
      {/if}

      <div>
        <label class="block text-sm font-bold mb-2">Email</label>
        <input
          type="email"
          bind:value={email}
          required
          class="w-full bg-gray-900 border-2 border-purple-400 p-3 text-white font-bold"
        />
      </div>

      <div>
        <label class="block text-sm font-bold mb-2">Password</label>
        <input
          type="password"
          bind:value={password}
          required
          class="w-full bg-gray-900 border-2 border-purple-400 p-3 text-white font-bold"
        />
      </div>

      {#if error}
        <div class="bg-red-900 border-2 border-red-400 p-3 text-red-100">
          {error}
        </div>
      {/if}

      <button
        type="submit"
        disabled={loading}
        class="w-full bg-yellow-300 text-black font-bold py-3 hover:bg-yellow-200 disabled:opacity-50"
      >
        {loading ? 'Loading...' : isLogin ? 'Login' : 'Register'}
      </button>

      <button
        type="button"
        on:click={toggleMode}
        class="w-full text-purple-400 font-bold py-2 hover:text-purple-300"
      >
        {isLogin ? 'Need an account? Register' : 'Already have an account? Login'}
      </button>
    </form>
  </div>
</div>
```

**Step 4: Commit**

```bash
git add services/sveltekit-frontend/src/lib/api.ts services/sveltekit-frontend/src/stores/auth.ts services/sveltekit-frontend/src/routes/login/+page.svelte
git commit -m "feat(frontend): add login/register page with authentication"
```

**Expected:** Login page functional with API integration.

---

### Task 11: Create Protected Routes & Layout (SvelteKit)

**Files:**
- Create: `services/sveltekit-frontend/src/routes/+layout.svelte`
- Create: `services/sveltekit-frontend/src/routes/+page.svelte` (updated)
- Create: `services/sveltekit-frontend/src/routes/recommendations/+page.svelte`

**Step 1: Create root layout with navigation**

Create `services/sveltekit-frontend/src/routes/+layout.svelte`:
```svelte
<script lang="ts">
  import { goto } from '$app/navigation';
  import { authStore } from '../stores/auth';
  import '../app.css';

  let isLoggedIn = false;

  authStore.subscribe(state => {
    isLoggedIn = state.isLoggedIn;
  });

  async function handleLogout() {
    authStore.logout();
    await goto('/');
  }
</script>

<div class="min-h-screen bg-black text-white">
  <nav class="border-b-2 border-yellow-300 p-4 flex justify-between items-center">
    <a href="/" class="text-2xl font-black text-yellow-300">ANIME.VEC</a>
    
    <div class="flex gap-4 items-center">
      {#if isLoggedIn}
        <a href="/discovery" class="font-bold hover:text-yellow-300">Discovery</a>
        <a href="/recommendations" class="font-bold hover:text-yellow-300">Recommendations</a>
        <a href="/profile" class="font-bold hover:text-yellow-300">Profile</a>
        <button
          on:click={handleLogout}
          class="bg-purple-600 px-4 py-2 font-bold hover:bg-purple-500"
        >
          Logout
        </button>
      {:else}
        <a href="/login" class="btn-primary">Login</a>
      {/if}
    </div>
  </nav>

  <main class="p-4">
    <slot />
  </main>
</div>
```

**Step 2: Update home page**

Update `services/sveltekit-frontend/src/routes/+page.svelte`:
```svelte
<script>
  import { authStore } from '../stores/auth';
  
  let isLoggedIn = false;
  authStore.subscribe(state => {
    isLoggedIn = state.isLoggedIn;
  });
</script>

<div class="min-h-screen flex flex-col items-center justify-center">
  <h1 class="text-6xl font-black mb-4 text-yellow-300">ANIME.VEC</h1>
  <p class="text-2xl mb-8 text-purple-400">Vectorial Recommendation Engine</p>
  
  {#if !isLoggedIn}
    <a href="/login" class="btn-primary text-lg">
      GET STARTED
    </a>
  {:else}
    <a href="/recommendations" class="btn-primary text-lg">
      VIEW RECOMMENDATIONS
    </a>
  {/if}
</div>
```

**Step 3: Create recommendations page (stub)**

Create `services/sveltekit-frontend/src/routes/recommendations/+page.svelte`:
```svelte
<script>
  import { authStore } from '../../stores/auth';
  
  let user = null;
  authStore.subscribe(state => {
    user = state.user;
  });
</script>

<div>
  <h1 class="text-4xl font-black mb-6 text-yellow-300">Your Recommendations</h1>
  
  {#if user}
    <p class="text-lg mb-4">Welcome, <span class="text-purple-400 font-bold">{user.username}</span>!</p>
    <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
      <!-- Recommendations will go here -->
      <div class="card">
        <p>Loading recommendations from Rust engine...</p>
      </div>
    </div>
  {:else}
    <p>Please log in to see recommendations</p>
  {/if}
</div>
```

**Step 4: Commit**

```bash
git add services/sveltekit-frontend/src/routes/+layout.svelte services/sveltekit-frontend/src/routes/+page.svelte services/sveltekit-frontend/src/routes/recommendations/+page.svelte
git commit -m "feat(frontend): add protected routes and main layout with navigation"
```

**Expected:** Navigation and protected routes in place.

---

## Phase 2 Checkpoint

After Task 11, verify:
- ✅ NestJS backend builds and runs
- ✅ Database migrations applied (PostgreSQL user table created)
- ✅ Auth endpoints tested (POST /auth/login, POST /auth/register, POST /auth/refresh)
- ✅ SvelteKit frontend builds and runs
- ✅ Login page connects to NestJS API
- ✅ Protected routes redirect to login when not authenticated
- ✅ Docker Compose brings up all services with health checks

**Build & Test Commands:**
```bash
# Backend
cd services/nestjs-bridge
npm run build
npm run test

# Frontend
cd ../sveltekit-frontend
npm run build

# Full stack
cd ../..
docker-compose up -d
sleep 20
docker-compose ps
# Expected: All services healthy
```

---

## Phase 3: Recommendations Integration (4 tasks)

### Task 12: Create Recommendations Module (NestJS)

**Files:**
- Create: `services/nestjs-bridge/src/recommendations/recommendations.module.ts`
- Create: `services/nestjs-bridge/src/recommendations/recommendations.service.ts`
- Create: `services/nestjs-bridge/src/recommendations/recommendations.controller.ts`
- Create: `services/nestjs-bridge/src/recommendations/dtos/recommendations.dto.ts`

**[Detailed implementation follows same pattern as Auth/Users modules]**

**Key Implementation:**
- Call Rust API: `GET http://rust-engine:3000/api/recommendations/:id`
- Enrich response with user's watchlist status
- Cache responses (Redis, 10min TTL)
- Return transformed DTO

---

### Task 13: Create Watchlist Module (NestJS)

**Files:**
- Create: `services/nestjs-bridge/src/watchlist/watchlist.entity.ts`
- Create: `services/nestjs-bridge/src/watchlist/watchlist.module.ts`
- Create: `services/nestjs-bridge/src/watchlist/watchlist.service.ts`
- Create: `services/nestjs-bridge/src/watchlist/watchlist.controller.ts`

**[PostgreSQL persistence for user's saved anime]**

---

### Task 14: Connect Recommendations Frontend

**Files:**
- Update: `services/sveltekit-frontend/src/routes/recommendations/+page.svelte`
- Create: `services/sveltekit-frontend/src/components/AnimeCard.svelte`
- Create: `services/sveltekit-frontend/src/stores/recommendations.ts`

---

### Task 15: Create Watchlist UI

**Files:**
- Update: `services/sveltekit-frontend/src/components/AnimeCard.svelte`
- Create: `services/sveltekit-frontend/src/routes/profile/+page.svelte`

---

## Phase 4: Search & Filtering (3 tasks)

### Task 16: Create Search Module (NestJS)

**Files:**
- Create: `services/nestjs-bridge/src/search/search.module.ts`
- Create: `services/nestjs-bridge/src/search/search.service.ts`
- Create: `services/nestjs-bridge/src/search/search.controller.ts`

---

### Task 17: Create Discovery Page (Frontend)

**Files:**
- Create: `services/sveltekit-frontend/src/routes/discovery/+page.svelte`
- Create: `services/sveltekit-frontend/src/components/SearchBar.svelte`

---

### Task 18: Add Filtering & Pagination

**Files:**
- Update: `services/nestjs-bridge/src/search/search.service.ts` (add filters)
- Update: `services/sveltekit-frontend/src/routes/discovery/+page.svelte` (add UI)

---

## Phase 5: Admin Dashboard (2 tasks)

### Task 19: Proxy Admin Endpoints (NestJS)

**Files:**
- Create: `services/nestjs-bridge/src/admin/admin.module.ts`
- Create: `services/nestjs-bridge/src/admin/admin.service.ts`
- Create: `services/nestjs-bridge/src/admin/admin.controller.ts`

---

### Task 20: Admin Dashboard Page (Frontend)

**Files:**
- Create: `services/sveltekit-frontend/src/routes/admin/+page.svelte`

---

## Phase 6: Docker & Deployment (3 tasks)

### Task 21: Update Docker Compose & Verify Services

**Files:**
- Verify: Root `docker-compose.yml` (all services, health checks, dependencies)
- Create: `.github/workflows/docker-build.yml` (CI/CD pipeline)

---

### Task 22: Integration Tests

**Files:**
- Create: `services/nestjs-bridge/test/auth.integration.spec.ts`
- Create: `services/sveltekit-frontend/src/test/login.spec.ts`

---

### Task 23: Documentation & Production Ready

**Files:**
- Create: `DEVELOPMENT.md` (how to run locally)
- Create: `DEPLOYMENT.md` (production setup)
- Update: Root `README.md` with frontend architecture

---

## Phase 7: Testing & Polish (Ongoing)

### Task 24-30: Full E2E Testing, Performance, Documentation

---

## Implementation Notes

### Code Patterns

**NestJS Service Template:**
```typescript
@Injectable()
export class FeatureService {
  constructor(
    @InjectRepository(Entity)
    private repo: Repository<Entity>,
  ) {}

  async create(data: CreateDto) { }
  async findById(id: string) { }
  async update(id: string, data: UpdateDto) { }
  async delete(id: string) { }
}
```

**SvelteKit Page Template:**
```svelte
<script>
  import { store } from '$stores/name';
  let data = null;
  store.subscribe(value => { data = value; });
</script>
```

### Testing Strategy

**NestJS:**
- Unit tests for services (mocked repos)
- Integration tests for controllers (real DB)
- E2E tests for user flows (full stack)

**SvelteKit:**
- Component tests (Vitest + JSDOM)
- E2E tests (Playwright)

### Error Handling

**NestJS:** Consistent error responses (status, code, message)
**SvelteKit:** Toast notifications, fallback UI, retry buttons

---

## Success Criteria

**By end of Phase 6:**
- ✅ All three services running together in Docker
- ✅ User can register → login → view recommendations → save watchlist
- ✅ Admin can access system stats
- ✅ Search and filtering working
- ✅ Caching reduces Neo4j load 60-70%
- ✅ Full E2E test coverage for critical flows

---

**Status: Implementation Plan Ready**

Generated: 2026-03-24
Execution Model: Subagent-driven per task with checkpoint reviews
Tech Stack: SvelteKit (frontend), NestJS (bridge), Rust (engine)
