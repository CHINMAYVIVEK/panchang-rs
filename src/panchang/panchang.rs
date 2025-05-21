//! Panchanga Calculator - A Rust implementation for Hindu Astrological Calendar calculations
//!
//! This module implements calculations for the five main elements (Pancha-anga) of the Hindu calendar:
//! - Tithi (Lunar Day)
//! - Nakshatra (Lunar Mansion)
//! - Yoga (Luni-Solar Day)
//! - Karana (Half Lunar Day)
//! - Rashi (Zodiac Sign)
//!
//! The calculations are based on astronomical algorithms for computing planetary positions
//! and traditional Hindu astrological principles.

use std::f64::consts::PI;

/// Conversion constant from degrees to radians
const D2R: f64 = PI / 180.0;
/// Conversion constant from radians to degrees
const R2D: f64 = 180.0 / PI;

/// Represents the five main elements of Panchanga (Hindu Calendar)
#[derive(Debug)]
pub struct Panchanga {
    /// Current day (not used in current implementation)
    // current_day: String,
    /// Current Yoga (Luni-Solar Day)
    pub current_yoga: String,
    /// Current Nakshatra (Lunar Mansion)
    pub current_nakshatra: String,
    /// Current Tithi (Lunar Day)
    pub current_tithi: String,
    /// Current Karana (Half Lunar Day)
    pub current_karana: String,
    /// Current Paksha (Lunar Phase - Shukla/Krishna)
    pub current_paksha: String,
    /// Current Rashi (Zodiac Sign)
    pub current_rashi: String,
}

/// Standard Gregorian calendar months (not used in current implementation)
// static MONTHS: [&str; 12] = [
//     "January", "February", "March", "April", "May", "June",
//     "July", "August", "September", "October", "November", "December"
// ];

/// The 12 Rashis (zodiac signs) in Hindu astrology
/// Each Rashi corresponds to a 30° arc of the ecliptic
static RASHI: [&str; 12] = [
    "Mesha",
    "Vrishabha",
    "Mithuna",
    "Karka",
    "Simha",
    "Kanya",
    "Tula",
    "Vrischika",
    "Dhanu",
    "Makara",
    "Kumbha",
    "Meena",
];

/// The 30 Tithis (lunar days) in Hindu calendar
/// Each Tithi is defined as 12° of angular distance between the Sun and Moon
/// The first 15 Tithis are in Shukla Paksha (waxing phase)
/// The next 15 Tithis are in Krishna Paksha (waning phase)
static TITHI: [&str; 30] = [
    "Prathame",
    "Dwithiya",
    "Thrithiya",
    "Chathurthi",
    "Panchami",
    "Shrashti",
    "Saptami",
    "Ashtami",
    "Navami",
    "Dashami",
    "Ekadashi",
    "Dwadashi",
    "Thrayodashi",
    "Chaturdashi",
    "Poornima",
    "Prathame",
    "Dwithiya",
    "Thrithiya",
    "Chathurthi",
    "Panchami",
    "Shrashti",
    "Saptami",
    "Ashtami",
    "Navami",
    "Dashami",
    "Ekadashi",
    "Dwadashi",
    "Thrayodashi",
    "Chaturdashi",
    "Amavasya",
];

/// The 11 Karanas (half lunar days) in Hindu astrology
/// Each Karana is half of a Tithi (6° of angular distance between Sun and Moon)
/// The first 7 Karanas (Bava to Visti) repeat 8 times in a lunar month
/// The last 4 Karanas occur only once per lunar month
static KARAN: [&str; 11] = [
    "Bava",
    "Balava",
    "Kaulava",
    "Taitula",
    "Garija",
    "Vanija",
    "Visti",
    "Sakuni",
    "Chatuspada",
    "Naga",
    "Kimstughna",
];

/// The 27 Yogas in Hindu astrology
/// Yoga is calculated by dividing the sum of lunar and solar longitudes by 13°20'
/// Each Yoga represents specific energy combinations and their effects
static YOGA: [&str; 27] = [
    "Vishkambha",
    "Prithi",
    "Ayushman",
    "Saubhagya",
    "Shobhana",
    "Atiganda",
    "Sukarman",
    "Dhrithi",
    "Shoola",
    "Ganda",
    "Vridhi",
    "Dhruva",
    "Vyaghata",
    "Harshana",
    "Vajra",
    "Siddhi",
    "Vyatipata",
    "Variyan",
    "Parigha",
    "Shiva",
    "Siddha",
    "Sadhya",
    "Shubha",
    "Shukla",
    "Bramha",
    "Indra",
    "Vaidhruthi",
];

/// The 27 Nakshatras (lunar mansions) in Hindu astrology
/// Each Nakshatra spans 13°20' of the ecliptic
/// The Moon's position in a Nakshatra determines the lunar mansion for that time
static NAKSHATRA: [&str; 27] = [
    "Ashwini",
    "Bharani",
    "Krittika",
    "Rohini",
    "Mrigashira",
    "Ardhra",
    "Punarvasu",
    "Pushya",
    "Ashlesa",
    "Magha",
    "Poorva Phalguni",
    "Uttara Phalguni",
    "Hasta",
    "Chitra",
    "Swathi",
    "Vishaka",
    "Anuradha",
    "Jyeshta",
    "Mula",
    "Poorva Ashada",
    "Uttara Ashada",
    "Sravana",
    "Dhanishta",
    "Shatabisha",
    "Poorva Bhadra",
    "Uttara Bhadra",
    "Revathi",
];

// Global variables for storing intermediate calculations
/// Stores the Sun's mean longitude
static mut SUN_MEAN_LONGITUDE: f64 = 0.0;
/// Stores the Moon's mean longitude
static mut MOON_MEAN_LONGITUDE: f64 = 0.0;
/// Stores the Sun's mean anomaly
static mut SUN_MEAN_ANOMALY: f64 = 0.0;
/// Stores the Moon's mean anomaly
static mut MOON_MEAN_ANOMALY: f64 = 0.0;

/// Normalizes an angle to the range [0, 360) degrees
///
/// # Arguments
/// * `x` - The angle to normalize in degrees
///
/// # Returns
/// The normalized angle in degrees
fn rev(x: f64) -> f64 {
    x - (x / 360.0).floor() * 360.0
}

/// Calculates the Ayanamsa (precession of equinoxes) using Lahiri's method
///
/// The Ayanamsa is the angular difference between the Tropical and Sidereal zodiacs.
/// This implementation uses Lahiri's method, which is officially used in Indian ephemeris.
///
/// # Arguments
/// * `d` - Number of days since J2000.0 (January 1, 2000 12:00 UT)
///
/// # Returns
/// The Ayanamsa value in degrees
fn calc_ayanamsa(d: f64) -> f64 {
    // Convert to Julian centuries since J2000.0
    let t = (d + 36523.5) / 36525.0;

    // Calculate the longitude of the Moon's ascending node
    let o = 259.183275 - 1934.142008333206 * t + 0.0020777778 * t * t;

    // Calculate the mean longitude of the Sun
    let l = 279.696678 + 36000.76892 * t + 0.0003025 * t * t;

    // Calculate Ayanamsa using Lahiri's formula
    let mut ayan =
        17.23 * (o * D2R).sin() + 1.27 * (l * 2.0 * D2R).sin() - (5025.64 + 1.11 * t) * t;
    ayan = (ayan - 80861.27) / 3600.0; // Convert to degrees
    ayan
}

/// Calculates the Sun's true geocentric longitude
///
/// This function implements a simplified VSOP87 algorithm to calculate
/// the Sun's position in the ecliptic coordinate system.
///
/// # Arguments
/// * `d` - Number of days since J2000.0 (January 1, 2000 12:00 UT)
///
/// # Returns
/// The Sun's true geocentric longitude in degrees
fn sun_long(d: f64) -> f64 {
    // Calculate the Sun's mean orbital elements
    let perihelion_longitude = 282.9404 + 4.70935e-5 * d; // Argument of perihelion
    // let semi_major_axis = 1.000000;  // Semi-major axis (in AU)
    let orbital_eccentricity = 0.016709 - 1.151e-9 * d; // Eccentricity
    let mean_anomaly = rev(356.0470 + 0.9856002585 * d); // Mean anomaly

    unsafe {
        SUN_MEAN_ANOMALY = mean_anomaly; // Store mean anomaly for later use
        SUN_MEAN_LONGITUDE = perihelion_longitude + mean_anomaly; // Store mean longitude
    }

    // Solve Kepler's equation iteratively
    let mean_anomaly_radians = mean_anomaly * D2R;
    let eccentric_anomaly = mean_anomaly
        + R2D
            * orbital_eccentricity
            * mean_anomaly_radians.sin()
            * (1.0 + orbital_eccentricity * mean_anomaly_radians.cos());

    // Convert to rectangular coordinates
    let eccentric_anomaly_radians = eccentric_anomaly * D2R;
    let x_coord = eccentric_anomaly_radians.cos() - orbital_eccentricity;
    let y_coord = eccentric_anomaly_radians.sin()
        * (1.0 - orbital_eccentricity * orbital_eccentricity).sqrt();

    // Calculate true anomaly and return true longitude
    let true_anomaly = rev(R2D * y_coord.atan2(x_coord));
    rev(true_anomaly + perihelion_longitude)
}

/// Calculates the Moon's true geocentric longitude
///
/// This function implements a simplified ELP2000 algorithm for lunar position calculation.
/// It accounts for various periodic perturbations in the Moon's orbit.
///
/// # Arguments
/// * `d` - Number of days since J2000.0 (January 1, 2000 12:00 UT)
///
/// # Returns
/// The Moon's true geocentric longitude in degrees
fn moon_long(d: f64) -> f64 {
    // Calculate the Moon's mean orbital elements
    let ascending_node_longitude = 125.1228 - 0.0529538083 * d; // Longitude of ascending node
    let orbital_inclination = 5.1454; // Inclination to ecliptic
    let perigee_argument = rev(318.0634 + 0.1643573223 * d); // Argument of perigee
    let semi_major_axis = 60.2666; // Semi-major axis (Earth radii)
    let orbital_eccentricity = 0.054900; // Eccentricity
    let mean_anomaly = rev(115.3654 + 13.0649929509 * d); // Mean anomaly

    unsafe {
        MOON_MEAN_ANOMALY = mean_anomaly; // Store mean anomaly for later use
        MOON_MEAN_LONGITUDE = ascending_node_longitude + perigee_argument + mean_anomaly; // Store mean longitude
    }

    // Solve Kepler's equation iteratively for eccentric anomaly
    let mut anomaly_radians = mean_anomaly * D2R;
    let mut eccentric_anomaly = mean_anomaly
        + R2D
            * orbital_eccentricity
            * anomaly_radians.sin()
            * (1.0 + orbital_eccentricity * anomaly_radians.cos());
    let mut eccentric_correction;

    loop {
        anomaly_radians = eccentric_anomaly * D2R;
        eccentric_correction = eccentric_anomaly
            - (eccentric_anomaly
                - R2D * orbital_eccentricity * anomaly_radians.sin()
                - mean_anomaly)
                / (1.0 - orbital_eccentricity * anomaly_radians.cos());
        if (eccentric_anomaly - eccentric_correction).abs() <= 0.005 {
            break;
        }
        eccentric_anomaly = eccentric_correction;
    }

    // Convert to rectangular coordinates in the orbital plane
    anomaly_radians = eccentric_anomaly * D2R;
    let orbital_x = semi_major_axis * (anomaly_radians.cos() - orbital_eccentricity);
    let orbital_y = semi_major_axis
        * (1.0 - orbital_eccentricity * orbital_eccentricity).sqrt()
        * anomaly_radians.sin();

    let orbital_radius = (orbital_x * orbital_x + orbital_y * orbital_y).sqrt(); // Distance to Moon
    let true_anomaly = rev(R2D * orbital_y.atan2(orbital_x)); // True anomaly

    // Convert to ecliptic coordinates
    let node_radians = ascending_node_longitude * D2R;
    let argument_radians = (true_anomaly + perigee_argument) * D2R;
    let inclination_radians = orbital_inclination * D2R;

    let ecliptic_x = orbital_radius
        * (node_radians.cos() * argument_radians.cos()
            - node_radians.sin() * argument_radians.sin() * inclination_radians.cos());
    let ecliptic_y = orbital_radius
        * (node_radians.sin() * argument_radians.cos()
            + node_radians.cos() * argument_radians.sin() * inclination_radians.cos());
    // let ecliptic_z = orbital_radius * argument_radians.sin() * inclination_radians.sin();

    unsafe {
        let mean_elongation = MOON_MEAN_LONGITUDE - SUN_MEAN_LONGITUDE; // Mean elongation
        let argument_of_latitude = MOON_MEAN_LONGITUDE - ascending_node_longitude; // Argument of latitude

        // Calculate longitude with periodic perturbations
        let mut ecliptic_longitude = R2D * ecliptic_y.atan2(ecliptic_x);

        // Apply major periodic perturbations
        ecliptic_longitude += -1.274 * ((MOON_MEAN_ANOMALY - 2.0 * mean_elongation) * D2R).sin(); // Evection
        ecliptic_longitude += 0.658 * ((2.0 * mean_elongation) * D2R).sin(); // Variation
        ecliptic_longitude += -0.186 * (SUN_MEAN_ANOMALY * D2R).sin(); // Yearly equation
        ecliptic_longitude +=
            -0.059 * ((2.0 * MOON_MEAN_ANOMALY - 2.0 * mean_elongation) * D2R).sin();
        ecliptic_longitude +=
            -0.057 * ((MOON_MEAN_ANOMALY - 2.0 * mean_elongation + SUN_MEAN_ANOMALY) * D2R).sin();
        ecliptic_longitude += 0.053 * ((MOON_MEAN_ANOMALY + 2.0 * mean_elongation) * D2R).sin();
        ecliptic_longitude += 0.046 * ((2.0 * mean_elongation - SUN_MEAN_ANOMALY) * D2R).sin();
        ecliptic_longitude += 0.041 * ((MOON_MEAN_ANOMALY - SUN_MEAN_ANOMALY) * D2R).sin();
        ecliptic_longitude += -0.035 * (mean_elongation * D2R).sin();
        ecliptic_longitude += -0.031 * ((MOON_MEAN_ANOMALY + SUN_MEAN_ANOMALY) * D2R).sin();
        ecliptic_longitude +=
            -0.015 * ((2.0 * argument_of_latitude - 2.0 * mean_elongation) * D2R).sin();
        ecliptic_longitude += 0.011 * ((MOON_MEAN_ANOMALY - 4.0 * mean_elongation) * D2R).sin();

        rev(ecliptic_longitude)
    }
}

/// Calculates all elements of Panchanga (Hindu astrological calendar)
///
/// This function computes the five main elements of Panchanga:
/// - Tithi (lunar day)
/// - Nakshatra (lunar mansion)
/// - Yoga (luni-solar day)
/// - Karana (half lunar day)
/// - Rashi (zodiac sign)
///
/// # Arguments
/// * `dd` - Day of month
/// * `mm` - Month number (1-12)
/// * `yy` - Year
/// * `hr` - Hour in local time
/// * `zhr` - Time zone offset from GMT in hours
///
/// # Returns
/// A Panchanga struct containing all calculated elements
pub fn calculate_panchanga(
    day: i32,
    month: i32,
    year: i32,
    hour: f64,
    timezone_offset: f64,
) -> Panchanga {
    let mut panchanga_data = Panchanga {
        // current_day: String::new(),
        current_yoga: String::new(),
        current_nakshatra: String::new(),
        current_tithi: String::new(),
        current_karana: String::new(),
        current_paksha: String::new(),
        current_rashi: String::new(),
    };

    // Calculate Julian Day number relative to J2000.0
    let days_since_j2000 =
        (367 * year - 7 * (year + (month + 9) / 12) / 4 + 275 * month / 9 + day - 730530) as f64;

    // Calculate basic astronomical values
    let ayanamsa = calc_ayanamsa(days_since_j2000);
    let sun_longitude = sun_long(days_since_j2000 + ((hour - timezone_offset) / 24.0));
    let moon_longitude = moon_long(days_since_j2000 + ((hour - timezone_offset) / 24.0));

    // Calculate Tithi (lunar day)
    let mut adjusted_moon_longitude = moon_longitude
        + if moon_longitude < sun_longitude {
            360.0
        } else {
            0.0
        };
    let adjusted_sun_longitude = sun_longitude;
    let mut tithi_index = ((adjusted_moon_longitude - adjusted_sun_longitude) / 12.0) as usize; // Each Tithi = 12 degrees

    panchanga_data.current_tithi = TITHI[tithi_index].to_string();
    panchanga_data.current_paksha = if tithi_index <= 14 {
        "Shukla"
    } else {
        "Krishna"
    }
    .to_string();

    // Calculate Nakshatra (lunar mansion)
    adjusted_moon_longitude = rev(moon_longitude + ayanamsa);
    panchanga_data.current_nakshatra =
        NAKSHATRA[(adjusted_moon_longitude * 6.0 / 80.0) as usize].to_string(); // Each Nakshatra = 13°20'

    // Calculate Yoga (luni-solar day)
    adjusted_moon_longitude = moon_longitude + ayanamsa;
    let adjusted_sun_longitude = sun_longitude + ayanamsa;
    panchanga_data.current_yoga = YOGA
        [(rev(adjusted_moon_longitude + adjusted_sun_longitude) * 6.0 / 80.0) as usize]
        .to_string(); // Each Yoga = 13°20'

    // Calculate Karana (half lunar day)
    adjusted_moon_longitude = moon_longitude
        + if moon_longitude < sun_longitude {
            360.0
        } else {
            0.0
        };
    let adjusted_sun_longitude = sun_longitude;
    tithi_index = ((adjusted_moon_longitude - adjusted_sun_longitude) / 6.0) as usize; // Each Karana = 6 degrees

    // Apply special rules for Karana calculation
    if tithi_index == 0 {
        tithi_index = 10;
    }
    if tithi_index >= 57 {
        tithi_index -= 50;
    }
    if tithi_index > 0 && tithi_index < 57 {
        tithi_index = (tithi_index - 1) - ((tithi_index - 1) / 7 * 7);
    }
    panchanga_data.current_karana = KARAN[tithi_index].to_string();

    // Calculate Rashi (zodiac sign)
    adjusted_moon_longitude = rev(moon_longitude + ayanamsa);
    panchanga_data.current_rashi = RASHI[(adjusted_moon_longitude / 30.0) as usize].to_string(); // Each Rashi = 30 degrees

    panchanga_data
}

pub fn parse_time(time_str: &str) -> Result<(f64, i32), &'static str> {
    let parts: Vec<&str> = time_str.split(':').collect();
    if parts.len() != 2 {
        return Err("Invalid time format");
    }

    let hours: f64 = parts[0].parse().map_err(|_| "Invalid hours")?;
    let minutes: i32 = parts[1].parse().map_err(|_| "Invalid minutes")?;

    Ok((hours, minutes))
}

pub fn parse_date(date_str: &str) -> Result<(f64, i32, i32), &'static str> {
    let parts: Vec<&str> = date_str.split('/').collect();
    if parts.len() != 3 {
        return Err("Invalid date format");
    }

    let day: f64 = parts[0].parse().map_err(|_| "Invalid day")?;
    let month: i32 = parts[1].parse().map_err(|_| "Invalid month")?;
    let year: i32 = parts[2].parse().map_err(|_| "Invalid year")?;

    Ok((day, month, year))
}
