-- Cache Database Schema
-- SQLite with SpatiaLite extension

-- Enable SpatiaLite
-- Note: This is loaded via code, not in migration

-- Create spatial metadata
SELECT InitSpatialMetadata(1);

-- ============================================================================
-- Waypoints Table
-- ============================================================================
CREATE TABLE IF NOT EXISTS waypoints (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    lat REAL NOT NULL CHECK(lat >= -90 AND lat <= 90),
    lon REAL NOT NULL CHECK(lon >= -180 AND lon <= 180),
    altitude REAL,  -- meters above sea level
    category TEXT,  -- e.g., 'harbor', 'anchorage', 'poi', 'hazard'
    icon TEXT,      -- icon identifier
    color TEXT,     -- hex color for display
    metadata_json TEXT,  -- flexible JSON storage for custom fields
    user_id TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    -- Vector clocks for sync
    version INTEGER NOT NULL DEFAULT 1,
    last_modified_by TEXT
);

-- Add geometry column for waypoints
SELECT AddGeometryColumn('waypoints', 'geom', 4326, 'POINT', 'XY');

-- Create spatial index
SELECT CreateSpatialIndex('waypoints', 'geom');

-- Create regular indexes
CREATE INDEX idx_waypoints_created ON waypoints(created_at);
CREATE INDEX idx_waypoints_user ON waypoints(user_id);
CREATE INDEX idx_waypoints_category ON waypoints(category);


-- ============================================================================
-- Trajectories Table
-- ============================================================================
CREATE TABLE IF NOT EXISTS trajectories (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    waypoint_ids TEXT NOT NULL,  -- JSON array of waypoint IDs (ordered)
    directionality TEXT NOT NULL CHECK(directionality IN ('one_way', 'bidirectional', 'reverse')),
    total_distance_m REAL,  -- total length in meters
    metadata_json TEXT,
    user_id TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    version INTEGER NOT NULL DEFAULT 1,
    last_modified_by TEXT
);

-- Add geometry column for trajectory LineString
SELECT AddGeometryColumn('trajectories', 'geom', 4326, 'LINESTRING', 'XY');

-- Create spatial index
SELECT CreateSpatialIndex('trajectories', 'geom');

-- Create indexes
CREATE INDEX idx_trajectories_user ON trajectories(user_id);
CREATE INDEX idx_trajectories_created ON trajectories(created_at);


-- ============================================================================
-- Plans Table
-- ============================================================================
CREATE TABLE IF NOT EXISTS plans (
    id TEXT PRIMARY KEY NOT NULL,
    trajectory_id TEXT NOT NULL REFERENCES trajectories(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    description TEXT,
    start_time TEXT NOT NULL,  -- ISO 8601 datetime
    estimated_duration_seconds INTEGER,  -- total estimated duration
    waypoint_times TEXT,  -- JSON array of ISO 8601 datetimes for each waypoint
    pause_durations TEXT,  -- JSON array of pause durations in seconds
    status TEXT NOT NULL CHECK(status IN ('draft', 'active', 'completed', 'cancelled')) DEFAULT 'draft',
    beacon_id TEXT,  -- Optional: which beacon is following this plan
    metadata_json TEXT,
    user_id TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    version INTEGER NOT NULL DEFAULT 1,
    last_modified_by TEXT
);

-- Create indexes
CREATE INDEX idx_plans_trajectory ON plans(trajectory_id);
CREATE INDEX idx_plans_status ON plans(status);
CREATE INDEX idx_plans_start_time ON plans(start_time);
CREATE INDEX idx_plans_beacon ON plans(beacon_id);
CREATE INDEX idx_plans_user ON plans(user_id);


-- ============================================================================
-- Beacons Table
-- ============================================================================
CREATE TABLE IF NOT EXISTS beacons (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    type TEXT NOT NULL CHECK(type IN ('vehicle', 'person', 'asset', 'vessel', 'aircraft')),
    icon TEXT,
    color TEXT,
    sensors TEXT,  -- JSON array of sensor IDs attached to this beacon
    active INTEGER NOT NULL DEFAULT 1,  -- boolean: 1 = active, 0 = inactive
    last_seen TEXT,  -- ISO 8601 datetime of last position update
    metadata_json TEXT,
    user_id TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    version INTEGER NOT NULL DEFAULT 1,
    last_modified_by TEXT
);

-- Create indexes
CREATE INDEX idx_beacons_active ON beacons(active);
CREATE INDEX idx_beacons_type ON beacons(type);
CREATE INDEX idx_beacons_user ON beacons(user_id);
CREATE INDEX idx_beacons_last_seen ON beacons(last_seen);


-- ============================================================================
-- Tracks Table (Position History)
-- ============================================================================
CREATE TABLE IF NOT EXISTS tracks (
    id TEXT PRIMARY KEY NOT NULL,
    beacon_id TEXT NOT NULL REFERENCES beacons(id) ON DELETE CASCADE,
    lat REAL NOT NULL CHECK(lat >= -90 AND lat <= 90),
    lon REAL NOT NULL CHECK(lon >= -180 AND lon <= 180),
    altitude REAL,  -- meters
    heading REAL CHECK(heading >= 0 AND heading < 360),  -- degrees, 0 = North
    speed REAL,  -- m/s
    accuracy REAL,  -- meters (GPS accuracy)
    source TEXT NOT NULL CHECK(source IN ('gps', 'adsb', 'manual', 'estimated')),
    timestamp TEXT NOT NULL,  -- ISO 8601 datetime
    metadata_json TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Add geometry column
SELECT AddGeometryColumn('tracks', 'geom', 4326, 'POINT', 'XY');

-- Create spatial index
SELECT CreateSpatialIndex('tracks', 'geom');

-- Create indexes for time-series queries
CREATE INDEX idx_tracks_beacon_time ON tracks(beacon_id, timestamp DESC);
CREATE INDEX idx_tracks_timestamp ON tracks(timestamp);
CREATE INDEX idx_tracks_source ON tracks(source);

-- Partition suggestion: Consider partitioning by date for large datasets
-- CREATE TABLE tracks_2025_09 AS SELECT * FROM tracks WHERE ...


-- ============================================================================
-- Sensors Table
-- ============================================================================
CREATE TABLE IF NOT EXISTS sensors (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    type TEXT NOT NULL CHECK(type IN ('temperature', 'wind_speed', 'wind_direction', 'humidity', 'pressure', 'depth', 'wave_height', 'current_speed', 'custom')),
    unit TEXT NOT NULL,  -- e.g., 'celsius', 'm/s', 'hPa', 'meters'
    fidelity TEXT NOT NULL CHECK(fidelity IN ('forecast', 'measured')),
    origin TEXT NOT NULL CHECK(origin IN ('api', 'hardware', 'manual', 'derived')),
    source_identifier TEXT,  -- API endpoint, hardware serial number, etc.
    metadata_json TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Create indexes
CREATE INDEX idx_sensors_type ON sensors(type);
CREATE INDEX idx_sensors_fidelity ON sensors(fidelity);
CREATE INDEX idx_sensors_origin ON sensors(origin);


-- ============================================================================
-- Readings Table (Sensor Measurements)
-- ============================================================================
CREATE TABLE IF NOT EXISTS readings (
    id TEXT PRIMARY KEY NOT NULL,
    sensor_id TEXT NOT NULL REFERENCES sensors(id) ON DELETE CASCADE,
    lat REAL NOT NULL CHECK(lat >= -90 AND lat <= 90),
    lon REAL NOT NULL CHECK(lon >= -180 AND lon <= 180),
    value REAL NOT NULL,
    timestamp TEXT NOT NULL,  -- ISO 8601: when reading was taken/generated
    forecast_time TEXT,  -- ISO 8601: for forecasts, the predicted time (NULL for measured)
    quality REAL CHECK(quality >= 0 AND quality <= 1),  -- confidence/quality score 0-1
    source_id TEXT,  -- Reference to specific data source (API request ID, hardware reading ID)
    metadata_json TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Add geometry column
SELECT AddGeometryColumn('readings', 'geom', 4326, 'POINT', 'XY');

-- Create spatial index
SELECT CreateSpatialIndex('readings', 'geom');

-- Create indexes for efficient time-series and spatial queries
CREATE INDEX idx_readings_sensor_time ON readings(sensor_id, timestamp DESC);
CREATE INDEX idx_readings_forecast_time ON readings(forecast_time);
CREATE INDEX idx_readings_timestamp ON readings(timestamp);

-- Composite index for common query pattern: get readings for sensor in time range
CREATE INDEX idx_readings_sensor_forecast ON readings(sensor_id, forecast_time);


-- ============================================================================
-- Tiles Cache Table (MBTiles-compatible)
-- ============================================================================
CREATE TABLE IF NOT EXISTS tiles (
    zoom_level INTEGER NOT NULL,
    tile_column INTEGER NOT NULL,
    tile_row INTEGER NOT NULL,
    tile_data BLOB NOT NULL,
    tile_source TEXT NOT NULL,  -- e.g., 'sentinel', 'landsat', 'openfreemap'
    last_accessed TEXT NOT NULL DEFAULT (datetime('now')),
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    PRIMARY KEY (zoom_level, tile_column, tile_row, tile_source)
);

-- Create index for LRU eviction
CREATE INDEX idx_tiles_last_accessed ON tiles(last_accessed);
CREATE INDEX idx_tiles_source ON tiles(tile_source);


-- ============================================================================
-- Tile Metadata Table (MBTiles-compatible)
-- ============================================================================
CREATE TABLE IF NOT EXISTS tile_metadata (
    name TEXT PRIMARY KEY NOT NULL,
    value TEXT NOT NULL
);

-- Insert default metadata
INSERT INTO tile_metadata (name, value) VALUES
    ('name', 'Cache'),
    ('format', 'png'),
    ('bounds', '-180.0,-85.0511,180.0,85.0511'),
    ('center', '0,0,2'),
    ('minzoom', '0'),
    ('maxzoom', '18'),
    ('attribution', 'OpenFreeMap, Sentinel-2, Landsat');


-- ============================================================================
-- Sync Metadata Table (for distributed sync)
-- ============================================================================
CREATE TABLE IF NOT EXISTS sync_metadata (
    device_id TEXT PRIMARY KEY NOT NULL,
    device_name TEXT,
    last_sync TEXT,  -- ISO 8601 datetime
    vector_clock TEXT NOT NULL,  -- JSON object representing vector clock
    sync_filters TEXT,  -- JSON object: what data this device syncs
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX idx_sync_last_sync ON sync_metadata(last_sync);


-- ============================================================================
-- Change Log Table (for incremental sync)
-- ============================================================================
CREATE TABLE IF NOT EXISTS change_log (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    table_name TEXT NOT NULL,
    record_id TEXT NOT NULL,
    operation TEXT NOT NULL CHECK(operation IN ('insert', 'update', 'delete')),
    changed_at TEXT NOT NULL DEFAULT (datetime('now')),
    device_id TEXT NOT NULL,
    synced INTEGER NOT NULL DEFAULT 0  -- boolean: has this been synced?
);

CREATE INDEX idx_changelog_table ON change_log(table_name, record_id);
CREATE INDEX idx_changelog_changed ON change_log(changed_at);
CREATE INDEX idx_changelog_synced ON change_log(synced);


-- ============================================================================
-- Triggers for updated_at timestamps
-- ============================================================================

-- Waypoints
CREATE TRIGGER waypoints_updated_at
AFTER UPDATE ON waypoints
BEGIN
    UPDATE waypoints SET updated_at = datetime('now') WHERE id = NEW.id;
END;

-- Trajectories
CREATE TRIGGER trajectories_updated_at
AFTER UPDATE ON trajectories
BEGIN
    UPDATE trajectories SET updated_at = datetime('now') WHERE id = NEW.id;
END;

-- Plans
CREATE TRIGGER plans_updated_at
AFTER UPDATE ON plans
BEGIN
    UPDATE plans SET updated_at = datetime('now') WHERE id = NEW.id;
END;

-- Beacons
CREATE TRIGGER beacons_updated_at
AFTER UPDATE ON beacons
BEGIN
    UPDATE beacons SET updated_at = datetime('now') WHERE id = NEW.id;
END;

-- Sensors
CREATE TRIGGER sensors_updated_at
AFTER UPDATE ON sensors
BEGIN
    UPDATE sensors SET updated_at = datetime('now') WHERE id = NEW.id;
END;


-- ============================================================================
-- Views for Common Queries
-- ============================================================================

-- Active beacons with their latest position
CREATE VIEW active_beacons_latest AS
SELECT
    b.id,
    b.name,
    b.type,
    b.color,
    b.active,
    b.last_seen,
    t.lat,
    t.lon,
    t.altitude,
    t.heading,
    t.speed,
    t.timestamp as position_timestamp,
    t.source as position_source
FROM beacons b
LEFT JOIN (
    SELECT DISTINCT beacon_id,
           FIRST_VALUE(lat) OVER (PARTITION BY beacon_id ORDER BY timestamp DESC) as lat,
           FIRST_VALUE(lon) OVER (PARTITION BY beacon_id ORDER BY timestamp DESC) as lon,
           FIRST_VALUE(altitude) OVER (PARTITION BY beacon_id ORDER BY timestamp DESC) as altitude,
           FIRST_VALUE(heading) OVER (PARTITION BY beacon_id ORDER BY timestamp DESC) as heading,
           FIRST_VALUE(speed) OVER (PARTITION BY beacon_id ORDER BY timestamp DESC) as speed,
           FIRST_VALUE(timestamp) OVER (PARTITION BY beacon_id ORDER BY timestamp DESC) as timestamp,
           FIRST_VALUE(source) OVER (PARTITION BY beacon_id ORDER BY timestamp DESC) as source
    FROM tracks
) t ON b.id = t.beacon_id
WHERE b.active = 1;


-- Active plans with trajectory information
CREATE VIEW active_plans_detail AS
SELECT
    p.id as plan_id,
    p.name as plan_name,
    p.start_time,
    p.estimated_duration_seconds,
    p.status,
    p.beacon_id,
    t.id as trajectory_id,
    t.name as trajectory_name,
    t.total_distance_m,
    t.waypoint_ids
FROM plans p
JOIN trajectories t ON p.trajectory_id = t.id
WHERE p.status IN ('draft', 'active');
