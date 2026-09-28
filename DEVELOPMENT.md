# Cache - Development & Testing Environment

## Overview
This document outlines the development environment, tools, testing strategies, and deployment workflows for the Cache distributed geospatial system.

---

## Development Machine Requirements

### Minimum Specifications (Basic Development)
- **CPU**: 4 cores / 8 threads (for parallel compilation)
- **RAM**: 16 GB (8 GB for OS/tools, 4 GB for Flutter, 4 GB for databases)
- **Storage**: 100 GB free SSD
  - 30 GB for development tools (Rust, Flutter, databases)
  - 20 GB for workspace and source code
  - 50 GB for test tile cache and sample data
- **OS**: Linux (Ubuntu 22.04+ recommended), macOS 12+, or Windows 11 with WSL2
- **Network**: Stable internet (10+ Mbps for tile downloads)

### Recommended Specifications (Full-Stack Development)
- **CPU**: 8+ cores (Ryzen 7 / Intel i7 or better)
- **RAM**: 32 GB
  - 16 GB for development tools
  - 8 GB for databases and tile server
  - 8 GB for multiple Flutter instances and profiling
- **Storage**: 500 GB SSD
  - 50 GB for development tools
  - 50 GB for workspace
  - 400 GB for realistic tile cache testing (mirrors production)
- **GPU**: Integrated graphics sufficient; dedicated GPU helps with:
  - Map rendering performance testing
  - Multiple monitor setup
  - 3D terrain visualization development
- **Network**: Fast internet (100+ Mbps) for:
  - Rapid tile downloads
  - Multiple simultaneous API testing
  - Docker image pulls

### Optimal Specifications (Production Testing)
- **CPU**: 16+ cores (for simulating multi-client scenarios)
- **RAM**: 64 GB
- **Storage**: 1-2 TB NVMe SSD
  - 100 GB development tools
  - 100 GB workspace
  - 800+ GB for full server simulation (4 TB dataset testing)
  - Separate drive for database performance testing
- **Network**: Gigabit ethernet for local network testing
- **Optional**: Multiple machines for distributed testing

### Storage Breakdown by Development Scenario

**Scenario 1: Frontend-Only Development (Mobile/Desktop UI)**
```
30 GB  - Flutter SDK, Android SDK, dependencies
10 GB  - Test tile cache (sample region)
10 GB  - Workspace and builds
---
50 GB total minimum
```

**Scenario 2: Backend-Only Development (Rust API Server)**
```
10 GB  - Rust toolchain, dependencies, target/ builds
50 GB  - Test database with sample data
20 GB  - Tile processing and cache testing
20 GB  - Workspace
---
100 GB total minimum
```

**Scenario 3: Full-Stack Development (Complete System)**
```
30 GB  - All language toolchains (Rust, Flutter, Node if needed)
100 GB - Test databases (PostgreSQL + sample data)
200 GB - Realistic tile cache for integration testing
50 GB  - Workspace, logs, temporary files
20 GB  - Docker images and containers
---
400 GB recommended
```

**Scenario 4: Server Development (Production Simulation)**
```
50 GB  - Development tools
100 GB - Workspace
800 GB - Tile cache approximating 4 TB server (20% scale)
50 GB  - Database with realistic multi-client data
---
1 TB recommended (allows testing at scale)
```

---

## Core Development Tools

### 1. Backend (Rust)

**Required:**
- **Rust**: Version 1.75+ ([installation guide](https://rustup.rs/))
  ```bash
  # Install via rustup
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

  # Verify installation
  rustc --version
  cargo --version
  ```

- **PostgreSQL**: Version 15+ with PostGIS 3.3+
  ```bash
  # Ubuntu/Debian
  sudo apt install postgresql-15 postgresql-15-postgis-3 libpq-dev

  # macOS (Homebrew)
  brew install postgresql@15 postgis
  ```

- **SpatiaLite**: Version 5.0+
  ```bash
  # Ubuntu/Debian
  sudo apt install libsqlite3-mod-spatialite spatialite-bin libsqlite3-dev

  # macOS
  brew install libspatialite
  ```

**Recommended Rust Tools:**
```bash
# Code formatting (included with Rust)
rustup component add rustfmt

# Linting
rustup component add clippy

# Live reload during development
cargo install cargo-watch

# Database migrations
cargo install sqlx-cli --no-default-features --features postgres,sqlite

# Better cargo commands
cargo install cargo-edit  # cargo add, cargo rm
cargo install cargo-outdated  # Check for outdated dependencies
cargo install cargo-audit  # Security vulnerability scanning

# Code coverage
cargo install cargo-tarpaulin
```

### 2. Frontend (Flutter)

**Required:**
- **Flutter SDK**: Version 3.24+ (stable channel)
  ```bash
  # Linux setup
  git clone https://github.com/flutter/flutter.git -b stable
  export PATH="$PATH:`pwd`/flutter/bin"
  flutter doctor
  ```

- **Dart SDK**: Included with Flutter
- **Android Studio**: For Android development (includes Android SDK)
- **Android SDK**: API level 24+ (Android 7.0)
- **Linux Development** (for desktop builds):
  ```bash
  # Ubuntu/Debian
  sudo apt install clang cmake ninja-build pkg-config libgtk-3-dev
  ```

**Recommended Flutter Tools:**
```bash
# Code generation for JSON serialization
flutter pub global activate build_runner

# Static analysis
flutter pub global activate dart_code_metrics
```

### 3. Database Tools

**PostgreSQL/PostGIS:**
- **pgAdmin 4**: GUI for database management
- **psql**: Command-line client (included with PostgreSQL)
- **QGIS**: Desktop GIS for visualizing geospatial data ([download](https://qgis.org/))

**SQLite:**
- **DB Browser for SQLite**: GUI for SQLite databases
- **sqlite3**: Command-line tool
- **SpatiaLite GUI**: For viewing geospatial data in SQLite

### 4. Version Control & Collaboration

```bash
# Git (required)
git --version  # Should be 2.30+

# Recommended: Git LFS for storing test data files
git lfs install
```

### 5. API Testing & Development

```bash
# curl (for API testing)
curl --version

# HTTPie (friendlier alternative to curl)
pip install httpie

# Postman or Insomnia (GUI API clients)
# Download from respective websites
```

### 6. Containerization (Optional but Recommended)

```bash
# Docker for containerized development databases
docker --version  # Version 24+
docker-compose --version
```

---

## Development Environment Setup

### Backend Setup

1. **Clone the repository:**
   ```bash
   git clone https://github.com/your-org/cache.git
   cd cache/backend
   ```

2. **Install Rust dependencies:**
   ```bash
   cargo build
   ```

3. **Setup PostgreSQL database:**
   ```bash
   # Create database and user
   sudo -u postgres psql
   ```
   ```sql
   CREATE USER cache_dev WITH PASSWORD 'dev_password';
   CREATE DATABASE cache_dev OWNER cache_dev;
   \c cache_dev
   CREATE EXTENSION postgis;
   CREATE EXTENSION postgis_topology;
   \q
   ```

4. **Run database migrations:**
   ```bash
   # Using sqlx-cli
   sqlx database create
   sqlx migrate run

   # Or using diesel (alternative)
   # diesel setup
   # diesel migration run
   ```

5. **Setup environment variables:**
   ```bash
   cp .env.example .env
   # Edit .env with your configuration
   ```

   Example `.env`:
   ```bash
   DATABASE_URL=postgresql://cache_dev:dev_password@localhost:5432/cache_dev

   SERVER_HOST=127.0.0.1
   SERVER_PORT=8080
   RUST_ENV=development
   RUST_LOG=debug,cache=trace

   TILE_CACHE_DIR=./data/tiles
   TILE_CACHE_SIZE_MB=5120
   ```

6. **Run the development server:**
   ```bash
   # With live reload (watches for changes and rebuilds)
   cargo watch -x run

   # Or standard cargo run
   cargo run --bin cache-server

   # With optimizations (faster runtime, slower compile)
   cargo run --release
   ```

7. **Verify setup:**
   ```bash
   curl http://localhost:8080/health
   # Should return: {"status":"healthy","timestamp":"..."}
   ```

### Frontend Setup

1. **Navigate to frontend directory:**
   ```bash
   cd cache/frontend
   ```

2. **Install Flutter dependencies:**
   ```bash
   flutter pub get
   ```

3. **Enable desktop support (Linux):**
   ```bash
   flutter config --enable-linux-desktop
   ```

4. **Generate code (for JSON serialization, etc.):**
   ```bash
   flutter pub run build_runner build --delete-conflicting-outputs
   ```

5. **Setup environment configuration:**
   ```bash
   cp lib/config/env.example.dart lib/config/env.dart
   # Edit env.dart with your API endpoint
   ```

6. **Run the app:**
   ```bash
   # Linux desktop
   flutter run -d linux

   # Android emulator (ensure emulator is running)
   flutter run -d emulator-5554

   # Physical Android device (enable USB debugging)
   flutter run -d <device-id>

   # Hot reload is automatic - press 'r' to reload, 'R' to restart
   ```

### Docker Development Environment (Recommended)

Use Docker Compose for consistent development databases:

**`docker-compose.dev.yml`:**
```yaml
version: '3.8'

services:
  postgres:
    image: postgis/postgis:15-3.3
    container_name: cache_postgres_dev
    environment:
      POSTGRES_USER: cache_dev
      POSTGRES_PASSWORD: dev_password
      POSTGRES_DB: cache_dev
    ports:
      - "5432:5432"
    volumes:
      - postgres_data:/var/lib/postgresql/data
      - ./scripts/init-db.sql:/docker-entrypoint-initdb.d/init.sql
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U cache_dev"]
      interval: 5s
      timeout: 5s
      retries: 5

  redis:
    image: redis:7-alpine
    container_name: cache_redis_dev
    ports:
      - "6379:6379"
    volumes:
      - redis_data:/data

volumes:
  postgres_data:
  redis_data:
```

**Start services:**
```bash
docker-compose -f docker-compose.dev.yml up -d
```

---

## Project Structure

```
cache/
├── backend/
│   ├── src/
│   │   ├── bin/
│   │   │   ├── server.rs        # Main API server
│   │   │   └── cli.rs           # CLI tools (migration, seed data)
│   │   ├── api/                 # HTTP handlers and routes
│   │   ├── models/              # Data models and schemas
│   │   ├── services/            # Business logic
│   │   ├── repository/          # Database access layer
│   │   ├── sync/                # Sync protocol implementation
│   │   ├── tiles/               # Tile fetching and caching
│   │   ├── config/              # Configuration management
│   │   ├── geospatial/          # Geospatial utilities
│   │   ├── nmea/                # NMEA parser
│   │   └── lib.rs               # Library root
│   ├── migrations/              # Database migrations (SQLx or Diesel)
│   ├── tests/                   # Integration tests
│   ├── benches/                 # Benchmarks
│   ├── Cargo.toml               # Dependencies and package info
│   ├── Cargo.lock
│   └── Makefile
│
├── frontend/
│   ├── lib/
│   │   ├── main.dart
│   │   ├── config/          # Environment configuration
│   │   ├── models/          # Data models (matching backend)
│   │   ├── services/        # API clients, sync logic
│   │   ├── providers/       # State management (Riverpod)
│   │   ├── screens/         # UI screens
│   │   ├── widgets/         # Reusable widgets
│   │   ├── map/             # Map rendering components
│   │   └── utils/           # Helper functions
│   ├── test/                # Unit and widget tests
│   ├── integration_test/    # Integration tests
│   ├── assets/              # Images, fonts, etc.
│   ├── pubspec.yaml
│   └── analysis_options.yaml
│
├── shared/
│   ├── proto/               # Protocol Buffer definitions
│   └── schemas/             # JSON schemas for API contracts
│
├── docs/                    # Documentation
├── scripts/                 # Cross-project scripts
├── docker-compose.dev.yml
├── docker-compose.prod.yml
├── .gitignore
├── LICENSE
├── README.md
├── Requirements.md
└── DEVELOPMENT.md
```

---

## Testing Strategy

### Unit Tests

**Backend (Rust):**
```bash
# Run all tests
cargo test

# Run tests with output (shows println! statements)
cargo test -- --nocapture

# Run tests with coverage
cargo tarpaulin --out Html --output-dir coverage

# Run specific test module
cargo test services::waypoint

# Run tests matching a pattern
cargo test waypoint

# Run with verbose output
cargo test -- --test-threads=1 --nocapture

# Run benchmarks
cargo bench

# Check tests compile without running
cargo test --no-run
```

**Test organization:**
- Place unit tests in same file as source code in `#[cfg(test)]` module
- Place integration tests in `tests/` directory
- Use table-driven tests with macros or loops
- Mock external dependencies using traits

**Example test structure:**
```rust
// src/services/waypoint.rs
pub struct WaypointService<R: WaypointRepository> {
    repo: R,
}

impl<R: WaypointRepository> WaypointService<R> {
    pub fn create(&self, waypoint: &Waypoint) -> Result<Waypoint, Error> {
        // Validate coordinates
        if waypoint.lat < -90.0 || waypoint.lat > 90.0 {
            return Err(Error::InvalidLatitude);
        }
        self.repo.insert(waypoint)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repository::MockWaypointRepository;

    #[test]
    fn test_create_valid_waypoint() {
        let mut mock_repo = MockWaypointRepository::new();
        mock_repo.expect_insert()
            .returning(|w| Ok(w.clone()));

        let service = WaypointService { repo: mock_repo };
        let waypoint = Waypoint {
            name: "Test".into(),
            lat: 45.0,
            lon: -122.0,
            ..Default::default()
        };

        let result = service.create(&waypoint);
        assert!(result.is_ok());
    }

    #[test]
    fn test_create_invalid_latitude() {
        let mock_repo = MockWaypointRepository::new();
        let service = WaypointService { repo: mock_repo };
        let waypoint = Waypoint {
            name: "Test".into(),
            lat: 100.0,  // Invalid
            lon: -122.0,
            ..Default::default()
        };

        let result = service.create(&waypoint);
        assert!(matches!(result, Err(Error::InvalidLatitude)));
    }
}
```

**Frontend (Flutter):**
```bash
# Run all unit and widget tests
flutter test

# Run with coverage
flutter test --coverage
genhtml coverage/lcov.info -o coverage/html

# Run specific test file
flutter test test/services/waypoint_service_test.dart

# Watch mode (re-run on file changes)
flutter test --watch
```

**Example Flutter test:**
```dart
// test/services/waypoint_service_test.dart
import 'package:flutter_test/flutter_test.dart';
import 'package:cache/services/waypoint_service.dart';
import 'package:cache/models/waypoint.dart';

void main() {
  group('WaypointService', () {
    late WaypointService service;

    setUp(() {
      service = WaypointService();
    });

    test('creates waypoint with valid coordinates', () async {
      final waypoint = Waypoint(
        name: 'Test',
        lat: 45.0,
        lon: -122.0,
      );

      final result = await service.create(waypoint);
      expect(result, isNotNull);
      expect(result.id, isNotEmpty);
    });

    test('rejects waypoint with invalid latitude', () {
      final waypoint = Waypoint(
        name: 'Test',
        lat: 100.0,  // Invalid
        lon: -122.0,
      );

      expect(() => service.create(waypoint), throwsArgumentError);
    });
  });
}
```

### Integration Tests

**Backend:**
Create tests that use real database (test database):

```rust
// tests/integration/waypoint_api_test.rs
use cache::api;
use cache::config::Config;
use sqlx::PgPool;
use reqwest;

#[sqlx::test]
async fn test_waypoint_create_and_retrieve(pool: PgPool) -> sqlx::Result<()> {
    // Setup test server
    let config = Config::test_config();
    let app = api::create_router(pool.clone()).await;
    let server = axum::Server::bind(&"127.0.0.1:0".parse().unwrap())
        .serve(app.into_make_service());
    let addr = server.local_addr();

    // Spawn server in background
    tokio::spawn(server);

    // Test POST /api/waypoints
    let client = reqwest::Client::new();
    let waypoint = json!({
        "name": "Test Point",
        "lat": 45.0,
        "lon": -122.0
    });

    let response = client
        .post(format!("http://{}/api/waypoints", addr))
        .json(&waypoint)
        .send()
        .await?;

    assert_eq!(response.status(), 201);

    // Test GET /api/waypoints/:id
    let created: Waypoint = response.json().await?;
    let get_response = client
        .get(format!("http://{}/api/waypoints/{}", addr, created.id))
        .send()
        .await?;

    assert_eq!(get_response.status(), 200);

    Ok(())
}
```

Run integration tests:
```bash
# Integration tests use sqlx::test macro for DB setup
cargo test --test '*'

# With logging
RUST_LOG=debug cargo test --test '*'
```

**Frontend (Flutter):**
```bash
# Integration tests (run on device/emulator)
flutter test integration_test/

# Specific test
flutter drive --driver=test_driver/integration_test.dart --target=integration_test/map_test.dart
```

### End-to-End Tests

**Workflow testing:**
1. Start backend server (test mode)
2. Run Flutter app on emulator
3. Execute automated UI tests using `flutter_driver` or `patrol`

**Example scenarios:**
- User creates waypoint → appears on map → syncs to server
- User creates trajectory → converts to plan → receives weather forecast
- Offline mode → create data → go online → data syncs

### Performance Tests

**Backend load testing** using `vegeta`, `k6`, or `drill`:
```bash
# Using drill (Rust-based load testing)
cargo install drill

# Create benchmark.yml
echo 'concurrency: 50
base: http://localhost:8080
iterations: 1000
rampup: 10

plan:
  - name: Create waypoints
    request:
      url: /api/waypoints
      method: POST
      body: {"name":"Test","lat":45.0,"lon":-122.0}
      headers:
        Content-Type: application/json' > benchmark.yml

# Run load test
drill --benchmark benchmark.yml --stats

# Or using vegeta
echo "POST http://localhost:8080/api/waypoints" | vegeta attack -duration=30s -rate=50 | vegeta report
```

**Frontend performance:**
```bash
# Flutter performance profiling
flutter run --profile

# In DevTools: Analyze frame rendering, memory usage
```

### Test Data Generation

**Backend seed script:**
```bash
# Run seed script to populate test data
cargo run --bin cache-cli -- seed --waypoints=1000 --tracks=100 --beacons=10
```

**Automated test data:**
- Generate realistic GPS tracks using GPX generators
- Create test tile caches with known coverage areas
- Mock weather API responses for consistent testing

**Rust test data generation example:**
```rust
// tests/common/fixtures.rs
use cache::models::*;
use fake::{Fake, Faker};

pub fn create_test_waypoint() -> Waypoint {
    Waypoint {
        id: uuid::Uuid::new_v4(),
        name: Faker.fake(),
        lat: (-90.0..90.0).fake(),
        lon: (-180.0..180.0).fake(),
        altitude: Some((0.0..5000.0).fake()),
        ..Default::default()
    }
}

pub fn create_gps_track(num_points: usize) -> Vec<Track> {
    (0..num_points)
        .map(|i| Track {
            beacon_id: uuid::Uuid::new_v4(),
            lat: 45.0 + (i as f64 * 0.001),
            lon: -122.0 + (i as f64 * 0.001),
            timestamp: chrono::Utc::now() - chrono::Duration::seconds(i as i64 * 60),
            ..Default::default()
        })
        .collect()
}
```

---

## Development Workflow

### Daily Development Loop

1. **Pull latest changes:**
   ```bash
   git pull origin main
   ```

2. **Update dependencies:**
   ```bash
   cd backend && cargo update
   cd ../frontend && flutter pub get
   ```

3. **Start development services:**
   ```bash
   docker-compose -f docker-compose.dev.yml up -d
   ```

4. **Run backend with hot reload:**
   ```bash
   cd backend
   cargo watch -x run  # Watches for changes and recompiles
   ```

5. **Run frontend with hot reload:**
   ```bash
   cd frontend
   flutter run -d linux  # Press 'r' for hot reload, 'R' for hot restart
   ```

6. **Make changes and test:**
   - Edit code
   - Hot reload applies changes automatically
   - Run relevant tests: `cargo test` or `flutter test`

7. **Commit changes:**
   ```bash
   git add .
   git commit -m "feat: add waypoint clustering"
   git push origin feature/waypoint-clustering
   ```

### Code Quality Checks

**Pre-commit hooks** (`.git/hooks/pre-commit`):
```bash
#!/bin/bash

# Backend checks
cd backend
echo "Running Rust linter..."
cargo clippy -- -D warnings
if [ $? -ne 0 ]; then exit 1; fi

echo "Running Rust formatter check..."
cargo fmt -- --check
if [ $? -ne 0 ]; then exit 1; fi

echo "Running Rust tests..."
cargo test
if [ $? -ne 0 ]; then exit 1; fi

# Frontend checks
cd ../frontend
echo "Running Dart analyzer..."
flutter analyze
if [ $? -ne 0 ]; then exit 1; fi

echo "Running Flutter tests..."
flutter test
if [ $? -ne 0 ]; then exit 1; fi

echo "All checks passed!"
```

**Automated formatting:**
```bash
# Rust formatting
cd backend
cargo fmt

# Flutter formatting
cd frontend
dart format .
```

### Code Review Process

1. **Create feature branch:**
   ```bash
   git checkout -b feature/new-feature
   ```

2. **Implement feature with tests**

3. **Push and create pull request:**
   ```bash
   git push origin feature/new-feature
   # Create PR on GitHub/GitLab
   ```

4. **Automated CI checks:**
   - Linting
   - Unit tests
   - Integration tests
   - Coverage report

5. **Manual review:**
   - Code quality
   - Architecture alignment
   - Security considerations
   - Performance implications

6. **Merge after approval:**
   ```bash
   git checkout main
   git pull origin main
   git merge feature/new-feature
   git push origin main
   ```

---

## Debugging

### Backend Debugging

**Using rust-lldb/rust-gdb:**
```bash
# Run with debugger
rust-lldb target/debug/cache-server

# Or with rust-gdb
rust-gdb target/debug/cache-server

# Common commands:
# b main                - Set breakpoint at function
# b src/main.rs:42      - Set breakpoint at line
# r                     - Run
# c                     - Continue
# n                     - Next line
# s                     - Step into
# p variable            - Print variable
# bt                    - Backtrace
```

**VS Code debugging** (`.vscode/launch.json`):
```json
{
  "version": "0.2.0",
  "configurations": [
    {
      "name": "Launch Server",
      "type": "lldb",
      "request": "launch",
      "program": "${workspaceFolder}/backend/target/debug/cache-server",
      "args": [],
      "cwd": "${workspaceFolder}/backend",
      "env": {
        "DATABASE_URL": "postgresql://cache_dev:dev_password@localhost:5432/cache_dev",
        "RUST_LOG": "debug,cache=trace"
      },
      "sourceLanguages": ["rust"]
    }
  ]
}
```

**Logging:**
```rust
// Use structured logging with tracing
use tracing::{info, debug, warn, error};

#[instrument]
pub async fn create_waypoint(waypoint: Waypoint) -> Result<Waypoint, Error> {
    info!(waypoint_name = %waypoint.name, "Creating waypoint");
    debug!(lat = waypoint.lat, lon = waypoint.lon, "Coordinates");

    // ... implementation

    Ok(waypoint)
}

// In main.rs
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info,cache=debug".into())
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // ... rest of main
}
```

### Frontend Debugging

**Flutter DevTools:**
```bash
flutter run -d linux
# Press 'w' to open DevTools in browser
```

**VS Code debugging:**
- Set breakpoints in Dart code
- Press F5 to start debugging
- Use debug console to evaluate expressions

**Logging:**
```dart
import 'package:logger/logger.dart';

final logger = Logger();
logger.d('Debug message');
logger.i('Info message');
logger.w('Warning message');
logger.e('Error message');
```

---

## Continuous Integration / Continuous Deployment (CI/CD)

### GitHub Actions Example

**`.github/workflows/backend.yml`:**
```yaml
name: Backend CI

on:
  push:
    branches: [ main, develop ]
  pull_request:
    branches: [ main ]

jobs:
  test:
    runs-on: ubuntu-latest

    services:
      postgres:
        image: postgis/postgis:15-3.3
        env:
          POSTGRES_USER: cache_test
          POSTGRES_PASSWORD: test_password
          POSTGRES_DB: cache_test
        options: >-
          --health-cmd pg_isready
          --health-interval 10s
          --health-timeout 5s
          --health-retries 5
        ports:
          - 5432:5432

    steps:
    - uses: actions/checkout@v3

    - name: Install system dependencies
      run: |
        sudo apt-get update
        sudo apt-get install -y libsqlite3-dev libpq-dev

    - name: Set up Rust
      uses: dtolnay/rust-toolchain@stable
      with:
        components: rustfmt, clippy

    - name: Cache Rust dependencies
      uses: actions/cache@v3
      with:
        path: |
          ~/.cargo/registry
          ~/.cargo/git
          backend/target
        key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}

    - name: Check formatting
      run: cd backend && cargo fmt -- --check

    - name: Run clippy
      run: cd backend && cargo clippy -- -D warnings

    - name: Run tests
      run: cd backend && cargo test --all-features
      env:
        DATABASE_URL: postgresql://cache_test:test_password@localhost:5432/cache_test
        RUST_LOG: debug

    - name: Build
      run: cd backend && cargo build --release
```

**`.github/workflows/frontend.yml`:**
```yaml
name: Frontend CI

on:
  push:
    branches: [ main, develop ]
  pull_request:
    branches: [ main ]

jobs:
  test:
    runs-on: ubuntu-latest

    steps:
    - uses: actions/checkout@v3

    - name: Set up Flutter
      uses: subosito/flutter-action@v2
      with:
        flutter-version: '3.24.0'
        channel: 'stable'

    - name: Install dependencies
      run: cd frontend && flutter pub get

    - name: Analyze
      run: cd frontend && flutter analyze

    - name: Run tests
      run: cd frontend && flutter test --coverage

    - name: Upload coverage
      uses: codecov/codecov-action@v3
      with:
        files: frontend/coverage/lcov.info
```

---

## Deployment

### Production Build

**Backend:**
```bash
cd backend

# Build for current platform (optimized)
cargo build --release

# Build for Linux x86_64
cargo build --release --target x86_64-unknown-linux-gnu

# Build for ARM64 (Raspberry Pi, etc.)
rustup target add aarch64-unknown-linux-gnu
cargo build --release --target aarch64-unknown-linux-gnu

# Build for ARM (32-bit)
rustup target add armv7-unknown-linux-gnueabihf
cargo build --release --target armv7-unknown-linux-gnueabihf

# Output binary location:
# target/release/cache-server
# Or for cross-compilation:
# target/aarch64-unknown-linux-gnu/release/cache-server

# Strip symbols for smaller binary
strip target/release/cache-server
```

**Frontend:**
```bash
cd frontend

# Android APK
flutter build apk --release

# Android App Bundle (for Play Store)
flutter build appbundle --release

# Linux desktop
flutter build linux --release

# Output locations:
# Android: build/app/outputs/flutter-apk/app-release.apk
# Linux: build/linux/x64/release/bundle/
```

### Docker Production Image

**`Dockerfile.backend`:**
```dockerfile
# Multi-stage build for minimal image size
FROM rust:1.75-slim AS builder

# Install system dependencies
RUN apt-get update && apt-get install -y \
    libpq-dev \
    libsqlite3-dev \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY backend/ .

# Build in release mode
RUN cargo build --release

FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    libpq5 \
    libsqlite3-0 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/target/release/cache-server .

# Copy migrations if needed
COPY --from=builder /app/migrations ./migrations

EXPOSE 8080

ENV RUST_LOG=info
ENV DATABASE_URL=postgresql://cache:cache@postgres:5432/cache

CMD ["./cache-server"]
```

**Alternative: Smaller image using distroless**
```dockerfile
FROM rust:1.75-slim AS builder
# ... same as above ...

FROM gcr.io/distroless/cc-debian12
COPY --from=builder /app/target/release/cache-server /
EXPOSE 8080
CMD ["/cache-server"]
```

Build and run:
```bash
docker build -f Dockerfile.backend -t cache-backend:latest .
docker run -p 8080:8080 \
  -e DATABASE_URL=postgresql://user:pass@host:5432/db \
  cache-backend:latest
```

---

## Recommended Development Hardware

### For Testing GPS/Sensor Integration

- **GPS Receiver**: USB GPS receiver (e.g., GlobalSat BU-353S4, $30-40)
- **RTL-SDR Dongle**: For ADS-B reception (e.g., RTL-SDR Blog V3, $30)
- **Bluetooth Sensors**: BLE temperature/humidity sensors for testing
- **Android Device**: Physical device for mobile testing (emulators lack GPS hardware)

### For Tile Caching Testing

- **Fast SSD**: For rapid tile cache I/O
- **Large storage**: 100GB+ to test realistic cache sizes

---

## Troubleshooting

### Common Issues

**Problem: PostgreSQL connection refused**
```bash
# Check if PostgreSQL is running
sudo systemctl status postgresql

# Check connection settings
psql -h localhost -U cache_dev -d cache_dev
```

**Problem: Flutter can't find Android SDK**
```bash
# Set environment variables
export ANDROID_HOME=$HOME/Android/Sdk
export PATH=$PATH:$ANDROID_HOME/tools:$ANDROID_HOME/platform-tools

# Verify
flutter doctor
```

**Problem: SpatiaLite not loading in SQLite**
```bash
# Test SpatiaLite loading
sqlite3
> .load /usr/lib/x86_64-linux-gnu/mod_spatialite.so
> SELECT InitSpatialMetadata();
```

**Problem: Port 8080 already in use**
```bash
# Find process using port
lsof -i :8080

# Kill process
kill -9 <PID>

# Or change port in .env
SERVER_PORT=8081
```

---

## Additional Resources

### Documentation
- [The Rust Programming Language Book](https://doc.rust-lang.org/book/)
- [Rust Async Book](https://rust-lang.github.io/async-book/)
- [Axum Framework Docs](https://docs.rs/axum/latest/axum/)
- [Flutter Documentation](https://docs.flutter.dev/)
- [PostGIS Manual](https://postgis.net/documentation/)
- [MapLibre GL JS Docs](https://maplibre.org/maplibre-gl-js-docs/api/)

### Tutorials
- [Building a REST API with Rust and Axum](https://dev.to/davidedelpapa/rust-web-development-tutorial-rest-api-with-axum-5dnh)
- [Async Rust](https://rust-lang.github.io/async-book/)
- [SQLx Tutorial](https://github.com/launchbadge/sqlx/blob/main/README.md)
- [Flutter State Management with Riverpod](https://riverpod.dev/docs/introduction/getting_started)
- [Working with Geographic Data in PostgreSQL](https://postgis.net/workshops/postgis-intro/)

### Rust Geospatial Ecosystem
- [geo](https://docs.rs/geo/latest/geo/) - Geospatial primitives and algorithms
- [geozero](https://docs.rs/geozero/latest/geozero/) - Zero-copy geospatial format conversion
- [proj](https://docs.rs/proj/latest/proj/) - Coordinate transformation (PROJ bindings)

### Community
- [r/rust](https://reddit.com/r/rust)
- [Rust Users Forum](https://users.rust-lang.org/)
- [r/FlutterDev](https://reddit.com/r/FlutterDev)
- [PostGIS Users Mailing List](https://lists.osgeo.org/mailman/listinfo/postgis-users)

---

## Next Steps

1. **Set up development environment** following this guide
2. **Review Requirements.md** to understand the full scope
3. **Start with Phase 1 (MVP)** from the development phases
4. **Write tests first** (TDD approach recommended for complex geospatial logic)
5. **Iterate and refine** based on testing and feedback
