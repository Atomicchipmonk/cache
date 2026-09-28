pub mod models;
pub mod astronomy;

use anyhow::{Context, Result};
use rusqlite::Connection;
use std::path::Path;

/// Initialize a SQLite database with SpatiaLite extension
pub fn init_database<P: AsRef<Path>>(db_path: P) -> Result<Connection> {
    let conn = Connection::open(&db_path)
        .context("Failed to open database")?;

    // Load SpatiaLite extension
    unsafe {
        conn.load_extension_enable()?;

        // Try loading from environment variable first (set by Nix shell)
        if let Ok(spatialite_path) = std::env::var("SPATIALITE_LIB_PATH") {
            conn.load_extension(Path::new(&spatialite_path), None)
                .context(format!("Failed to load SpatiaLite from {}", spatialite_path))?;
        } else {
            // Fallback to standard locations
            let possible_paths = vec![
                "/usr/lib/x86_64-linux-gnu/mod_spatialite.so",
                "/usr/lib/mod_spatialite.so",
                "/usr/local/lib/mod_spatialite.so",
                "mod_spatialite",
            ];

            let mut loaded = false;
            for path in possible_paths {
                if let Ok(_) = conn.load_extension(path, None) {
                    loaded = true;
                    break;
                }
            }

            if !loaded {
                anyhow::bail!("Could not load SpatiaLite extension. Set SPATIALITE_LIB_PATH env var.");
            }
        }

        conn.load_extension_disable()?;
    }

    // Enable foreign keys
    conn.execute_batch("PRAGMA foreign_keys = ON;")?;

    Ok(conn)
}

/// Run database migrations
pub fn run_migrations(conn: &Connection) -> Result<()> {
    let migration_sql = include_str!("../migrations/001_initial_schema.sql");

    conn.execute_batch(migration_sql)
        .context("Failed to run migrations")?;

    Ok(())
}

/// Calculate distance between two points using Haversine formula
pub fn haversine_distance(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    const EARTH_RADIUS_M: f64 = 6371000.0; // Earth's radius in meters

    let lat1_rad = lat1.to_radians();
    let lat2_rad = lat2.to_radians();
    let delta_lat = (lat2 - lat1).to_radians();
    let delta_lon = (lon2 - lon1).to_radians();

    let a = (delta_lat / 2.0).sin().powi(2)
        + lat1_rad.cos() * lat2_rad.cos() * (delta_lon / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

    EARTH_RADIUS_M * c
}

/// Calculate total distance for a trajectory given waypoints
pub fn calculate_trajectory_distance(waypoints: &[(f64, f64)]) -> f64 {
    if waypoints.len() < 2 {
        return 0.0;
    }

    waypoints
        .windows(2)
        .map(|w| haversine_distance(w[0].0, w[0].1, w[1].0, w[1].1))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_haversine_distance() {
        // Distance from San Francisco to Los Angeles (approx 559 km)
        let sf_lat = 37.7749;
        let sf_lon = -122.4194;
        let la_lat = 34.0522;
        let la_lon = -118.2437;

        let distance = haversine_distance(sf_lat, sf_lon, la_lat, la_lon);

        // Should be approximately 559,000 meters (within 10km tolerance)
        assert!((distance - 559000.0).abs() < 10000.0);
    }

    #[test]
    fn test_trajectory_distance() {
        let waypoints = vec![
            (37.7749, -122.4194),  // San Francisco
            (36.7783, -119.4179),  // Fresno
            (34.0522, -118.2437),  // Los Angeles
        ];

        let distance = calculate_trajectory_distance(&waypoints);

        // Total distance should be positive
        assert!(distance > 0.0);
    }
}
