-- Seattle to San Francisco Coastal Route
-- A realistic maritime voyage down the Pacific Coast
-- Route distance: ~700 nautical miles / ~1300 km
-- Typical sailing time: 5-7 days depending on conditions

-- ============================================================================
-- Waypoints (in order from Seattle to San Francisco)
-- ============================================================================

INSERT INTO waypoints (id, name, description, lat, lon, altitude, category, icon, color, metadata_json, user_id, created_at, updated_at, version, geom)
VALUES
-- 1. Port of Seattle (Start)
('wp-sea-001', 'Port of Seattle', 'Departure point - Elliott Bay, Seattle WA', 47.5881, -122.3589, 0, 'harbor', 'anchor', '#2E86DE', '{"type":"departure","port":"Seattle","state":"WA"}', 'system', datetime('now'), datetime('now'), 1, MakePoint(-122.3589, 47.5881, 4326)),

-- 2. Port Angeles (refuel/safety stop)
('wp-sea-002', 'Port Angeles', 'Fuel and provisions - Gateway to Strait of Juan de Fuca', 48.1182, -123.4307, 0, 'harbor', 'fuel', '#27AE60', '{"type":"fuel_stop","port":"Port Angeles","state":"WA"}', 'system', datetime('now'), datetime('now'), 1, MakePoint(-123.4307, 48.1182, 4326)),

-- 3. Neah Bay (last WA stop before open ocean)
('wp-sea-003', 'Neah Bay', 'Last stop in Washington - entrance to Pacific Ocean', 48.3656, -124.6222, 0, 'harbor', 'lighthouse', '#F39C12', '{"type":"waypoint","significance":"last_protected_harbor","state":"WA"}', 'system', datetime('now'), datetime('now'), 1, MakePoint(-124.6222, 48.3656, 4326)),

-- 4. Astoria, OR (Columbia River)
('wp-sea-004', 'Astoria/Columbia River Bar', 'Major port - fuel, provisions - dangerous bar crossing', 46.1883, -123.8100, 0, 'harbor', 'anchor', '#E74C3C', '{"type":"major_port","hazard":"columbia_bar","state":"OR"}', 'system', datetime('now'), datetime('now'), 1, MakePoint(-123.8100, 46.1883, 4326)),

-- 5. Newport, OR
('wp-sea-005', 'Newport (Yaquina Bay)', 'Protected harbor - fuel and provisions', 44.6383, -124.0514, 0, 'harbor', 'anchor', '#27AE60', '{"type":"fuel_stop","port":"Newport","state":"OR"}', 'system', datetime('now'), datetime('now'), 1, MakePoint(-124.0514, 44.6383, 4326)),

-- 6. Coos Bay, OR
('wp-sea-006', 'Coos Bay', 'Deepest coastal harbor between SF and Puget Sound', 43.4293, -124.2294, 0, 'harbor', 'anchor', '#3498DB', '{"type":"major_port","significance":"deepest_harbor","state":"OR"}', 'system', datetime('now'), datetime('now'), 1, MakePoint(-124.2294, 43.4293, 4326)),

-- 7. Crescent City, CA
('wp-sea-007', 'Crescent City Harbor', 'First California port - fuel and rest', 41.7561, -124.2017, 0, 'harbor', 'anchor', '#9B59B6', '{"type":"fuel_stop","port":"Crescent City","state":"CA"}', 'system', datetime('now'), datetime('now'), 1, MakePoint(-124.2017, 41.7561, 4326)),

-- 8. Humboldt Bay (Eureka), CA
('wp-sea-008', 'Humboldt Bay (Eureka)', 'Major port - fuel, provisions, and weather check', 40.7356, -124.2303, 0, 'harbor', 'anchor', '#1ABC9C', '{"type":"major_port","port":"Eureka","state":"CA"}', 'system', datetime('now'), datetime('now'), 1, MakePoint(-124.2303, 40.7356, 4326)),

-- 9. Fort Bragg (Noyo Harbor), CA
('wp-sea-009', 'Fort Bragg (Noyo Harbor)', 'Fishing harbor - optional fuel stop', 39.4458, -123.8044, 0, 'harbor', 'anchor', '#F39C12', '{"type":"waypoint","port":"Fort Bragg","state":"CA"}', 'system', datetime('now'), datetime('now'), 1, MakePoint(-123.8044, 39.4458, 4326)),

-- 10. Bodega Bay, CA
('wp-sea-010', 'Bodega Bay', 'Last stop before San Francisco Bay', 38.3244, -123.0386, 0, 'harbor', 'anchor', '#E67E22', '{"type":"fuel_stop","significance":"last_stop_before_sf","state":"CA"}', 'system', datetime('now'), datetime('now'), 1, MakePoint(-123.0386, 38.3244, 4326)),

-- 11. San Francisco Bay (Golden Gate)
('wp-sea-011', 'Golden Gate Bridge', 'San Francisco Bay entrance - iconic landmark', 37.8199, -122.4783, 0, 'poi', 'lighthouse', '#C0392B', '{"type":"landmark","significance":"golden_gate","state":"CA"}', 'system', datetime('now'), datetime('now'), 1, MakePoint(-122.4783, 37.8199, 4326)),

-- 12. San Francisco (Destination)
('wp-sea-012', 'San Francisco Marina', 'Destination - San Francisco Bay', 37.8069, -122.3961, 0, 'harbor', 'anchor', '#16A085', '{"type":"arrival","port":"San Francisco","state":"CA"}', 'system', datetime('now'), datetime('now'), 1, MakePoint(-122.3961, 37.8069, 4326));


-- ============================================================================
-- Trajectory: Seattle to San Francisco Coastal Route
-- ============================================================================

INSERT INTO trajectories (id, name, description, waypoint_ids, directionality, total_distance_m, metadata_json, user_id, created_at, updated_at, version, geom)
VALUES (
    'traj-sea-sf-001',
    'Seattle to San Francisco - Pacific Coast',
    'Classic Pacific Northwest to California coastal sailing route. ~700nm voyage through diverse conditions including protected waters, open ocean, and challenging bars. Typical duration: 5-7 days. Best weather window: late summer (Aug-Sept). Note: Columbia River Bar crossing requires careful planning.',
    '["wp-sea-001","wp-sea-002","wp-sea-003","wp-sea-004","wp-sea-005","wp-sea-006","wp-sea-007","wp-sea-008","wp-sea-009","wp-sea-010","wp-sea-011","wp-sea-012"]',
    'one_way',
    1296000,  -- Approximately 700nm = 1296 km
    '{"route_type":"coastal","difficulty":"moderate_to_challenging","season":"summer","hazards":["columbia_bar","cape_blanco","point_conception"],"typical_duration_days":6,"fuel_stops":["Port Angeles","Astoria","Newport","Coos Bay","Crescent City","Eureka","Bodega Bay"]}',
    'system',
    datetime('now'),
    datetime('now'),
    1,
    GeomFromText('LINESTRING(-122.3589 47.5881, -123.4307 48.1182, -124.6222 48.3656, -123.8100 46.1883, -124.0514 44.6383, -124.2294 43.4293, -124.2017 41.7561, -124.2303 40.7356, -123.8044 39.4458, -123.0386 38.3244, -122.4783 37.8199, -122.3961 37.8069)', 4326)
);


-- ============================================================================
-- Beacon: MV Pacific Explorer (fictional vessel)
-- ============================================================================

INSERT INTO beacons (id, name, type, icon, color, sensors, active, last_seen, metadata_json, user_id, created_at, updated_at, version)
VALUES (
    'beacon-pacific-001',
    'MV Pacific Explorer',
    'vessel',
    'sailboat',
    '#3498DB',
    '["sensor-weather-001"]',
    1,
    NULL,
    '{"vessel_type":"sailboat","length_ft":42,"beam_ft":13,"draft_ft":6,"home_port":"Seattle","owner":"Demo User"}',
    'system',
    datetime('now'),
    datetime('now'),
    1
);


-- ============================================================================
-- Plan: Seattle to SF Departure 9/28/2026 8:00 AM PDT
-- ============================================================================

-- Note: 9/28/2026 8:00 AM PDT (Pacific Daylight Time, UTC-7)
--       = 2026-09-28T15:00:00Z in UTC

-- Estimated waypoint times (assuming 5-6 knots average speed, with stops)
-- Total distance: ~700nm
-- Sailing hours per day: 12-16 hours (daylight sailing preferred)
-- Daily distance: 70-90nm
-- Total trip: 6 days

INSERT INTO plans (id, trajectory_id, name, description, start_time, estimated_duration_seconds, waypoint_times, pause_durations, status, beacon_id, metadata_json, user_id, created_at, updated_at, version)
VALUES (
    'plan-sea-sf-001',
    'traj-sea-sf-001',
    'Fall 2026 Coastal Cruise: Seattle → San Francisco',
    'Six-day coastal passage departing Seattle on Sunday, September 28, 2026 at 8:00 AM Pacific Time. Weather window looks favorable with light NW winds transitioning to calm. This route follows the traditional Pacific Coast cruising route with planned stops at major ports for fuel, provisions, and rest.',
    '2026-09-28T15:00:00Z',  -- Start: 9/28/2026 8:00 AM PDT
    518400,  -- 6 days in seconds (144 hours)
    '["2026-09-28T15:00:00Z","2026-09-28T22:00:00Z","2026-09-29T06:00:00Z","2026-09-29T23:00:00Z","2026-09-30T16:00:00Z","2026-10-01T09:00:00Z","2026-10-02T02:00:00Z","2026-10-02T19:00:00Z","2026-10-03T09:00:00Z","2026-10-03T23:00:00Z","2026-10-04T10:00:00Z","2026-10-04T16:00:00Z"]',
    '[0,14400,28800,14400,14400,14400,14400,14400,7200,14400,0,0]',  -- Pause durations in seconds (various stops: 4-8 hours)
    'draft',
    'beacon-pacific-001',
    '{"weather_window":"favorable","crew_size":2,"provisioning_complete":false,"chart_version":"2026","notes":"Check Columbia River Bar conditions before crossing. Monitor Cape Blanco and Point Arena conditions."}',
    'system',
    datetime('now'),
    datetime('now'),
    1
);


-- ============================================================================
-- Sample Track Points (showing departure from Seattle)
-- ============================================================================

INSERT INTO tracks (id, beacon_id, lat, lon, altitude, heading, speed, accuracy, source, timestamp, metadata_json, created_at, geom)
VALUES
-- Departure from Seattle (Day 1: 9/28/2026 8:00 AM PDT = 15:00 UTC)
('track-001', 'beacon-pacific-001', 47.5881, -122.3589, 0, 270, 0.0, 5.0, 'gps', '2026-09-28T15:00:00Z', '{"status":"departure","location":"Port of Seattle"}', datetime('now'), MakePoint(-122.3589, 47.5881, 4326)),

-- 30 minutes later - heading west through Puget Sound
('track-002', 'beacon-pacific-001', 47.6123, -122.4201, 0, 285, 6.2, 5.0, 'gps', '2026-09-28T15:30:00Z', '{"status":"underway","conditions":"calm"}', datetime('now'), MakePoint(-122.4201, 47.6123, 4326)),

-- 1 hour - passing Point No Point
('track-003', 'beacon-pacific-001', 47.7234, -122.5267, 0, 290, 6.5, 5.0, 'gps', '2026-09-28T16:00:00Z', '{"status":"underway","location":"near Point No Point"}', datetime('now'), MakePoint(-122.5267, 47.7234, 4326)),

-- 2 hours - entering Strait of Juan de Fuca
('track-004', 'beacon-pacific-001', 47.9823, -122.7845, 0, 275, 6.8, 5.0, 'gps', '2026-09-28T17:00:00Z', '{"status":"underway","location":"Strait of Juan de Fuca"}', datetime('now'), MakePoint(-122.7845, 47.9823, 4326));


-- ============================================================================
-- View: Plan Details with Local Time
-- ============================================================================

-- Create a view that shows plan times in Pacific Time
CREATE VIEW IF NOT EXISTS plan_pacific_time AS
SELECT
    p.id,
    p.name,
    p.start_time as start_time_utc,
    datetime(p.start_time, '-7 hours') as start_time_pdt,  -- Note: Simplified, doesn't handle PST/PDT boundary
    p.status,
    p.beacon_id,
    t.name as trajectory_name,
    t.total_distance_m / 1852.0 as distance_nautical_miles,
    p.estimated_duration_seconds / 3600.0 as duration_hours
FROM plans p
JOIN trajectories t ON p.trajectory_id = t.id;


-- ============================================================================
-- Verification Queries
-- ============================================================================

-- Check waypoints
SELECT COUNT(*) as waypoint_count FROM waypoints WHERE id LIKE 'wp-sea-%';

-- Check trajectory
SELECT name, total_distance_m / 1852.0 as nautical_miles FROM trajectories WHERE id = 'traj-sea-sf-001';

-- Check plan
SELECT * FROM plan_pacific_time WHERE id = 'plan-sea-sf-001';

-- Check beacon
SELECT name, type, active FROM beacons WHERE id = 'beacon-pacific-001';

-- Check initial tracks
SELECT COUNT(*) as track_count FROM tracks WHERE beacon_id = 'beacon-pacific-001';
