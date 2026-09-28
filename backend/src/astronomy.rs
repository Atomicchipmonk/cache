use chrono::{DateTime, Datelike, Timelike, Utc};
use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

/// Solar calculation results for a given location and time
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolarTimes {
    /// Sunrise time (when upper edge of sun appears on horizon)
    pub sunrise: DateTime<Utc>,
    /// Sunset time (when upper edge of sun disappears below horizon)
    pub sunset: DateTime<Utc>,
    /// Solar noon (sun at highest point)
    pub solar_noon: DateTime<Utc>,
    /// Civil twilight start (sun 6° below horizon)
    pub civil_dawn: DateTime<Utc>,
    /// Civil twilight end (sun 6° below horizon)
    pub civil_dusk: DateTime<Utc>,
    /// Nautical twilight start (sun 12° below horizon)
    pub nautical_dawn: DateTime<Utc>,
    /// Nautical twilight end (sun 12° below horizon)
    pub nautical_dusk: DateTime<Utc>,
    /// Astronomical twilight start (sun 18° below horizon)
    pub astronomical_dawn: DateTime<Utc>,
    /// Astronomical twilight end (sun 18° below horizon)
    pub astronomical_dusk: DateTime<Utc>,
}

/// Lunar calculation results for a given location and time
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LunarTimes {
    /// Moonrise time (approximate)
    pub moonrise: Option<DateTime<Utc>>,
    /// Moonset time (approximate)
    pub moonset: Option<DateTime<Utc>>,
    /// Moon phase (0.0 = new moon, 0.5 = full moon, 1.0 = new moon)
    pub phase: f64,
    /// Moon illumination fraction (0.0 to 1.0)
    pub illumination: f64,
    /// Moon phase name
    pub phase_name: String,
    /// Moon age in days
    pub age_days: f64,
}

/// Moon phase names
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MoonPhase {
    NewMoon,
    WaxingCrescent,
    FirstQuarter,
    WaxingGibbous,
    FullMoon,
    WaningGibbous,
    LastQuarter,
    WaningCrescent,
}

impl MoonPhase {
    /// Get moon phase from phase value (0.0 to 1.0)
    pub fn from_phase_value(phase: f64) -> Self {
        match phase {
            p if p < 0.033 => MoonPhase::NewMoon,
            p if p < 0.216 => MoonPhase::WaxingCrescent,
            p if p < 0.283 => MoonPhase::FirstQuarter,
            p if p < 0.466 => MoonPhase::WaxingGibbous,
            p if p < 0.533 => MoonPhase::FullMoon,
            p if p < 0.716 => MoonPhase::WaningGibbous,
            p if p < 0.783 => MoonPhase::LastQuarter,
            p if p < 0.967 => MoonPhase::WaningCrescent,
            _ => MoonPhase::NewMoon,
        }
    }

    /// Get human-readable name
    pub fn name(&self) -> &'static str {
        match self {
            MoonPhase::NewMoon => "New Moon",
            MoonPhase::WaxingCrescent => "Waxing Crescent",
            MoonPhase::FirstQuarter => "First Quarter",
            MoonPhase::WaxingGibbous => "Waxing Gibbous",
            MoonPhase::FullMoon => "Full Moon",
            MoonPhase::WaningGibbous => "Waning Gibbous",
            MoonPhase::LastQuarter => "Last Quarter",
            MoonPhase::WaningCrescent => "Waning Crescent",
        }
    }
}

/// Convert DateTime to Julian Day Number
fn to_julian_day(dt: DateTime<Utc>) -> f64 {
    let year = dt.year();
    let month = dt.month() as i32;
    let day = dt.day() as f64;
    let hour = dt.hour() as f64;
    let minute = dt.minute() as f64;
    let second = dt.second() as f64;

    let (y, m) = if month <= 2 {
        (year - 1, month + 12)
    } else {
        (year, month)
    };

    let a = (y / 100) as f64;
    let b = 2.0 - a + (a / 4.0).floor();

    let decimal_day = day + (hour + minute / 60.0 + second / 3600.0) / 24.0;

    (365.25 * (y + 4716) as f64).floor()
        + (30.6001 * (m + 1) as f64).floor()
        + decimal_day + b - 1524.5
}

/// Calculate solar noon and sunrise/sunset for a given altitude
fn calculate_solar_times_for_altitude(jd: f64, lat: f64, lon: f64, altitude: f64) -> (f64, f64, f64) {
    let n = (jd - 2451545.0 - 0.0009 - lon / 360.0).round();
    let j_star = 2451545.0 + 0.0009 + lon / 360.0 + n;
    let m = (357.5291 + 0.98560028 * (j_star - 2451545.0)) % 360.0;
    let m_rad = m.to_radians();

    let c = 1.9148 * m_rad.sin() + 0.0200 * (2.0 * m_rad).sin() + 0.0003 * (3.0 * m_rad).sin();
    let lambda = (m + c + 180.0 + 102.9372) % 360.0;
    let lambda_rad = lambda.to_radians();

    let j_transit = j_star + 0.0053 * m_rad.sin() - 0.0069 * (2.0 * lambda_rad).sin();

    let delta = (lambda_rad.sin() * 23.4397f64.to_radians().sin()).asin();
    let lat_rad = lat.to_radians();

    let cos_omega = (altitude.to_radians().sin() - lat_rad.sin() * delta.sin())
        / (lat_rad.cos() * delta.cos());

    if cos_omega.abs() > 1.0 {
        // Sun doesn't rise or set
        return (j_transit, f64::NAN, f64::NAN);
    }

    let omega = cos_omega.acos().to_degrees();
    let j_rise = j_transit - omega / 360.0;
    let j_set = j_transit + omega / 360.0;

    (j_transit, j_rise, j_set)
}

/// Convert Julian Day to DateTime<Utc>
fn from_julian_day(jd: f64) -> DateTime<Utc> {
    let z = (jd + 0.5).floor() as i32;
    let f = (jd + 0.5) - z as f64;

    let alpha = ((z as f64 - 1867216.25) / 36524.25).floor() as i32;
    let a = z + 1 + alpha - (alpha / 4);
    let b = a + 1524;
    let c = ((b as f64 - 122.1) / 365.25).floor() as i32;
    let d = (365.25 * c as f64).floor() as i32;
    let e = ((b - d) as f64 / 30.6001).floor() as i32;

    let day = (b - d - (30.6001 * e as f64).floor() as i32) as u32;
    let month = if e < 14 { e - 1 } else { e - 13 } as u32;
    let year = if month > 2 { c - 4716 } else { c - 4715 };

    let hours = f * 24.0;
    let hour = hours.floor() as u32;
    let minutes = (hours - hour as f64) * 60.0;
    let minute = minutes.floor() as u32;
    let second = ((minutes - minute as f64) * 60.0).floor() as u32;

    DateTime::from_timestamp(
        chrono::NaiveDate::from_ymd_opt(year, month, day)
            .and_then(|d| d.and_hms_opt(hour, minute, second))
            .map(|dt| dt.and_utc())
            .unwrap_or(Utc::now())
            .timestamp(),
        0
    ).unwrap_or(Utc::now())
}

/// Calculate solar times for a given location and date
pub fn calculate_solar_times(lat: f64, lon: f64, date: DateTime<Utc>) -> SolarTimes {
    let jd = to_julian_day(date);

    // Standard sunrise/sunset (center of sun at horizon, accounting for refraction)
    let (noon_jd, rise_jd, set_jd) = calculate_solar_times_for_altitude(jd, lat, lon, -0.833);

    // Civil twilight (sun 6° below horizon)
    let (_, civil_rise_jd, civil_set_jd) = calculate_solar_times_for_altitude(jd, lat, lon, -6.0);

    // Nautical twilight (sun 12° below horizon)
    let (_, naut_rise_jd, naut_set_jd) = calculate_solar_times_for_altitude(jd, lat, lon, -12.0);

    // Astronomical twilight (sun 18° below horizon)
    let (_, astro_rise_jd, astro_set_jd) = calculate_solar_times_for_altitude(jd, lat, lon, -18.0);

    SolarTimes {
        sunrise: from_julian_day(rise_jd),
        sunset: from_julian_day(set_jd),
        solar_noon: from_julian_day(noon_jd),
        civil_dawn: from_julian_day(civil_rise_jd),
        civil_dusk: from_julian_day(civil_set_jd),
        nautical_dawn: from_julian_day(naut_rise_jd),
        nautical_dusk: from_julian_day(naut_set_jd),
        astronomical_dawn: from_julian_day(astro_rise_jd),
        astronomical_dusk: from_julian_day(astro_set_jd),
    }
}

/// Calculate moon phase and illumination
pub fn calculate_lunar_times(_lat: f64, _lon: f64, date: DateTime<Utc>) -> LunarTimes {
    let jd = to_julian_day(date);

    // Moon phase calculation (simplified)
    // Based on the moon's synodic period of 29.53 days
    let days_since_new_moon = (jd - 2451550.1) % 29.53058770576;
    let phase = days_since_new_moon / 29.53058770576;

    // Calculate illumination (approximate)
    let illumination = (1.0 - (2.0 * PI * phase).cos()) / 2.0;

    let moon_phase = MoonPhase::from_phase_value(phase);

    // Note: Moonrise/moonset calculations are more complex and would require
    // implementing the full lunar position algorithm. For now, returning None.
    // This can be enhanced later with proper lunar position calculations.

    LunarTimes {
        moonrise: None,
        moonset: None,
        phase,
        illumination,
        phase_name: moon_phase.name().to_string(),
        age_days: days_since_new_moon,
    }
}

/// Get current solar position (altitude and azimuth)
pub fn get_solar_position(lat: f64, lon: f64, date: DateTime<Utc>) -> (f64, f64) {
    let jd = to_julian_day(date);
    let n = jd - 2451545.0;

    // Mean longitude
    let l = (280.460 + 0.9856474 * n) % 360.0;

    // Mean anomaly
    let g = ((357.528 + 0.9856003 * n) % 360.0).to_radians();

    // Ecliptic longitude
    let lambda = (l + 1.915 * g.sin() + 0.020 * (2.0 * g).sin()).to_radians();

    // Obliquity
    let epsilon = (23.439 - 0.0000004 * n).to_radians();

    // Right ascension and declination
    let ra = (lambda.cos().atan2(epsilon.cos() * lambda.sin())).to_degrees();
    let dec = (epsilon.sin() * lambda.sin()).asin();

    // Sidereal time at Greenwich
    let gmst = (280.46061837 + 360.98564736629 * n + lon) % 360.0;

    // Hour angle
    let h = (gmst - ra).to_radians();
    let lat_rad = lat.to_radians();

    // Altitude and azimuth
    let altitude = (lat_rad.sin() * dec.sin() + lat_rad.cos() * dec.cos() * h.cos()).asin();
    let azimuth = (dec.sin() * lat_rad.cos() - dec.cos() * h.cos() * lat_rad.sin())
        .atan2(dec.cos() * h.sin());

    (altitude.to_degrees(), azimuth.to_degrees() + 180.0)
}

/// Get current lunar position (altitude and azimuth)
pub fn get_lunar_position(lat: f64, lon: f64, date: DateTime<Utc>) -> (f64, f64, f64) {
    let jd = to_julian_day(date);
    let d = jd - 2451545.0;

    // Simplified lunar position (low precision)
    let l = (218.316 + 13.176396 * d) % 360.0;
    let m = ((134.963 + 13.064993 * d) % 360.0).to_radians();
    let f = ((93.272 + 13.229350 * d) % 360.0).to_radians();

    let lambda = (l + 6.289 * m.sin()).to_radians();
    let beta = (5.128 * f.sin()).to_radians();

    // Distance in km (approximate)
    let distance = 385001.0 - 20905.0 * m.cos();

    // Ecliptic to equatorial coordinates
    let epsilon = (23.439 - 0.0000004 * d).to_radians();

    let ra = (beta.cos() * lambda.cos().atan2(
        epsilon.cos() * beta.cos() * lambda.sin() - epsilon.sin() * beta.sin()
    )).to_degrees();

    let dec = (epsilon.sin() * beta.cos() * lambda.sin() + epsilon.cos() * beta.sin()).asin();

    // Sidereal time and hour angle
    let gmst = (280.46061837 + 360.98564736629 * d + lon) % 360.0;
    let h = (gmst - ra).to_radians();
    let lat_rad = lat.to_radians();

    // Altitude and azimuth
    let altitude = (lat_rad.sin() * dec.sin() + lat_rad.cos() * dec.cos() * h.cos()).asin();
    let azimuth = (dec.sin() * lat_rad.cos() - dec.cos() * h.cos() * lat_rad.sin())
        .atan2(dec.cos() * h.sin());

    (altitude.to_degrees(), azimuth.to_degrees() + 180.0, distance)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_julian_day() {
        // J2000.0 epoch: Jan 1, 2000, 12:00 UTC = JD 2451545.0
        let date = Utc.with_ymd_and_hms(2000, 1, 1, 12, 0, 0).unwrap();
        let jd = to_julian_day(date);
        assert!((jd - 2451545.0).abs() < 0.1);
    }

    #[test]
    fn test_solar_times() {
        let lat = 37.7749;
        let lon = -122.4194;
        let date = Utc.with_ymd_and_hms(2026, 6, 21, 12, 0, 0).unwrap();

        let solar = calculate_solar_times(lat, lon, date);

        // Sunrise should be before sunset
        assert!(solar.sunrise < solar.sunset);
        // Solar noon should be between sunrise and sunset
        assert!(solar.sunrise < solar.solar_noon);
        assert!(solar.solar_noon < solar.sunset);
    }

    #[test]
    fn test_lunar_phase() {
        let lat = 37.7749;
        let lon = -122.4194;
        let date = Utc.with_ymd_and_hms(2026, 9, 28, 15, 0, 0).unwrap();

        let lunar = calculate_lunar_times(lat, lon, date);

        // Phase should be between 0 and 1
        assert!(lunar.phase >= 0.0 && lunar.phase <= 1.0);
        // Illumination should be between 0 and 1
        assert!(lunar.illumination >= 0.0 && lunar.illumination <= 1.0);
        assert!(!lunar.phase_name.is_empty());
    }

    #[test]
    fn test_moon_phase_names() {
        assert_eq!(MoonPhase::from_phase_value(0.0).name(), "New Moon");
        assert_eq!(MoonPhase::from_phase_value(0.25).name(), "First Quarter");
        assert_eq!(MoonPhase::from_phase_value(0.5).name(), "Full Moon");
        assert_eq!(MoonPhase::from_phase_value(0.75).name(), "Last Quarter");
    }
}
