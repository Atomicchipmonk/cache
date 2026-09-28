use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ============================================================================
// Waypoint
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Waypoint {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub lat: f64,
    pub lon: f64,
    pub altitude: Option<f64>,
    pub category: Option<String>,
    pub icon: Option<String>,
    pub color: Option<String>,
    pub metadata_json: Option<String>,
    pub user_id: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub version: i64,
    pub last_modified_by: Option<String>,
}

impl Waypoint {
    pub fn new(name: String, lat: f64, lon: f64) -> Self {
        let now = Utc::now().to_rfc3339();
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            description: None,
            lat,
            lon,
            altitude: None,
            category: None,
            icon: None,
            color: None,
            metadata_json: None,
            user_id: None,
            created_at: now.clone(),
            updated_at: now,
            version: 1,
            last_modified_by: None,
        }
    }
}

// ============================================================================
// Trajectory
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trajectory {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub waypoint_ids: String,  // JSON array
    pub directionality: String,  // 'one_way' | 'bidirectional' | 'reverse'
    pub total_distance_m: Option<f64>,
    pub metadata_json: Option<String>,
    pub user_id: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub version: i64,
    pub last_modified_by: Option<String>,
}

impl Trajectory {
    pub fn new(name: String, waypoint_ids: Vec<String>) -> Self {
        let now = Utc::now().to_rfc3339();
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            description: None,
            waypoint_ids: serde_json::to_string(&waypoint_ids).unwrap(),
            directionality: "one_way".to_string(),
            total_distance_m: None,
            metadata_json: None,
            user_id: None,
            created_at: now.clone(),
            updated_at: now,
            version: 1,
            last_modified_by: None,
        }
    }
}

// ============================================================================
// Plan
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Plan {
    pub id: String,
    pub trajectory_id: String,
    pub name: String,
    pub description: Option<String>,
    pub start_time: String,  // ISO 8601
    pub estimated_duration_seconds: Option<i64>,
    pub waypoint_times: Option<String>,  // JSON array
    pub pause_durations: Option<String>,  // JSON array
    pub status: String,  // 'draft' | 'active' | 'completed' | 'cancelled'
    pub beacon_id: Option<String>,
    pub metadata_json: Option<String>,
    pub user_id: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub version: i64,
    pub last_modified_by: Option<String>,
}

impl Plan {
    pub fn new(trajectory_id: String, name: String, start_time: DateTime<Utc>) -> Self {
        let now = Utc::now().to_rfc3339();
        Self {
            id: Uuid::new_v4().to_string(),
            trajectory_id,
            name,
            description: None,
            start_time: start_time.to_rfc3339(),
            estimated_duration_seconds: None,
            waypoint_times: None,
            pause_durations: None,
            status: "draft".to_string(),
            beacon_id: None,
            metadata_json: None,
            user_id: None,
            created_at: now.clone(),
            updated_at: now,
            version: 1,
            last_modified_by: None,
        }
    }
}

// ============================================================================
// Beacon
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Beacon {
    pub id: String,
    pub name: String,
    pub r#type: String,  // 'vehicle' | 'person' | 'asset' | 'vessel' | 'aircraft'
    pub icon: Option<String>,
    pub color: Option<String>,
    pub sensors: Option<String>,  // JSON array
    pub active: bool,
    pub last_seen: Option<String>,
    pub metadata_json: Option<String>,
    pub user_id: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub version: i64,
    pub last_modified_by: Option<String>,
}

impl Beacon {
    pub fn new(name: String, beacon_type: String) -> Self {
        let now = Utc::now().to_rfc3339();
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            r#type: beacon_type,
            icon: None,
            color: None,
            sensors: None,
            active: true,
            last_seen: None,
            metadata_json: None,
            user_id: None,
            created_at: now.clone(),
            updated_at: now,
            version: 1,
            last_modified_by: None,
        }
    }
}

// ============================================================================
// Track (GPS position history)
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Track {
    pub id: String,
    pub beacon_id: String,
    pub lat: f64,
    pub lon: f64,
    pub altitude: Option<f64>,
    pub heading: Option<f64>,
    pub speed: Option<f64>,
    pub accuracy: Option<f64>,
    pub source: String,  // 'gps' | 'adsb' | 'manual' | 'estimated'
    pub timestamp: String,  // ISO 8601
    pub metadata_json: Option<String>,
    pub created_at: String,
}

impl Track {
    pub fn new(beacon_id: String, lat: f64, lon: f64, timestamp: DateTime<Utc>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            beacon_id,
            lat,
            lon,
            altitude: None,
            heading: None,
            speed: None,
            accuracy: None,
            source: "gps".to_string(),
            timestamp: timestamp.to_rfc3339(),
            metadata_json: None,
            created_at: Utc::now().to_rfc3339(),
        }
    }
}

// ============================================================================
// Sensor
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sensor {
    pub id: String,
    pub name: String,
    pub r#type: String,  // 'temperature' | 'wind_speed' | etc.
    pub unit: String,
    pub fidelity: String,  // 'forecast' | 'measured'
    pub origin: String,  // 'api' | 'hardware' | 'manual' | 'derived'
    pub source_identifier: Option<String>,
    pub metadata_json: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl Sensor {
    pub fn new(name: String, sensor_type: String, unit: String, fidelity: String, origin: String) -> Self {
        let now = Utc::now().to_rfc3339();
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            r#type: sensor_type,
            unit,
            fidelity,
            origin,
            source_identifier: None,
            metadata_json: None,
            created_at: now.clone(),
            updated_at: now,
        }
    }
}

// ============================================================================
// Reading (sensor measurement)
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reading {
    pub id: String,
    pub sensor_id: String,
    pub lat: f64,
    pub lon: f64,
    pub value: f64,
    pub timestamp: String,  // ISO 8601
    pub forecast_time: Option<String>,  // ISO 8601
    pub quality: Option<f64>,
    pub source_id: Option<String>,
    pub metadata_json: Option<String>,
    pub created_at: String,
}

impl Reading {
    pub fn new(sensor_id: String, lat: f64, lon: f64, value: f64, timestamp: DateTime<Utc>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            sensor_id,
            lat,
            lon,
            value,
            timestamp: timestamp.to_rfc3339(),
            forecast_time: None,
            quality: None,
            source_id: None,
            metadata_json: None,
            created_at: Utc::now().to_rfc3339(),
        }
    }
}

// ============================================================================
// Tile
// ============================================================================

#[derive(Debug, Clone)]
pub struct Tile {
    pub zoom_level: i32,
    pub tile_column: i32,
    pub tile_row: i32,
    pub tile_data: Vec<u8>,
    pub tile_source: String,
    pub last_accessed: String,
    pub created_at: String,
}

impl Tile {
    pub fn new(z: i32, x: i32, y: i32, data: Vec<u8>, source: String) -> Self {
        let now = Utc::now().to_rfc3339();
        Self {
            zoom_level: z,
            tile_column: x,
            tile_row: y,
            tile_data: data,
            tile_source: source,
            last_accessed: now.clone(),
            created_at: now,
        }
    }
}
