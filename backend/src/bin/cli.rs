use anyhow::{Context, Result};
use cache::{init_database, run_migrations, calculate_trajectory_distance, models::*, astronomy};
use chrono::{Duration, Timelike, Utc};
use clap::{Parser, Subcommand};
use fake::{Fake, Faker};
use rand::Rng;
use rusqlite::Connection;
use serde_json::Value;
use tracing::{info, warn};

#[derive(Parser)]
#[command(name = "cache-cli")]
#[command(about = "Cache database CLI tool", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize the database
    Init {
        #[arg(short, long, default_value = "cache.db")]
        database: String,
    },
    /// Seed the database with sample data
    Seed {
        #[arg(short, long, default_value = "cache.db")]
        database: String,

        #[arg(long, default_value = "100")]
        waypoints: usize,

        #[arg(long, default_value = "10")]
        beacons: usize,

        #[arg(long, default_value = "20")]
        trajectories: usize,

        #[arg(long, default_value = "5")]
        plans: usize,

        #[arg(long, default_value = "1000")]
        tracks: usize,

        #[arg(long, default_value = "50")]
        tiles: usize,

        #[arg(long)]
        fetch_weather: bool,
    },
    /// Calculate astronomical data (sun/moon) for a location
    Astro {
        /// Latitude in degrees (-90 to 90)
        #[arg(short, long)]
        lat: f64,

        /// Longitude in degrees (-180 to 180)
        #[arg(short = 'o', long)]
        lon: f64,

        /// Date in YYYY-MM-DD format (defaults to today)
        #[arg(short, long)]
        date: Option<String>,
    },
    /// Load Seattle to SF route data
    LoadRoute {
        #[arg(short, long, default_value = "cache.db")]
        database: String,
    },
    /// Get weather forecast along a route
    RouteWeather {
        #[arg(short, long, default_value = "cache.db")]
        database: String,

        /// Route/trajectory ID (defaults to Seattle-SF route)
        #[arg(short, long, default_value = "traj-sea-sf-001")]
        route_id: String,

        /// Start date in YYYY-MM-DD format (defaults to plan start date)
        #[arg(short, long)]
        start_date: Option<String>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Init { database } => {
            info!("Initializing database at {}", database);
            let conn = init_database(&database)?;
            run_migrations(&conn)?;
            info!("✓ Database initialized successfully");
        }
        Commands::Seed {
            database,
            waypoints,
            beacons,
            trajectories,
            plans,
            tracks,
            tiles,
            fetch_weather,
        } => {
            info!("Seeding database at {}", database);
            let conn = init_database(&database)?;
            run_migrations(&conn)?;

            seed_database(
                &conn,
                waypoints,
                beacons,
                trajectories,
                plans,
                tracks,
                tiles,
                fetch_weather,
            )
            .await?;

            info!("✓ Database seeded successfully");
        }
        Commands::Astro { lat, lon, date } => {
            // Parse date or use today
            let date = if let Some(date_str) = date {
                chrono::NaiveDate::parse_from_str(&date_str, "%Y-%m-%d")
                    .context("Invalid date format. Use YYYY-MM-DD")?
                    .and_hms_opt(12, 0, 0)
                    .ok_or_else(|| anyhow::anyhow!("Invalid time"))?
                    .and_utc()
            } else {
                Utc::now()
            };

            println!("\n🌍 Location: {:.4}°N, {:.4}°E", lat, lon);
            println!("📅 Date: {}\n", date.format("%Y-%m-%d"));

            // Calculate solar times
            let solar = astronomy::calculate_solar_times(lat, lon, date);
            println!("☀️  SOLAR DATA:");
            println!("   Sunrise:           {}", solar.sunrise.format("%H:%M:%S UTC"));
            println!("   Sunset:            {}", solar.sunset.format("%H:%M:%S UTC"));
            println!("   Solar Noon:        {}", solar.solar_noon.format("%H:%M:%S UTC"));
            println!("   Civil Dawn:        {}", solar.civil_dawn.format("%H:%M:%S UTC"));
            println!("   Civil Dusk:        {}", solar.civil_dusk.format("%H:%M:%S UTC"));
            println!("   Nautical Dawn:     {}", solar.nautical_dawn.format("%H:%M:%S UTC"));
            println!("   Nautical Dusk:     {}", solar.nautical_dusk.format("%H:%M:%S UTC"));
            println!("   Astronomical Dawn: {}", solar.astronomical_dawn.format("%H:%M:%S UTC"));
            println!("   Astronomical Dusk: {}", solar.astronomical_dusk.format("%H:%M:%S UTC"));

            // Calculate daylight duration
            let daylight = solar.sunset.signed_duration_since(solar.sunrise);
            println!("   Daylight Duration: {}h {}m", daylight.num_hours(), daylight.num_minutes() % 60);

            // Calculate lunar times
            let lunar = astronomy::calculate_lunar_times(lat, lon, date);
            println!("\n🌙 LUNAR DATA:");
            if let Some(moonrise) = lunar.moonrise {
                println!("   Moonrise:          {}", moonrise.format("%H:%M:%S UTC"));
            } else {
                println!("   Moonrise:          No moonrise today");
            }
            if let Some(moonset) = lunar.moonset {
                println!("   Moonset:           {}", moonset.format("%H:%M:%S UTC"));
            } else {
                println!("   Moonset:           No moonset today");
            }
            println!("   Phase:             {} ({:.1}%)", lunar.phase_name, lunar.phase * 100.0);
            println!("   Illumination:      {:.1}%", lunar.illumination * 100.0);

            // Calculate current solar position
            let (sun_alt, sun_az) = astronomy::get_solar_position(lat, lon, date);
            println!("\n☀️  SOLAR POSITION (at {}UTC):", date.format("%H:%M:%S"));
            println!("   Altitude:          {:.2}°", sun_alt);
            println!("   Azimuth:           {:.2}°", sun_az);

            // Calculate current lunar position
            let (moon_alt, moon_az, moon_dist) = astronomy::get_lunar_position(lat, lon, date);
            println!("\n🌙 LUNAR POSITION (at {}UTC):", date.format("%H:%M:%S"));
            println!("   Altitude:          {:.2}°", moon_alt);
            println!("   Azimuth:           {:.2}°", moon_az);
            println!("   Distance:          {:.0} km", moon_dist);

            println!();
        }
        Commands::LoadRoute { database } => {
            info!("Loading Seattle → San Francisco route data");
            let conn = init_database(&database)?;

            // Read and execute the SQL file
            let sql = include_str!("../../migrations/002_seattle_to_sf_route.sql");

            // Execute in a transaction
            conn.execute_batch("BEGIN TRANSACTION;")?;

            match conn.execute_batch(sql) {
                Ok(_) => {
                    conn.execute_batch("COMMIT;")?;
                    info!("✓ Route data loaded successfully");

                    // Verify
                    let count: i64 = conn.query_row(
                        "SELECT COUNT(*) FROM waypoints WHERE id LIKE 'wp-sea-%'",
                        [],
                        |row| row.get(0),
                    )?;

                    println!("\n✓ Loaded {} waypoints for Seattle → SF route", count);
                    println!("✓ Route ID: traj-sea-sf-001");
                    println!("✓ Plan ID: plan-sea-sf-001");
                    println!("\nRun: cargo run --release --bin cache-cli -- route-weather\n");
                }
                Err(e) => {
                    conn.execute_batch("ROLLBACK;")?;
                    if e.to_string().contains("UNIQUE constraint") {
                        warn!("Route data already loaded");
                        println!("\n⚠️  Route data already exists in database");
                    } else {
                        return Err(e.into());
                    }
                }
            }
        }
        Commands::RouteWeather {
            database,
            route_id,
            start_date,
        } => {
            let conn = init_database(&database)?;

            info!("Loading route: {}", route_id);

            // Get trajectory and waypoints
            let trajectory: (String, String, String, String) = conn.query_row(
                "SELECT id, name, description, waypoint_ids FROM trajectories WHERE id = ?1",
                [&route_id],
                |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?)),
            )?;

            let waypoint_ids: Vec<String> = serde_json::from_str(&trajectory.3)?;

            println!("\n🗺️  Route: {}", trajectory.1);
            println!("📝 {}", trajectory.2);
            println!("📍 {} waypoints\n", waypoint_ids.len());

            // Get plan to determine timing
            let plan_result: Result<(String, i64), _> = conn.query_row(
                "SELECT start_time, estimated_duration_seconds FROM plans WHERE trajectory_id = ?1 LIMIT 1",
                [&route_id],
                |row| Ok((row.get(0)?, row.get(1)?)),
            );

            let (start_time_str, total_duration) = if let Ok((st, dur)) = plan_result {
                (st, dur)
            } else {
                // Use provided start date or default to now
                let start = if let Some(date_str) = start_date {
                    chrono::NaiveDate::parse_from_str(&date_str, "%Y-%m-%d")?
                        .and_hms_opt(8, 0, 0)
                        .ok_or_else(|| anyhow::anyhow!("Invalid time"))?
                        .and_utc()
                } else {
                    Utc::now()
                };
                (start.to_rfc3339(), 518400) // Default 6 days
            };

            let start_time = chrono::DateTime::parse_from_rfc3339(&start_time_str)?
                .with_timezone(&Utc);

            println!("🚢 Departure: {} UTC", start_time.format("%Y-%m-%d %H:%M"));
            println!("⏱️  Duration: {} days\n", total_duration / 86400);

            // Calculate time per waypoint (evenly distributed for now)
            let hours_per_waypoint = (total_duration as f64 / 3600.0) / waypoint_ids.len() as f64;

            // Fetch weather for each waypoint
            let client = reqwest::Client::new();

            for (idx, wp_id) in waypoint_ids.iter().enumerate() {
                let waypoint: (String, f64, f64, Option<String>) = conn.query_row(
                    "SELECT name, lat, lon, category FROM waypoints WHERE id = ?1",
                    [wp_id],
                    |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?)),
                )?;

                let arrival_time = start_time + chrono::Duration::hours((hours_per_waypoint * idx as f64) as i64);

                println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                println!("📍 Waypoint {}: {}", idx + 1, waypoint.0);
                println!("   Coordinates: {:.4}°N, {:.4}°W", waypoint.1, waypoint.2.abs());
                println!("   Arrival: {}", arrival_time.format("%a %b %d, %Y at %H:%M UTC"));

                // Fetch weather from Open-Meteo
                let url = format!(
                    "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&hourly=temperature_2m,windspeed_10m,winddirection_10m,weathercode&timezone=UTC&start_date={}&end_date={}",
                    waypoint.1,
                    waypoint.2,
                    arrival_time.format("%Y-%m-%d"),
                    arrival_time.format("%Y-%m-%d")
                );

                match client.get(&url).send().await {
                    Ok(response) => {
                        if let Ok(data) = response.json::<Value>().await {
                            if let Some(hourly) = data.get("hourly") {
                                let target_hour = arrival_time.hour() as usize;

                                if let (Some(temps), Some(winds), Some(wind_dirs)) = (
                                    hourly.get("temperature_2m").and_then(|v| v.as_array()),
                                    hourly.get("windspeed_10m").and_then(|v| v.as_array()),
                                    hourly.get("winddirection_10m").and_then(|v| v.as_array()),
                                ) {
                                    if target_hour < temps.len() {
                                        let temp = temps[target_hour].as_f64().unwrap_or(0.0);
                                        let wind_speed = winds[target_hour].as_f64().unwrap_or(0.0);
                                        let wind_dir = wind_dirs[target_hour].as_f64().unwrap_or(0.0);

                                        println!("\n   🌡️  Weather Forecast:");
                                        println!("      Temperature: {:.1}°C ({:.1}°F)", temp, temp * 9.0/5.0 + 32.0);
                                        println!("      Wind: {:.1} km/h from {}°", wind_speed, wind_dir);
                                        println!("      Wind: {:.1} knots from {}", wind_speed / 1.852, cardinal_direction(wind_dir));
                                    }
                                }
                            }
                        } else {
                            println!("\n   ⚠️  Could not parse weather data");
                        }
                    }
                    Err(e) => {
                        println!("\n   ⚠️  Could not fetch weather: {}", e);
                    }
                }

                // Calculate astronomical data
                let solar = astronomy::calculate_solar_times(waypoint.1, waypoint.2, arrival_time);
                let lunar = astronomy::calculate_lunar_times(waypoint.1, waypoint.2, arrival_time);

                println!("\n   ☀️  Sun:");
                println!("      Sunrise: {}", solar.sunrise.format("%H:%M UTC"));
                println!("      Sunset:  {}", solar.sunset.format("%H:%M UTC"));
                println!("      Daylight: {}h {}m",
                    solar.sunset.signed_duration_since(solar.sunrise).num_hours(),
                    solar.sunset.signed_duration_since(solar.sunrise).num_minutes() % 60
                );

                println!("\n   🌙 Moon:");
                println!("      Phase: {} ({:.0}% illuminated)", lunar.phase_name, lunar.illumination * 100.0);

                println!();
            }

            println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
        }
    }

    Ok(())
}

/// Convert wind direction degrees to cardinal direction
fn cardinal_direction(degrees: f64) -> &'static str {
    let normalized = ((degrees % 360.0) + 360.0) % 360.0;
    match normalized {
        d if d < 22.5 => "N",
        d if d < 67.5 => "NE",
        d if d < 112.5 => "E",
        d if d < 157.5 => "SE",
        d if d < 202.5 => "S",
        d if d < 247.5 => "SW",
        d if d < 292.5 => "W",
        d if d < 337.5 => "NW",
        _ => "N",
    }
}

async fn seed_database(
    conn: &Connection,
    num_waypoints: usize,
    num_beacons: usize,
    num_trajectories: usize,
    num_plans: usize,
    num_tracks: usize,
    num_tiles: usize,
    fetch_weather: bool,
) -> Result<()> {
    let mut rng = rand::thread_rng();

    // Define a bounding box for sample data (San Francisco Bay Area)
    let bbox = BoundingBox {
        min_lat: 37.2,
        max_lat: 38.0,
        min_lon: -122.8,
        max_lon: -121.5,
    };

    // 1. Create waypoints
    info!("Creating {} waypoints...", num_waypoints);
    let waypoint_ids = create_waypoints(conn, &bbox, num_waypoints)?;
    info!("✓ Created {} waypoints", waypoint_ids.len());

    // 2. Create beacons
    info!("Creating {} beacons...", num_beacons);
    let beacon_ids = create_beacons(conn, num_beacons)?;
    info!("✓ Created {} beacons", beacon_ids.len());

    // 3. Create trajectories
    info!("Creating {} trajectories...", num_trajectories);
    let trajectory_ids = create_trajectories(conn, &waypoint_ids, num_trajectories)?;
    info!("✓ Created {} trajectories", trajectory_ids.len());

    // 4. Create plans
    info!("Creating {} plans...", num_plans);
    let plan_ids = create_plans(conn, &trajectory_ids, &beacon_ids, num_plans)?;
    info!("✓ Created {} plans", plan_ids.len());

    // 5. Create tracks (GPS history)
    info!("Creating {} tracks...", num_tracks);
    create_tracks(conn, &beacon_ids, &bbox, num_tracks)?;
    info!("✓ Created {} tracks", num_tracks);

    // 6. Create sensors
    info!("Creating weather sensors...");
    let sensor_ids = create_sensors(conn)?;
    info!("✓ Created {} sensors", sensor_ids.len());

    // 7. Fetch real weather data if requested
    if fetch_weather {
        info!("Fetching real weather data from Open-Meteo...");
        fetch_and_store_weather(conn, &sensor_ids, &bbox).await?;
        info!("✓ Fetched and stored weather data");
    } else {
        info!("Creating synthetic sensor readings...");
        create_readings(conn, &sensor_ids, &bbox, 100)?;
        info!("✓ Created sensor readings");
    }

    // 8. Fetch real tiles
    info!("Fetching {} real tiles from OpenFreeMap...", num_tiles);
    fetch_and_store_tiles(conn, &bbox, num_tiles).await?;
    info!("✓ Fetched and stored {} tiles", num_tiles);

    Ok(())
}

struct BoundingBox {
    min_lat: f64,
    max_lat: f64,
    min_lon: f64,
    max_lon: f64,
}

impl BoundingBox {
    fn random_point(&self) -> (f64, f64) {
        let mut rng = rand::thread_rng();
        let lat = rng.gen_range(self.min_lat..self.max_lat);
        let lon = rng.gen_range(self.min_lon..self.max_lon);
        (lat, lon)
    }
}

fn create_waypoints(conn: &Connection, bbox: &BoundingBox, count: usize) -> Result<Vec<String>> {
    let categories = vec!["harbor", "anchorage", "poi", "fuel", "hazard", "lighthouse"];
    let colors = vec!["#FF6B6B", "#4ECDC4", "#45B7D1", "#FFA07A", "#98D8C8", "#F7DC6F"];
    let mut waypoint_ids = Vec::new();

    for i in 0..count {
        let (lat, lon) = bbox.random_point();
        let mut waypoint = Waypoint::new(
            format!("Waypoint {}", i + 1),
            lat,
            lon,
        );

        waypoint.category = Some(categories[i % categories.len()].to_string());
        waypoint.color = Some(colors[i % colors.len()].to_string());
        waypoint.description = Some(Faker.fake());

        conn.execute(
            "INSERT INTO waypoints (id, name, description, lat, lon, altitude, category, color,
                                    metadata_json, user_id, created_at, updated_at, version, geom)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, MakePoint(?14, ?15, 4326))",
            (
                &waypoint.id,
                &waypoint.name,
                &waypoint.description,
                waypoint.lat,
                waypoint.lon,
                waypoint.altitude,
                &waypoint.category,
                &waypoint.color,
                &waypoint.metadata_json,
                &waypoint.user_id,
                &waypoint.created_at,
                &waypoint.updated_at,
                waypoint.version,
                waypoint.lon,
                waypoint.lat,
            ),
        )?;

        waypoint_ids.push(waypoint.id);
    }

    Ok(waypoint_ids)
}

fn create_beacons(conn: &Connection, count: usize) -> Result<Vec<String>> {
    let types = vec!["vessel", "vehicle", "person", "aircraft"];
    let colors = vec!["#E74C3C", "#3498DB", "#2ECC71", "#F39C12"];
    let mut beacon_ids = Vec::new();

    for i in 0..count {
        let beacon_type = types[i % types.len()].to_string();
        let mut beacon = Beacon::new(
            format!("Beacon {}", i + 1),
            beacon_type,
        );

        beacon.color = Some(colors[i % colors.len()].to_string());

        conn.execute(
            "INSERT INTO beacons (id, name, type, icon, color, sensors, active, last_seen,
                                  metadata_json, user_id, created_at, updated_at, version)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
            (
                &beacon.id,
                &beacon.name,
                &beacon.r#type,
                &beacon.icon,
                &beacon.color,
                &beacon.sensors,
                beacon.active as i32,
                &beacon.last_seen,
                &beacon.metadata_json,
                &beacon.user_id,
                &beacon.created_at,
                &beacon.updated_at,
                beacon.version,
            ),
        )?;

        beacon_ids.push(beacon.id);
    }

    Ok(beacon_ids)
}

fn create_trajectories(
    conn: &Connection,
    waypoint_ids: &[String],
    count: usize,
) -> Result<Vec<String>> {
    let mut rng = rand::thread_rng();
    let mut trajectory_ids = Vec::new();

    for i in 0..count {
        // Select 3-8 random waypoints for each trajectory
        let num_waypoints = rng.gen_range(3..=8);
        let selected_waypoints: Vec<String> = (0..num_waypoints)
            .map(|_| waypoint_ids[rng.gen_range(0..waypoint_ids.len())].clone())
            .collect();

        // Calculate distance
        let waypoint_coords: Result<Vec<(f64, f64)>> = selected_waypoints
            .iter()
            .map(|id| {
                let mut stmt = conn.prepare("SELECT lat, lon FROM waypoints WHERE id = ?1")?;
                let (lat, lon): (f64, f64) = stmt.query_row([id], |row| Ok((row.get(0)?, row.get(1)?)))?;
                Ok((lat, lon))
            })
            .collect();

        let coords = waypoint_coords?;
        let distance = calculate_trajectory_distance(&coords);

        let mut trajectory = Trajectory::new(
            format!("Route {}", i + 1),
            selected_waypoints.clone(),
        );
        trajectory.total_distance_m = Some(distance);

        // Create LineString geometry from waypoints
        let wkt_coords: Vec<String> = coords
            .iter()
            .map(|(lat, lon)| format!("{} {}", lon, lat))
            .collect();
        let linestring_wkt = format!("LINESTRING({})", wkt_coords.join(", "));

        conn.execute(
            "INSERT INTO trajectories (id, name, description, waypoint_ids, directionality,
                                       total_distance_m, metadata_json, user_id, created_at,
                                       updated_at, version, geom)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, GeomFromText(?12, 4326))",
            (
                &trajectory.id,
                &trajectory.name,
                &trajectory.description,
                &trajectory.waypoint_ids,
                &trajectory.directionality,
                trajectory.total_distance_m,
                &trajectory.metadata_json,
                &trajectory.user_id,
                &trajectory.created_at,
                &trajectory.updated_at,
                trajectory.version,
                &linestring_wkt,
            ),
        )?;

        trajectory_ids.push(trajectory.id);
    }

    Ok(trajectory_ids)
}

fn create_plans(
    conn: &Connection,
    trajectory_ids: &[String],
    beacon_ids: &[String],
    count: usize,
) -> Result<Vec<String>> {
    let mut rng = rand::thread_rng();
    let mut plan_ids = Vec::new();
    let statuses = vec!["draft", "active", "completed"];

    for i in 0..count {
        let trajectory_id = &trajectory_ids[i % trajectory_ids.len()];
        let beacon_id = if i < beacon_ids.len() {
            Some(&beacon_ids[i])
        } else {
            None
        };

        // Start time: random time in the next 30 days
        let days_offset = rng.gen_range(0..30);
        let start_time = Utc::now() + Duration::days(days_offset);

        let mut plan = Plan::new(
            trajectory_id.clone(),
            format!("Plan {}", i + 1),
            start_time,
        );

        plan.status = statuses[i % statuses.len()].to_string();
        plan.beacon_id = beacon_id.cloned();
        plan.estimated_duration_seconds = Some(rng.gen_range(3600..86400)); // 1-24 hours

        conn.execute(
            "INSERT INTO plans (id, trajectory_id, name, description, start_time,
                               estimated_duration_seconds, waypoint_times, pause_durations,
                               status, beacon_id, metadata_json, user_id, created_at,
                               updated_at, version)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15)",
            (
                &plan.id,
                &plan.trajectory_id,
                &plan.name,
                &plan.description,
                &plan.start_time,
                plan.estimated_duration_seconds,
                &plan.waypoint_times,
                &plan.pause_durations,
                &plan.status,
                &plan.beacon_id,
                &plan.metadata_json,
                &plan.user_id,
                &plan.created_at,
                &plan.updated_at,
                plan.version,
            ),
        )?;

        plan_ids.push(plan.id);
    }

    Ok(plan_ids)
}

fn create_tracks(
    conn: &Connection,
    beacon_ids: &[String],
    bbox: &BoundingBox,
    total_tracks: usize,
) -> Result<()> {
    let tracks_per_beacon = total_tracks / beacon_ids.len().max(1);
    let mut rng = rand::thread_rng();

    for beacon_id in beacon_ids {
        let mut lat = (bbox.min_lat + bbox.max_lat) / 2.0;
        let mut lon = (bbox.min_lon + bbox.max_lon) / 2.0;

        for i in 0..tracks_per_beacon {
            // Simulate movement (random walk)
            lat += rng.gen_range(-0.01..0.01);
            lon += rng.gen_range(-0.01..0.01);

            // Clamp to bbox
            lat = lat.clamp(bbox.min_lat, bbox.max_lat);
            lon = lon.clamp(bbox.min_lon, bbox.max_lon);

            let timestamp = Utc::now() - Duration::minutes((tracks_per_beacon - i) as i64 * 5);

            let track = Track::new(beacon_id.clone(), lat, lon, timestamp);

            conn.execute(
                "INSERT INTO tracks (id, beacon_id, lat, lon, altitude, heading, speed, accuracy,
                                    source, timestamp, metadata_json, created_at, geom)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, MakePoint(?13, ?14, 4326))",
                (
                    &track.id,
                    &track.beacon_id,
                    track.lat,
                    track.lon,
                    &track.altitude,
                    &track.heading,
                    &track.speed,
                    &track.accuracy,
                    &track.source,
                    &track.timestamp,
                    &track.metadata_json,
                    &track.created_at,
                    track.lon,
                    track.lat,
                ),
            )?;
        }
    }

    Ok(())
}

fn create_sensors(conn: &Connection) -> Result<Vec<String>> {
    let sensors_config = vec![
        ("Temperature", "temperature", "celsius", "forecast", "api"),
        ("Wind Speed", "wind_speed", "m/s", "forecast", "api"),
        ("Wind Direction", "wind_direction", "degrees", "forecast", "api"),
        ("Humidity", "humidity", "percent", "forecast", "api"),
        ("Pressure", "pressure", "hPa", "forecast", "api"),
    ];

    let mut sensor_ids = Vec::new();

    for (name, sensor_type, unit, fidelity, origin) in sensors_config {
        let sensor = Sensor::new(
            name.to_string(),
            sensor_type.to_string(),
            unit.to_string(),
            fidelity.to_string(),
            origin.to_string(),
        );

        conn.execute(
            "INSERT INTO sensors (id, name, type, unit, fidelity, origin, source_identifier,
                                 metadata_json, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            (
                &sensor.id,
                &sensor.name,
                &sensor.r#type,
                &sensor.unit,
                &sensor.fidelity,
                &sensor.origin,
                &sensor.source_identifier,
                &sensor.metadata_json,
                &sensor.created_at,
                &sensor.updated_at,
            ),
        )?;

        sensor_ids.push(sensor.id);
    }

    Ok(sensor_ids)
}

fn create_readings(
    conn: &Connection,
    sensor_ids: &[String],
    bbox: &BoundingBox,
    count: usize,
) -> Result<()> {
    let mut rng = rand::thread_rng();

    for _ in 0..count {
        let sensor_id = &sensor_ids[rng.gen_range(0..sensor_ids.len())];
        let (lat, lon) = bbox.random_point();

        // Generate realistic values based on sensor type
        let sensor_type: String = conn.query_row(
            "SELECT type FROM sensors WHERE id = ?1",
            [sensor_id],
            |row| row.get(0),
        )?;

        let value = match sensor_type.as_str() {
            "temperature" => rng.gen_range(10.0..30.0),
            "wind_speed" => rng.gen_range(0.0..20.0),
            "wind_direction" => rng.gen_range(0.0..360.0),
            "humidity" => rng.gen_range(30.0..90.0),
            "pressure" => rng.gen_range(990.0..1030.0),
            _ => rng.gen_range(0.0..100.0),
        };

        let timestamp = Utc::now() + Duration::hours(rng.gen_range(-24..48));
        let reading = Reading::new(sensor_id.clone(), lat, lon, value, timestamp);

        conn.execute(
            "INSERT INTO readings (id, sensor_id, lat, lon, value, timestamp, forecast_time,
                                  quality, source_id, metadata_json, created_at, geom)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, MakePoint(?12, ?13, 4326))",
            (
                &reading.id,
                &reading.sensor_id,
                reading.lat,
                reading.lon,
                reading.value,
                &reading.timestamp,
                &reading.forecast_time,
                &reading.quality,
                &reading.source_id,
                &reading.metadata_json,
                &reading.created_at,
                reading.lon,
                reading.lat,
            ),
        )?;
    }

    Ok(())
}

async fn fetch_and_store_weather(
    conn: &Connection,
    sensor_ids: &[String],
    bbox: &BoundingBox,
) -> Result<()> {
    // Use center point of bbox for weather query
    let lat = (bbox.min_lat + bbox.max_lat) / 2.0;
    let lon = (bbox.min_lon + bbox.max_lon) / 2.0;

    // Open-Meteo API endpoint
    let url = format!(
        "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&hourly=temperature_2m,windspeed_10m,winddirection_10m,relativehumidity_2m,surface_pressure",
        lat, lon
    );

    info!("Fetching weather from: {}", url);

    let client = reqwest::Client::new();
    let response = client.get(&url).send().await?;
    let data: Value = response.json().await?;

    // Parse sensor IDs by type
    let mut sensor_map = std::collections::HashMap::new();
    for id in sensor_ids {
        let sensor_type: String = conn.query_row(
            "SELECT type FROM sensors WHERE id = ?1",
            [id],
            |row| row.get(0),
        )?;
        sensor_map.insert(sensor_type, id.clone());
    }

    // Extract hourly data
    if let Some(hourly) = data["hourly"].as_object() {
        if let Some(times) = hourly["time"].as_array() {
            let temperatures = hourly["temperature_2m"].as_array();
            let windspeeds = hourly["windspeed_10m"].as_array();
            let winddirs = hourly["winddirection_10m"].as_array();
            let humidities = hourly["relativehumidity_2m"].as_array();
            let pressures = hourly["surface_pressure"].as_array();

            for (i, time) in times.iter().enumerate() {
                let timestamp = time.as_str().unwrap();

                // Temperature
                if let (Some(temps), Some(sensor_id)) = (temperatures, sensor_map.get("temperature")) {
                    if let Some(value) = temps[i].as_f64() {
                        insert_weather_reading(conn, sensor_id, lat, lon, value, timestamp)?;
                    }
                }

                // Wind speed
                if let (Some(speeds), Some(sensor_id)) = (windspeeds, sensor_map.get("wind_speed")) {
                    if let Some(value) = speeds[i].as_f64() {
                        insert_weather_reading(conn, sensor_id, lat, lon, value, timestamp)?;
                    }
                }

                // Wind direction
                if let (Some(dirs), Some(sensor_id)) = (winddirs, sensor_map.get("wind_direction")) {
                    if let Some(value) = dirs[i].as_f64() {
                        insert_weather_reading(conn, sensor_id, lat, lon, value, timestamp)?;
                    }
                }

                // Humidity
                if let (Some(hums), Some(sensor_id)) = (humidities, sensor_map.get("humidity")) {
                    if let Some(value) = hums[i].as_f64() {
                        insert_weather_reading(conn, sensor_id, lat, lon, value, timestamp)?;
                    }
                }

                // Pressure
                if let (Some(press), Some(sensor_id)) = (pressures, sensor_map.get("pressure")) {
                    if let Some(value) = press[i].as_f64() {
                        insert_weather_reading(conn, sensor_id, lat, lon, value, timestamp)?;
                    }
                }
            }
        }
    }

    Ok(())
}

fn insert_weather_reading(
    conn: &Connection,
    sensor_id: &str,
    lat: f64,
    lon: f64,
    value: f64,
    timestamp: &str,
) -> Result<()> {
    let reading_id = uuid::Uuid::new_v4().to_string();
    let created_at = Utc::now().to_rfc3339();

    conn.execute(
        "INSERT INTO readings (id, sensor_id, lat, lon, value, timestamp, forecast_time,
                              quality, source_id, metadata_json, created_at, geom)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, MakePoint(?12, ?13, 4326))",
        (
            &reading_id,
            sensor_id,
            lat,
            lon,
            value,
            timestamp,
            Some(timestamp), // forecast_time same as timestamp for forecasts
            Some(1.0), // high quality from API
            Some("open-meteo"),
            None::<String>,
            &created_at,
            lon,
            lat,
        ),
    )?;

    Ok(())
}

async fn fetch_and_store_tiles(
    conn: &Connection,
    bbox: &BoundingBox,
    count: usize,
) -> Result<()> {
    let client = reqwest::Client::new();

    // Calculate center tile at zoom 12
    let zoom = 12;
    let center_lat = (bbox.min_lat + bbox.max_lat) / 2.0;
    let center_lon = (bbox.min_lon + bbox.max_lon) / 2.0;

    let (center_x, center_y) = lat_lon_to_tile(center_lat, center_lon, zoom);

    info!("Fetching tiles around center tile {}/{}/{}", zoom, center_x, center_y);

    let radius = (count as f64).sqrt().ceil() as i32 / 2;
    let mut fetched = 0;

    for dx in -radius..=radius {
        for dy in -radius..=radius {
            if fetched >= count {
                break;
            }

            let x = center_x + dx;
            let y = center_y + dy;

            // OpenFreeMap tile URL
            let url = format!(
                "https://tiles.openfreemap.org/osm/{}/{}/{}.png",
                zoom, x, y
            );

            match client.get(&url).send().await {
                Ok(response) if response.status().is_success() => {
                    if let Ok(bytes) = response.bytes().await {
                        let tile = Tile::new(zoom, x, y, bytes.to_vec(), "openfreemap".to_string());

                        conn.execute(
                            "INSERT OR REPLACE INTO tiles (zoom_level, tile_column, tile_row, tile_data,
                                                          tile_source, last_accessed, created_at)
                             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                            (
                                tile.zoom_level,
                                tile.tile_column,
                                tile.tile_row,
                                &tile.tile_data,
                                &tile.tile_source,
                                &tile.last_accessed,
                                &tile.created_at,
                            ),
                        )?;

                        fetched += 1;
                        if fetched % 10 == 0 {
                            info!("Fetched {}/{} tiles", fetched, count);
                        }
                    }
                }
                Ok(response) => {
                    warn!("Failed to fetch tile {}/{}/{}: {}", zoom, x, y, response.status());
                }
                Err(e) => {
                    warn!("Error fetching tile {}/{}/{}: {}", zoom, x, y, e);
                }
            }
        }
        if fetched >= count {
            break;
        }
    }

    Ok(())
}

// Convert lat/lon to tile coordinates
fn lat_lon_to_tile(lat: f64, lon: f64, zoom: i32) -> (i32, i32) {
    let n = 2_f64.powi(zoom);
    let x = ((lon + 180.0) / 360.0 * n).floor() as i32;
    let y = ((1.0 - (lat.to_radians().tan() + 1.0 / lat.to_radians().cos()).ln() / std::f64::consts::PI) / 2.0 * n).floor() as i32;
    (x, y)
}
