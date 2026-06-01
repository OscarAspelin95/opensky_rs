use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[repr(i32)]
pub enum PositionSource {
    AdsB = 0,
    Asterix = 1,
    Mlat = 2,
    Flarm = 3,
}

#[derive(Debug, Clone, Deserialize)]
#[repr(i32)]
pub enum AircraftCategory {
    NoInformation = 0,
    NoAdsb = 1,
    Light = 2,
    Small = 3,
    Large = 4,
    HighVortexLarge = 5,
    Heavy = 6,
    HighPerformance = 7,
    Rotorcraft = 8,
    Glider = 9,
    LighterThanAir = 10,
    Parachutist = 11,
    Ultralight = 12,
    Reserved = 13,
    UnmannedAerialVehicle = 14,
    SpaceVehicle = 15,
    EmergencyVehicle = 16,
    ServiceVehicle = 17,
    PointObstacle = 18,
    ClusterObstacle = 19,
    LineObstacle = 20,
}

// A single aircraft state vector from the OpenSky Network API.
#[derive(Debug, Clone, Deserialize)]
pub struct StateVector {
    /// Unique ICAO 24-bit address of the transponder in hex string representation.
    pub icao24: String,

    /// Callsign of the vehicle (8 chars). None if no callsign has been received.
    pub callsign: Option<String>,

    /// Country name inferred from the ICAO 24-bit address.
    pub origin_country: String,

    /// Unix timestamp (seconds) of the last position update.
    /// None if within the past 15s.
    pub time_position: Option<i64>,

    /// Unix timestamp (seconds) of the last update.
    pub last_contact: i64,

    /// WGS-84 longitude in decimal degrees. None if unavailable.
    pub longitude: Option<f64>,

    /// WGS-84 latitude in decimal degrees. None if unavailable.
    pub latitude: Option<f64>,

    /// Barometric altitude in meters. None if unavailable.
    pub baro_altitude: Option<f64>,

    /// True if the position was retrieved from a surface position report.
    pub on_ground: bool,

    /// Velocity over ground in m/s. None if unavailable.
    pub velocity: Option<f64>,

    /// True track in decimal degrees clockwise from north (north=0°). None if unavailable.
    pub true_track: Option<f64>,

    /// Vertical rate in m/s. Positive = climbing, negative = descending. None if unavailable.
    pub vertical_rate: Option<f64>,

    /// IDs of receivers that contributed to this state vector.
    /// None if no sensor filtering was used in the request.
    pub sensors: Option<Vec<i32>>,

    /// Geometric altitude in meters. None if unavailable.
    pub geo_altitude: Option<f64>,

    /// Transponder code (Squawk). None if unavailable.
    pub squawk: Option<String>,

    /// Whether flight status indicates special purpose indicator.
    pub spi: bool,

    /// Origin of this state's position.
    pub position_source: PositionSource,

    /// Aircraft category.
    pub category: AircraftCategory,
}

type RawStateVector = (
    String,           // 0 icao24
    Option<String>,   // 1 callsign
    String,           // 2 origin_country
    Option<i64>,      // 3 time_position
    i64,              // 4 last_contact
    Option<f64>,      // 5 longitude
    Option<f64>,      // 6 latitude
    Option<f64>,      // 7 baro_altitude
    bool,             // 8 on_ground
    Option<f64>,      // 9 velocity
    Option<f64>,      // 10 true_track
    Option<f64>,      // 11 vertical_rate
    Option<Vec<i32>>, // 12 sensors
    Option<f64>,      // 13 geo_altitude
    Option<String>,   // 14 squawk
    bool,             // 15 spi
    i32,              // 16 position_source
    i32,              // 17 category
);

impl From<RawStateVector> for StateVector {
    fn from(value: RawStateVector) -> Self {
        Self {
            icao24: value.0,
            callsign: value.1,
            origin_country: value.2,
            time_position: value.3,
            last_contact: value.4,
            longitude: value.5,
            latitude: value.6,
            baro_altitude: value.7,
            on_ground: value.8,
            velocity: value.9,
            true_track: value.10,
            vertical_rate: value.11,
            sensors: value.12,
            geo_altitude: value.13,
            squawk: value.14,
            spi: value.15,
            // FIX
            position_source: value.16,
            // FIX
            category: value.17,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StateResponse {
    pub time: usize,
    pub states: Option<Vec<StateVector>>,
}
