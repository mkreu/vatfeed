use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Datafeed {
    pub general: General,
    pub pilots: Vec<Pilot>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct General {
    pub update: String,
    pub update_timestamp: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Pilot {
    pub cid: usize,
    pub callsign: String,
    pub flight_plan: Option<Flightplan>,
    pub logon_time: DateTime<Utc>,
    pub groundspeed: isize,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Flightplan {
    pub departure: String,
    pub arrival: String,
}
