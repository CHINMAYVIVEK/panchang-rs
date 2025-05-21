use actix_web::{web};
use serde::{Deserialize, Serialize};
use crate::panchang::{parse_date,parse_time, calculate_panchanga};

#[derive(Debug, Deserialize)]
pub struct PanchangRequest {
    /// Date in DD/MM/YYYY format
    date: String,
    /// Time in HH:MM 24-hour format
    time: String,
    /// Timezone offset from GMT in [+/-]HH:MM format
    zone: String,
}

#[derive(Debug, Serialize)]
pub struct PanchangResponse {
    /// Current Tithi (Lunar Day)
    tithi: String,
    /// Current Paksha (Lunar Phase)
    paksha: String,
    /// Current Nakshatra (Lunar Mansion)
    nakshatra: String,
    /// Current Yoga (Luni-Solar Day)
    yoga: String,
    /// Current Karana (Half Lunar Day)
    karana: String,
    /// Current Rashi (Zodiac Sign)
    rashi: String,
}

pub async fn calculate_panchang(data: web::Json<PanchangRequest>) -> Result<PanchangResponse, String> {
    // Parse date
    let (day, month, year) = parse_date(&data.date)
        .map_err(|e| format!("Error parsing date: {}", e))?;

    // Parse time
    let (hours, minutes) = parse_time(&data.time)
        .map_err(|e| format!("Error parsing time: {}", e))?;

    // Parse timezone
    let (zone_hours, zone_minutes) = parse_time(&data.zone.trim_start_matches('+'))
        .map_err(|e| format!("Error parsing timezone: {}", e))?;

    // Convert to decimal hours
    let hour = hours + minutes as f64 / 60.0;
    let zone_hour = if data.zone.starts_with('-') {
        -(zone_hours + zone_minutes as f64 / 60.0)
    } else {
        zone_hours + zone_minutes as f64 / 60.0
    };

    // Calculate panchanga
    let panchang_data = calculate_panchanga(day as i32, month, year, hour, zone_hour);

    // Prepare response
    Ok(PanchangResponse {
        tithi: panchang_data.current_tithi,
        paksha: panchang_data.current_paksha,
        nakshatra: panchang_data.current_nakshatra,
        yoga: panchang_data.current_yoga,
        karana: panchang_data.current_karana,
        rashi: panchang_data.current_rashi,
    })
}