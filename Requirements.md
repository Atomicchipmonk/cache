# Cache - Distributed Offline-First Geospatial System

## Concept
Cache is a distributed map caching service that runs on local computers and serves tiles and coordinates to applications. It is explicitly designed to:
- Work offline-first with intelligent tile caching
- Replicate and sync state across devices with conflict resolution
- Track historical data for routes, tracks, and sensor readings
- Enable route planning through time with forecasted environmental data

---

## Technology Stack

### Frontend
- **Flutter** (Dart) - Cross-platform UI (Linux + Android)
- **flutter_map** with **MapLibre GL** - Map rendering and tile display
- **SpatiaLite/SQLite** - Local offline database with geospatial support

### Backend/Server
- **Rust** - High-performance API server and sync coordinator
- **Axum** or **Actix-web** - Async web framework
- **PostgreSQL + PostGIS** - Server-side geospatial database
- **MBTiles/PMTiles** - Efficient tile storage format

### Data Synchronization
- **CRDT-based sync** (Conflict-free Replicated Data Types)
- Vector clocks for causality tracking
- Selective sync based on geographic bounds, time ranges, and data types

### Communication
- **REST API** for general operations
- **WebSockets** for real-time beacon tracking
- **Protocol Buffers** for efficient binary serialization

---

## Database Requirements

### DREQ-1: Online Tiling Services
Ability to choose from available free tiling services:

**Map Tiles:**
- **OpenFreeMap** - Free OSM-based vector tiles (no API key, unlimited)
- **VersaTiles** - Self-hostable open alternative (no API key)
- **MapTiler** - Free tier includes limited satellite imagery

**Satellite Imagery:**
- **SentinelMap.eu** - Cloud-free Sentinel-2 tiles (50k tiles/month free, 10m resolution)
- **Landsat on AWS** - STAC API access to Landsat COGs (free, requester-pays bucket)
- **USGS M2M API** - Programmatic Landsat downloads (free, requires registration)
- **Cesium Ion** - Global cloudless basemap (free tier, optimized for streaming)

**Elevation/Terrain:**
- **AWS Elevation Tiles** - Free global terrain tiles (Terrarium format)
- **OpenTopography API** - SRTM, ALOS, Copernicus DEM data (free with registration)

**Nautical/Depth Charts:**
- **NOAA ENCs** - Electronic Navigational Charts (free, updated weekly)
- **NOAA BlueTopo** - High-resolution bathymetry via AWS Open Data

**Tide Data:**
- **NOAA CO-OPS API** - Tide predictions and currents (free, no API key)

### DREQ-2: Fidelity-Based Caching
Ability to choose tile fidelity based on waypoints and routes:
- Fetch highest zoom level (e.g., z18-20) for tiles containing waypoints
- Apply distance-based decay function to reduce zoom level away from waypoints
- Base layer default fidelity (e.g., z8-12) for areas outside interest zones
- Cache budget management with LRU eviction policy

### DREQ-3: Waypoints
Store waypoints with associated metadata:
- **Schema**: `id, name, lat, lon, altitude, created_at, updated_at, metadata_json, user_id`
- **Metadata**: Description, category, icon type, color, custom fields
- **Geospatial indexing**: R-tree index for efficient spatial queries

### DREQ-4: Trajectories
Store trajectories as paths through waypoints:
- **Schema**: `id, name, waypoint_ids[], geometry (LineString), directionality (one_way|bidirectional|reverse), total_distance_m, created_at, metadata_json`
- Support for **spline interpolation** between waypoints
- Automatic distance calculation using geospatial functions
- Support for **multi-segment paths** (e.g., return journeys)

### DREQ-5: Plans
Commit trajectories to time-based plans:
- **Schema**: `id, trajectory_id (FK), start_time, estimated_duration, waypoint_times[], pause_durations[], status (draft|active|completed|cancelled), created_at`
- Automatic **time propagation** based on speed estimates
- **Pause/stop tracking** at waypoints (fuel, rest, etc.)
- **ETA calculations** with real-time updates based on beacon progress

### DREQ-6: Beacons
Vehicles/users with unique identities:
- **Schema**: `id, name, type (vehicle|person|asset), icon, color, sensors[], active, last_seen, metadata_json`
- Support for **multiple beacons** per user
- **Sensor associations**: Link dedicated hardware sensors to specific beacons

### DREQ-7: Tracks
Position history for beacons:
- **Schema**: `id, beacon_id (FK), lat, lon, altitude, heading, speed, accuracy, timestamp, source (gps|adsb|manual)`
- **Time-series optimization**: Partitioned by date for efficient queries
- **Track simplification**: Douglas-Peucker algorithm to reduce storage while preserving shape
- **Retention policy**: Configurable (e.g., keep full resolution for 30 days, simplified for 1 year)

### DREQ-8: Sensors
Definition of sensor types and sources:
- **Schema**: `id, name, type (temperature|wind_speed|humidity|pressure|depth|custom), unit, fidelity (forecast|measured), origin (api|hardware|manual), metadata_json`
- **Supported types**: Temperature (°C/°F), Wind Speed (m/s, knots), Humidity (%), Pressure (hPa), Depth (m), custom
- **Fidelity**: Forecast (future predictions), Measured (actual samples)
- **Origin**: API (weather services), Hardware (local sensors), Manual (user input)

### DREQ-9: Readings
Sensor measurements at time and location:
- **Schema**: `id, sensor_id (FK), lat, lon, value, timestamp, forecast_time, quality, source_id`
- **Reading types**:
  - **Forecast**: `timestamp` = when reading was generated, `forecast_time` = predicted time
  - **Measured**: `timestamp` = `forecast_time` (actual measurement time)
- **Spatial interpolation**: Support for querying interpolated values between readings
- **Time-series storage**: Optimize for range queries by time and location

### DREQ-10: Data Synchronization
Flexible sync between instances (server ↔ app, server ↔ server):

**Sync Modes:**
- **Wholesale sync**: All data of specific types (e.g., all waypoints)
- **Time-range sync**: Data within date/time bounds
- **Trajectory-proximity sync**: Data within X km of a trajectory
- **Plan-based sync**: All data relevant to active plans (trajectory + time window + proximity)
- **Beacon-following sync**: Track data and readings near specific beacons

**Conflict Resolution:**
- **Last-write-wins** with vector clocks for simple fields
- **CRDT merge** for collaborative editing scenarios
- **Tombstones** for deletion tracking across devices

### DREQ-11: Performance & Storage

**Performance Targets:**
- **Startup time**: <3 seconds cold start with cached tiles
- **Tile rendering**: 60 FPS pan/zoom on target hardware
- **Database optimization**: Spatial indexes, prepared statements, connection pooling

**Storage Capacity:**
- **Client cache**: 50 GB (configurable: 10-100 GB)
- **Server storage**: 4 TB (configurable: 1-10 TB)

**Client Storage Budget (50 GB):**
```
Recommended allocation for comprehensive offline capability:

30 GB - Satellite imagery tiles
        • 300,000 Sentinel-2 tiles (~100 KB each)
        • Coverage: 800-1000 km × 800-1000 km at z14 detail
        • Global context at z8-z12
        • Example: Entire US West Coast or Mediterranean Sea

10 GB - Historical Landsat scenes
        • 66 full scenes (150 MB each)
        • Key locations with temporal depth (5-10 years)
        • Seasonal comparisons, weather event archives
        • Example: 6-10 regions with 10-year history each

5 GB  - Nautical charts & bathymetry
        • NOAA ENCs (vector charts)
        • BlueTopo depth data for region
        • Harbor detail charts
        • Example: All charts for US West Coast + Hawaii

3 GB  - Weather data cache
        • 30-day historical data
        • 14-day forecast (multiple models)
        • Hourly resolution for active plans
        • Example: Pacific region complete weather coverage

2 GB  - Application data & database
        • Tracks (10,000+ GPS tracks)
        • Waypoints (100,000+)
        • Plans, beacons, sensor readings
        • Sync metadata, vector clocks
```

**Server Storage Budget (4 TB):**
```
Global multi-user service allocation:

2 TB   - Global Sentinel-2 tile cache
         • 20,000,000 tiles
         • Near-complete global coverage at z12-z14
         • z16 detail for major cities/ports (500+)
         • Covers: All oceans, coastlines, major cities

1 TB   - Landsat scene archive
         • 6,600 full scenes
         • Complete coverage of 200+ key regions
         • 5-10 year temporal depth for popular areas
         • On-demand processing for custom dates

500 GB - Nautical charts (global)
         • All NOAA ENCs
         • Global bathymetry (GEBCO, BlueTopo)
         • 1000+ harbor detail charts
         • Worldwide shipping lane coverage

300 GB - Weather data archive
         • 5 years of historical weather
         • Global current conditions + 14-day forecasts
         • Multiple models (GFS, ECMWF, regional)
         • Archived major weather events

200 GB - Elevation & terrain data
         • Global DEM (30-90m resolution)
         • High-res (10m) for populated areas
         • Terrain tiles for 3D visualization
         • Bathymetry integration
```

**Coverage Scale Reference:**
- **1 Landsat scene**: 185 km × 180 km (33,300 km²)
- **30 web tiles at z12**: ~200 km × 200 km (40,000 km²)
- **500 tiles at z14**: ~250 km × 50 km corridor (typical route with margins)
- **300,000 tiles at z14**: ~800 km × 800 km region (640,000 km²)
- **20M tiles at z12-z14**: Near-complete global coverage

**Real-World Coverage Examples:**

*Client (50 GB):*
- Entire US West Coast + major sailing routes
- Mediterranean Sea + all surrounding coastlines
- Great Lakes region (complete detail)
- Trans-Pacific route: SF → Hawaii → Japan

*Server (4 TB):*
- All major oceans with detailed coastlines
- Top 1000 ports/harbors worldwide
- Complete historical data for popular routes
- Antarctic/Arctic passage planning
- Global emergency/rescue capability

### DREQ-12: Export & Backup (New)
- **Export formats**: KML, GPX, GeoJSON, CSV
- **Backup**: SQLite database file export with compression
- **Data portability**: Full export/import between instances

---

## Input Requirements

### IREQ-1: Beacon Position Inputs
Support multiple GPS/tracking sources:
- **Dedicated GPS**: NMEA 0183/2000 sentence parsing via serial/USB
- **ADS-B Transponders**: Integration with dump1090 or similar receivers (Mode S, ADS-B)
- **App-based GPS**: Device GPS via platform APIs (Android LocationManager, Linux gpsd)
- **Manual input**: Tap-to-place or coordinate entry

### IREQ-2: Weather Data
**Primary: Open-Meteo API** (https://open-meteo.com/)
- **No API key required**, unlimited free access
- **Data**: Temperature, humidity, wind speed/direction, pressure, precipitation
- **Coverage**: Global, multiple weather models (NOAA GFS, ECMWF, DWD)
- **Polling rates**:
  - **Forecast data**: Every 30 minutes (forecasts update 4x daily)
  - **Current conditions**: Every 5-10 minutes for active plans
  - **Historical data**: On-demand via archive API
- **Endpoints**:
  - Current: `/v1/forecast?current_weather=true`
  - Forecast: `/v1/forecast?hourly=temperature_2m,windspeed_10m,pressure_msl`

**Backup: OpenWeatherMap API**
- Free tier: 60 calls/min, 1,000,000 calls/month
- Use if Open-Meteo has outages

### IREQ-3: Dedicated Hardware Sensors
Support for local sensor hardware:
- **Connection types**: USB serial, Bluetooth LE, I2C/SPI (for embedded scenarios)
- **Sensor types**: Temperature, humidity, barometric pressure, wind speed/direction, water depth
- **Protocols**: NMEA sentences, JSON over serial, custom binary formats
- **Auto-discovery**: Scan for BLE sensors advertising standard UUIDs

### IREQ-4: Nautical Data Integration (New)
- **Tide predictions**: NOAA CO-OPS API (free, no key)
- **Current predictions**: NOAA currents stations
- **Polling**: Daily fetch of 7-day forecasts for areas of interest

---

## Device Requirements

### DREQ-13: Client Device Specifications

**Minimum Requirements (Basic Functionality):**
- **Storage**: 10 GB available
  - Supports: Single region coverage (~200 km × 200 km)
  - Use case: Day trips, local area navigation
  - Tile capacity: ~60,000 tiles at z12-z14

- **RAM**: 4 GB
- **CPU**: Dual-core 1.5 GHz or equivalent
- **Display**: 1280×720 minimum resolution
- **GPS**: Built-in or USB GPS receiver (for beacon tracking)
- **Network**: WiFi for initial sync (offline operation thereafter)

**Recommended Requirements (Full Featured):**
- **Storage**: 50 GB available
  - Supports: Regional/expedition coverage (800-1000 km range)
  - Use case: Multi-day voyages, offline expeditions
  - Full feature set: Satellite imagery + historical data + charts

- **RAM**: 8 GB
- **CPU**: Quad-core 2.0 GHz or equivalent
- **Display**: 1920×1080 or higher
- **GPS**: Multi-constellation GNSS (GPS + GLONASS + Galileo)
- **Network**: WiFi + cellular (for opportunistic sync)

**Optimal Requirements (Power User):**
- **Storage**: 100+ GB available
  - Supports: Multi-region or global planning
  - Use case: Professional maritime, aviation, or expedition use
  - Extensive offline capability, full historical archives

- **RAM**: 16 GB
- **CPU**: Hexa-core 2.5 GHz or equivalent
- **Display**: 2560×1440 or higher, multi-monitor support
- **GPS**: RTK-capable or dedicated GPS with sub-meter accuracy
- **Network**: Bonded cellular + satellite for global connectivity

**Device Type Specific:**

*Android Mobile (Phone):*
- Minimum: 64 GB internal storage (10 GB available for Cache)
- Recommended: 128 GB internal storage (50 GB available)
- Optimal: 256+ GB with SD card expansion

*Android Tablet:*
- Minimum: 128 GB internal storage (10 GB available)
- Recommended: 256 GB internal storage (50 GB available)
- Optimal: 512+ GB internal storage

*Linux Desktop/Laptop:*
- Minimum: 50 GB available SSD storage
- Recommended: 100 GB available SSD storage
- Optimal: 500+ GB dedicated SSD or NVMe drive

*Embedded (Raspberry Pi, marine computers):*
- Minimum: 64 GB microSD card
- Recommended: 128+ GB microSD or SSD via USB 3.0
- Optimal: 500 GB+ SSD with USB 3.0+ connection

### DREQ-14: Server Specifications

**Minimum Server (Small Group/Family):**
- **Storage**: 500 GB
  - Supports: 5-10 clients, regional coverage
  - Coverage: Single ocean region or continental area
  - Tile cache: ~2.5M tiles

- **RAM**: 8 GB
- **CPU**: 4 cores
- **Network**: 100 Mbps uplink
- **Clients**: Up to 10 simultaneous

**Recommended Server (Community/Organization):**
- **Storage**: 4 TB
  - Supports: 50-100 clients, global coverage
  - Coverage: All major oceans and coastlines
  - Tile cache: 20M+ tiles, comprehensive historical data

- **RAM**: 32 GB
- **CPU**: 8+ cores
- **Network**: 1 Gbps uplink
- **Clients**: Up to 100 simultaneous
- **Database**: PostgreSQL with PostGIS on dedicated storage

**Optimal Server (Commercial Service):**
- **Storage**: 10+ TB (RAID 10 for redundancy)
  - Supports: 1000+ clients, complete global coverage
  - Coverage: Worldwide at high resolution
  - Tile cache: 50M+ tiles, 10+ year historical archive

- **RAM**: 64+ GB
- **CPU**: 16+ cores (or multi-server cluster)
- **Network**: 10 Gbps uplink, CDN integration
- **Clients**: 1000+ simultaneous
- **Database**: PostgreSQL cluster with replication
- **Redundancy**: Multi-region deployment, automated backups

**Server Storage Allocation (4 TB Recommended):**
```
2 TB   - Tile cache (read-heavy, SSD recommended)
1 TB   - Scene archive (Landsat, can use HDD)
500 GB - Nautical charts and vector data (SSD)
300 GB - Weather data (SSD, time-series optimized)
200 GB - System, database, logs (SSD, high IOPS)
```

**Bandwidth Estimates:**

*Initial Client Sync (50 GB cache):*
- Full sync: 50 GB download over 1-48 hours depending on connection
- Typical sync: 5-10 GB for new region (30-60 minutes on 100 Mbps)
- Incremental sync: 100-500 MB daily for active plans

*Server Bandwidth (100 clients):*
- Initial onboarding: ~500 GB/month
- Active usage: ~2-5 TB/month (incremental syncs, weather updates)
- Peak: ~10 TB/month (multiple expeditions, high activity)

---

## Frontend Requirements

### FREQ-1: Platform Support
- **Linux**: Native desktop app (GTK/Qt integration via Flutter)
  - Target: Desktop/laptop computers, embedded Linux systems
  - Storage: 50-100 GB available (recommended)

- **Android**: Native app (min SDK 24, Android 7.0+)
  - Target: Phones, tablets, marine Android devices
  - Storage: 10-50 GB available (64-256 GB device storage)
  - Note: Manages cache based on available storage

- **Single codebase**: 95%+ code sharing between platforms

### FREQ-2: Map Functionality
- **Zoom**: Smooth zoom from z0 (world) to z20 (1m resolution)
- **Pan/Drag**: 60 FPS panning with gesture support
- **Rotate**: Bearing/compass rotation (optional, disable for simplicity)
- **Offline tiles**: Seamless display of cached tiles with "no data" indicators

### FREQ-3: Trajectory Input
- **KML/KMZ import**: Parse Point, LineString, Polygon, MultiGeometry
- **Manual drawing**: Tap-to-add waypoints, drag to adjust
- **GPX import**: Support for routes and tracks
- **Route snapping**: Optional snap-to-road using local tile data

### FREQ-4: Plan Creation
- **Trajectory selection**: Choose from saved trajectories
- **Time configuration**: Set start time, estimated speed, or arrival time
- **Pause insertion**: Add stops with durations at waypoints
- **ETA preview**: Show estimated times at each waypoint before committing

### FREQ-5: Live Beacon Tracking
- **Real-time updates**: WebSocket connection for <1s latency
- **Track trails**: Configurable trail length (last N points or time window)
- **Beacon clustering**: Group nearby beacons at low zoom levels
- **Status indicators**: Active, stale (>5min), offline

### FREQ-6: Global View
- **Unified timeline**: All data shown at same global time
- **Time slider**: Scrub through historical data (last 24h/7d/30d/all)
- **Playback controls**: Play/pause animation of historical tracks
- **Layer toggles**: Show/hide tracks, waypoints, sensor readings, tiles

### FREQ-7: Plan View
- **Trajectory framing**: Auto-zoom to show full route with padding
- **Time-relative display**: Show forecasted sensor data aligned with ETA at each location
- **Example**: If beacon expected at waypoint at 14:00 tomorrow, show temperature forecast for that location at 14:00
- **Now line**: Visual indicator of current time vs plan timeline
- **Progress tracking**: Show beacon position relative to plan with ahead/behind indicators

### FREQ-8: Settings & Configuration (New)
- **Tile sources**: Enable/disable tile layers, set default zoom levels
- **Cache management**: View storage usage, clear old tiles, set size limits
- **Sync configuration**: Choose sync server, set sync frequency, selective sync filters
- **Units**: Metric/imperial, coordinate formats (DD, DDM, DMS)
- **Appearance**: Light/dark mode, color schemes

---

## Architecture Requirements (New)

### AREQ-1: Offline-First Design
- All core functionality works without network
- Background sync when connection available
- Queue outgoing changes during offline periods
- Graceful degradation when tile data unavailable

### AREQ-2: Multi-Device Sync
- **Device registration**: Each device gets unique ID
- **Sync protocol**: Bidirectional with incremental updates
- **Bandwidth optimization**: Only sync changes since last sync (delta sync)
- **Conflict detection**: Vector clocks to detect concurrent edits

### AREQ-3: Security (New)
- **Authentication**: API key or username/password for server sync
- **Encryption**: TLS for all network traffic
- **Local data**: Optional encryption at rest for sensitive data
- **Access control**: Per-beacon and per-plan permissions

### AREQ-4: Extensibility
- **Plugin architecture**: Support for custom tile sources
- **Custom sensors**: Define new sensor types via config
- **Scripting**: Optional Lua/JavaScript for custom data transformations

---

## Testing Requirements (New)

### TREQ-1: Unit Testing
- **Coverage target**: >80% for core business logic
- **Frameworks**: Rust cargo test (backend), Flutter test (frontend)
- **Geospatial**: Test calculations with known datasets

### TREQ-2: Integration Testing
- **Sync scenarios**: Multi-device conflict resolution
- **Offline/online transitions**: Verify queue and replay
- **API contract tests**: Ensure backend/frontend compatibility

### TREQ-3: Performance Testing
- **Load testing**: 1000 tracks, 10000 waypoints, 100000 sensor readings
- **Tile rendering**: Verify 60 FPS with 500+ tiles cached
- **Sync performance**: Measure time to sync 1GB dataset

### TREQ-4: Field Testing
- **Real GPS data**: Test with actual GPS receivers and tracks
- **Poor connectivity**: Test sync over intermittent/slow connections
- **Battery usage**: Monitor power consumption on mobile devices

---

## Development Phases (Recommendation)

### Phase 1: Core Infrastructure (MVP)
- SQLite/SpatiaLite database schema
- Basic tile fetching and caching (OpenFreeMap)
- Simple map display (pan/zoom)
- Waypoint CRUD operations

### Phase 2: Tracking & Plans
- Trajectory creation and editing
- Plan creation with time propagation
- Beacon tracking (manual input)
- Track history storage

### Phase 3: Real-Time & Sensors
- WebSocket real-time updates
- Weather API integration (Open-Meteo)
- Sensor reading storage
- Plan view with time-aligned data

### Phase 4: Sync & Distribution
- Server implementation
- Sync protocol and conflict resolution
- Multi-device testing
- Selective sync filters

### Phase 5: Advanced Features
- KML/GPX import
- Hardware sensor integration
- ADS-B tracking
- Nautical charts and tides

---

## References

**Map Tiles:**
- [OpenFreeMap](https://github.com/hyperknot/openfreemap)
- [VersaTiles](https://versatiles.org/)
- [MapTiler](https://www.maptiler.com/openstreetmap/)

**Terrain & Elevation:**
- [AWS Elevation Tiles](https://geodataviewer.com/datasets/dem/aws-elevation-tiles/)
- [OpenTopography](https://opentopography.org/developers)

**Nautical Charts:**
- [NOAA ENCs](https://nauticalcharts.noaa.gov/charts/noaa-enc.html)
- [NOAA BlueTopo](https://nauticalcharts.noaa.gov/data/bluetopo.html)

**Weather & Environment:**
- [Open-Meteo](https://open-meteo.com/)
- [NOAA CO-OPS Tides API](https://tidesandcurrents.noaa.gov/api/)

**Satellite Imagery:**
- [SentinelMap.eu Basemap](https://www.sentinelmap.eu/)
- [Landsat on AWS](https://registry.opendata.aws/usgs-landsat/)
- [USGS M2M API](https://www.usgs.gov/media/files/m2m-application-token-documentation)
- [Sentinel Hub API](https://www.sentinel-hub.com/develop/api/)
- [Cesium Ion](https://cesium.com/platform/cesium-ion/content/sentinel-2-imagery/)

**Geospatial Tools:**
- [SpatiaLite](https://www.gaia-gis.it/fossil/libspatialite/)
- [PostGIS](https://postgis.net/)
- [MapLibre GL](https://maplibre.org/)