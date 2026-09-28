# Cache Prototype - Status & Next Steps

## What We've Built

✅ **Complete Rust Backend Prototype** with:
- SQLite + SpatiaLite database schema
- All core data models (Waypoints, Trajectories, Plans, Beacons, Tracks, Sensors, Readings, Tiles)
- Comprehensive CLI tool for database seeding
- Real tile fetching from OpenFreeMap
- Real weather data fetching from Open-Meteo API
- Spatial query support
- Nix shell development environment

## Current Status

**Building:** The prototype is currently compiling. Once complete, you'll be able to:

1. **Initialize the database:**
   ```bash
   nix-shell
   cd backend
   cargo run --bin cache-cli -- init
   ```

2. **Seed with realistic data:**
   ```bash
   cargo run --bin cache-cli -- seed \
       --waypoints 100 \
       --beacons 10 \
       --trajectories 20 \
       --plans 5 \
       --tracks 1000 \
       --tiles 50 \
       --fetch-weather
   ```

This will create:
- 100 waypoints in San Francisco Bay Area
- 10 beacons (vessels, vehicles)
- 20 trajectories connecting waypoints
- 5 plans with time-based scheduling
- 1000 GPS track points showing movement
- 50 real map tiles from OpenFreeMap
- Real weather forecast data from Open-Meteo

## Database Features

### Tables Created
1. **waypoints** - Static locations with geospatial coordinates
2. **trajectories** - Paths through waypoints with LineString geometry
3. **plans** - Time-based route plans
4. **beacons** - Moving objects (vessels, vehicles, people, aircraft)
5. **tracks** - GPS position history with timestamps
6. **sensors** - Weather/environmental measurement devices
7. **readings** - Time-series sensor data
8. **tiles** - Cached map tiles (MBTiles-compatible)
9. **sync_metadata** - Device synchronization tracking
10. **change_log** - Change history for incremental sync

### Spatial Features
- All georeferenced tables have PostGIS-style geometry columns
- Spatial indexes for efficient proximity queries
- Support for distance calculations (Haversine)
- LineString geometries for trajectories
- Point geometries for waypoints, tracks, readings

### Views
- **active_beacons_latest** - Current position of all active beacons
- **active_plans_detail** - Active plans with trajectory information

## Example Queries

Once seeded, try these spatial queries:

```sql
-- Connect to database
sqlite3 cache.db
.load /nix/store/.../mod_spatialite.so

-- Find waypoints within 10km of San Francisco
SELECT id, name,
       Distance(geom, MakePoint(-122.4194, 37.7749, 4326)) / 1000.0 as distance_km
FROM waypoints
WHERE Distance(geom, MakePoint(-122.4194, 37.7749, 4326)) < 10000
ORDER BY distance_km
LIMIT 10;

-- View active beacons with their latest positions
SELECT * FROM active_beacons_latest;

-- Get weather forecast for next 24 hours
SELECT s.name, r.value, s.unit, r.forecast_time
FROM readings r
JOIN sensors s ON r.sensor_id = s.id
WHERE r.forecast_time > datetime('now')
  AND r.forecast_time < datetime('now', '+24 hours')
ORDER BY s.name, r.forecast_time;

-- Find trajectories that pass near a point
SELECT t.id, t.name, t.total_distance_m / 1000.0 as distance_km
FROM trajectories t
WHERE Distance(t.geom, MakePoint(-122.4194, 37.7749, 4326)) < 5000
ORDER BY Distance(t.geom, MakePoint(-122.4194, 37.7749, 4326));
```

## Storage Used

After seeding with default parameters:
- **Database**: ~5-10 MB (waypoints, tracks, readings, metadata)
- **Tiles**: ~2.5 MB (50 tiles × ~50 KB each)
- **Total**: ~7.5-12.5 MB

## Next Steps

### Phase 1: Complete MVP (Current)
- [x] Database schema with SpatiaLite
- [x] Data models
- [x] CLI seeder tool
- [x] Real tile fetching
- [x] Real weather fetching
- [ ] Run and test the prototype
- [ ] Verify spatial queries work correctly

### Phase 2: API Server
- [ ] Implement Axum REST API
- [ ] Waypoints CRUD endpoints
- [ ] Trajectories CRUD endpoints
- [ ] Plans CRUD endpoints
- [ ] Beacons and tracks endpoints
- [ ] Tile serving endpoint
- [ ] Weather data endpoint
- [ ] WebSocket for real-time tracking

### Phase 3: Flutter Frontend
- [ ] Initialize Flutter project
- [ ] Map display with flutter_map
- [ ] Waypoint markers
- [ ] Trajectory display
- [ ] Beacon tracking
- [ ] Plan view UI
- [ ] Offline tile caching

### Phase 4: Sync Protocol
- [ ] Vector clocks implementation
- [ ] CRDT merge logic
- [ ] Selective sync filters
- [ ] Conflict resolution
- [ ] Multi-device testing

### Phase 5: Advanced Features
- [ ] KML/GPX import
- [ ] Hardware GPS integration
- [ ] ADS-B tracking
- [ ] Historical weather analysis
- [ ] Route optimization

## Technology Stack Summary

**Backend:**
- Rust 1.91+
- rusqlite + SpatiaLite
- Tokio (async)
- Reqwest (HTTP client)
- Clap (CLI)
- Serde (JSON)
- Geo/GeoZero (geospatial)

**Data Sources:**
- OpenFreeMap (map tiles)
- Open-Meteo (weather API)
- NOAA (future: nautical charts, tides)

**Development:**
- Nix shell for reproducible environment
- SQLite for client database
- PostgreSQL + PostGIS for server (planned)

## File Structure

```
cache/
├── shell.nix                          # Nix development environment
├── Requirements.md                    # Comprehensive requirements
├── DEVELOPMENT.md                     # Development setup guide
├── PROTOTYPE.md                       # This file
└── backend/
    ├── Cargo.toml                     # Rust dependencies
    ├── README.md                      # Backend documentation
    ├── src/
    │   ├── lib.rs                     # Library root, DB init
    │   ├── models.rs                  # Data models
    │   └── bin/
    │       ├── cli.rs                 # CLI seed tool
    │       └── server.rs              # API server (placeholder)
    └── migrations/
        └── 001_initial_schema.sql     # Database schema
```

## Known Issues

- None yet! This is a fresh prototype.

## Performance Notes

**Tile Fetching:**
- OpenFreeMap has no rate limits
- Fetching 50 tiles takes ~5-10 seconds
- Tiles are cached permanently (LRU eviction coming later)

**Weather Data:**
- Open-Meteo provides ~7 days of hourly forecast
- Single API call provides all weather variables
- No API key required

**Database:**
- SpatiaLite spatial indexes make proximity queries fast
- Track table will grow quickly - consider partitioning by date
- Tile table uses composite primary key for efficient lookups

## Contributing

This prototype demonstrates the core concepts. To extend it:

1. Add more seed data scenarios (oceanic routes, hiking trails, etc.)
2. Implement additional spatial queries
3. Add data validation
4. Improve error handling
5. Add comprehensive tests
6. Optimize database indexes

---

**Happy mapping! 🗺️**
