# Cache Backend

Rust backend for the Cache distributed offline-first geospatial system.

## Quick Start

### 1. Enter Nix Shell

From the repository root:

```bash
cd /home/chris/development/cache
nix-shell
```

### 2. Build the Project

```bash
cd backend
cargo build
```

### 3. Initialize and Seed the Database

```bash
# Initialize empty database
cargo run --bin cache-cli -- init

# Seed with sample data (including real tiles and weather)
cargo run --bin cache-cli -- seed \
    --waypoints 100 \
    --beacons 10 \
    --trajectories 20 \
    --plans 5 \
    --tracks 1000 \
    --tiles 50 \
    --fetch-weather
```

This will:
- Create 100 waypoints in the San Francisco Bay Area
- Create 10 beacons (vessels, vehicles, etc.)
- Create 20 trajectories connecting waypoints
- Create 5 plans based on trajectories
- Generate 1000 GPS track points
- Fetch 50 real map tiles from OpenFreeMap
- Fetch real weather data from Open-Meteo API

### 4. Explore the Database

```bash
# Using sqlite3
sqlite3 cache.db

# Load SpatiaLite
.load /nix/store/.../mod_spatialite.so

# Query examples
SELECT COUNT(*) FROM waypoints;
SELECT COUNT(*) FROM tiles;
SELECT * FROM active_beacons_latest;
SELECT name, lat, lon, category FROM waypoints LIMIT 10;
```

## Database Schema

The database includes the following tables:

- **waypoints** - Static locations with geospatial coordinates
- **trajectories** - Paths through multiple waypoints
- **plans** - Time-based commitments to trajectories
- **beacons** - Moving objects (vessels, vehicles, people)
- **tracks** - Position history for beacons
- **sensors** - Measurement devices/APIs
- **readings** - Time-series sensor data
- **tiles** - Cached map tiles (MBTiles-compatible)
- **sync_metadata** - Device sync tracking
- **change_log** - Change tracking for sync

## CLI Commands

### Initialize Database

```bash
cargo run --bin cache-cli -- init --database cache.db
```

### Seed Database

```bash
cargo run --bin cache-cli -- seed \
    [--database cache.db] \
    [--waypoints 100] \
    [--beacons 10] \
    [--trajectories 20] \
    [--plans 5] \
    [--tracks 1000] \
    [--tiles 50] \
    [--fetch-weather]
```

Options:
- `--database` - Path to database file (default: `cache.db`)
- `--waypoints` - Number of waypoints to create
- `--beacons` - Number of beacons to create
- `--trajectories` - Number of trajectories to create
- `--plans` - Number of plans to create
- `--tracks` - Total number of GPS tracks to generate
- `--tiles` - Number of real tiles to fetch from OpenFreeMap
- `--fetch-weather` - Fetch real weather data from Open-Meteo

## Testing Spatial Queries

Once you have data, try these spatial queries:

```sql
-- Find waypoints within 10km of a point
SELECT id, name,
       Distance(geom, MakePoint(-122.4194, 37.7749, 4326)) / 1000.0 as distance_km
FROM waypoints
WHERE Distance(geom, MakePoint(-122.4194, 37.7749, 4326)) < 10000
ORDER BY distance_km;

-- Find tracks along a trajectory
SELECT t.id, t.timestamp, t.lat, t.lon
FROM tracks t
JOIN trajectories tr ON 1=1
WHERE Distance(t.geom, tr.geom) < 1000  -- within 1km of trajectory
ORDER BY t.timestamp;

-- Get weather readings near a waypoint
SELECT r.timestamp, s.name, r.value, s.unit
FROM readings r
JOIN sensors s ON r.sensor_id = s.id
WHERE Distance(r.geom, (SELECT geom FROM waypoints WHERE id = ?)) < 5000
ORDER BY r.timestamp;
```

## Development

### Running Tests

```bash
cargo test
```

### Building Release

```bash
cargo build --release
```

### Linting

```bash
cargo clippy
cargo fmt
```

## Architecture

- **SQLite + SpatiaLite** for client-side database
- **Rusqlite** for database access
- **Geo/GeoZero** for geospatial calculations
- **Tokio** for async runtime
- **Reqwest** for HTTP requests (tiles, weather API)
- **Fake** for realistic dummy data generation

## Next Steps

1. Implement Axum/Actix-web REST API server
2. Add PostgreSQL+PostGIS support for server
3. Implement CRDT-based sync protocol
4. Add authentication and authorization
5. Build Flutter frontend
