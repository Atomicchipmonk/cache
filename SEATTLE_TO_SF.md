# Seattle to San Francisco Coastal Route

## 🎉 Prototype Ready!

Your Cache database prototype is built and initialized! The release build completed successfully and the database is ready.

## The Route

**Seattle → San Francisco Pacific Coast**
- **Distance**: ~700 nautical miles (1,296 km)
- **Typical Duration**: 5-7 days
- **Waypoints**: 12 major ports and landmarks
- **Best Season**: Late summer (August-September)

### Waypoints (North to South)

1. **Port of Seattle** (47.5881°N, 122.3589°W) - Departure
2. **Port Angeles** (48.1182°N, 123.4307°W) - Fuel/provisions
3. **Neah Bay** (48.3656°N, 124.6222°W) - Last WA stop before Pacific
4. **Astoria/Columbia River** (46.1883°N, 123.8100°W) - Major port, dangerous bar
5. **Newport (Yaquina Bay)** (44.6383°N, 124.0514°W) - Protected harbor
6. **Coos Bay** (43.4293°N, 124.2294°W) - Deepest coastal harbor
7. **Crescent City** (41.7561°N, 124.2017°W) - First CA port
8. **Humboldt Bay (Eureka)** (40.7356°N, 124.2303°W) - Major port
9. **Fort Bragg (Noyo Harbor)** (39.4458°N, 123.8044°W) - Fishing harbor
10. **Bodega Bay** (38.3244°N, 123.0386°W) - Last stop before SF
11. **Golden Gate Bridge** (37.8199°N, 122.4783°W) - Iconic landmark
12. **San Francisco Marina** (37.8069°N, 122.3961°W) - Arrival

## The Plan

**Departure**: Sunday, September 28, 2026 at 8:00 AM PDT
**Vessel**: MV Pacific Explorer (42ft sailboat)
**Status**: Draft (planning phase)
**Estimated Arrival**: Saturday, October 4, 2026

**Timeline** (all times in PDT/PST):
- **Day 1 (9/28)**: Seattle → Port Angeles (depart 8:00 AM)
- **Day 2 (9/29)**: Port Angeles → Astoria (cross Columbia Bar)
- **Day 3 (9/30)**: Astoria → Newport
- **Day 4 (10/1)**: Newport → Coos Bay
- **Day 5 (10/2)**: Coos Bay → Eureka
- **Day 6 (10/3)**: Eureka → Bodega Bay
- **Day 7 (10/4)**: Bodega Bay → San Francisco (arrival ~9:00 AM)

## How to Load This Route

### Option 1: Using SQLite with SpatiaLite (Manual)

```bash
nix-shell
cd backend

# Connect to database and load extension
sqlite3 cache.db
.load $SPATIALITE_LIB_PATH

# Copy/paste the SQL from migrations/002_seattle_to_sf_route.sql
.read migrations/002_seattle_to_sf_route.sql
```

### Option 2: Using Rust (Coming Soon)

We'll add a command to the CLI tool:

```bash
cargo run --release --bin cache-cli -- load-route seattle-sf
```

## Query the Route

Once loaded, try these queries:

```sql
-- View all waypoints
SELECT name, lat, lon, category, description
FROM waypoints
WHERE id LIKE 'wp-sea-%'
ORDER BY id;

-- View the trajectory
SELECT name, total_distance_m / 1852.0 as nautical_miles, description
FROM trajectories
WHERE id = 'traj-sea-sf-001';

-- View the plan (in Pacific Time)
SELECT * FROM plan_pacific_time
WHERE id = 'plan-sea-sf-001';

-- View beacon
SELECT name, type, active, metadata_json
FROM beacons
WHERE id = 'beacon-pacific-001';

-- View initial track points
SELECT timestamp, lat, lon, speed, metadata_json
FROM tracks
WHERE beacon_id = 'beacon-pacific-001'
ORDER BY timestamp;

-- Calculate distance to each waypoint from Seattle
SELECT
    name,
    Distance(geom, MakePoint(-122.3589, 47.5881, 4326)) / 1852.0 as nm_from_seattle
FROM waypoints
WHERE id LIKE 'wp-sea-%'
ORDER BY id;
```

## Hazards & Considerations

**Columbia River Bar** (Astoria)
- One of the most dangerous bar crossings in the world
- Check conditions before attempting
- Best crossed on flood tide with calm conditions

**Cape Blanco** (Between Coos Bay and Crescent City)
- Westernmost point in Oregon
- Often rough conditions
- Plan to round in daylight with favorable weather

**Point Arena** (Northern California)
- Another challenging point to round
- Monitor weather forecasts carefully

**Fog**
- Common along entire coast, especially in summer
- Have radar or stay close to shore
- Sound fog signals

## Weather Window

**Favorable Conditions**:
- Light NW winds (10-15 knots)
- Calm seas (2-4 ft)
- Good visibility
- Stable barometric pressure

**Plan includes**:
- Daily weather updates
- Tide calculations for bar crossings
- Sunrise/sunset times for each waypoint
- Moon phase for night watches

## Time Zone Handling

All timestamps are stored in UTC:
- **8:00 AM PDT** = 15:00 UTC (stored as `2026-09-28T15:00:00Z`)
- **8:00 AM PST** = 16:00 UTC (after November DST change)

The system automatically converts for display based on user's timezone preference (default: America/Los_Angeles).

## Astronomical Data (Future)

For this voyage, the system will calculate:
- Sunrise/sunset at each waypoint
- Twilight hours (civil, nautical, astronomical)
- Moon phase and illumination
- Moonrise/moonset times

Useful for:
- Planning daylight sailing hours
- Night watch scheduling
- Photography opportunities
- Celestial navigation

## Real-World Data

This route uses:
- **Real coordinates** from major Pacific Coast ports
- **Realistic distances** based on actual nautical miles
- **Typical timing** for a cruising sailboat (5-6 knots average)
- **Actual hazards** documented by Pacific Coast cruising guides

## Sources

Route information compiled from:
- [Pacific Coast cruising guides](https://www.sailinganarchy.com/threads/seattle-to-san-francisco.130794/)
- [NOAA Coast Pilot](https://www.nauticalcharts.noaa.gov/publications/coast-pilot/)
- [Harbor coordinates](https://www.geodatos.net/)
- Sailing forums and cruiser reports

## Next Steps

1. **Load the route** (see options above)
2. **Fetch real weather** for the route in late September 2026
3. **Calculate tide predictions** for Columbia River Bar
4. **Add more track points** showing the full voyage
5. **Fetch real satellite tiles** along the coastal route
6. **Calculate astronomical data** for each day of the voyage

---

**Happy sailing! ⛵**
