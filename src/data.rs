use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Datafeed {
    pub general: General,
    pub pilots: Vec<Pilot>,
    pub controllers: Vec<Controller>,
    pub atis: Vec<Controller>,
    pub servers: Vec<Server>,
    pub prefiles: Vec<Prefile>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct General {
    pub version: isize,
    pub reload: isize,
    pub update: String,
    pub update_timestamp: DateTime<Utc>,
    pub connected_clients: isize,
    pub unique_users: isize,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Pilot {
    pub cid: isize,
    pub name: String,
    pub callsign: String,
    pub server: String,
    pub pilot_rating: isize,
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: isize,
    pub groundspeed: isize,
    pub transponder: String,
    pub heading: isize,
    pub qnh_i_hg: f32,
    pub qnh_mb: isize,
    pub flight_plan: Option<Flightplan>,
    pub logon_time: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Flightplan {
    pub departure: String,
    pub arrival: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Controller {
    pub cid: isize,
    pub name: String,
    pub callsign: String,
    pub frequency: String,
    pub facility: isize,
    pub rating: isize,
    pub server: String,
    pub visual_range: isize,
    pub atis_code: Option<String>,
    pub text_atis: Option<Vec<String>>,
    pub last_updated: DateTime<Utc>,
    pub logon_time: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Server {
    pub ident: String,
    pub hostname_or_ip: String,
    pub location: String,
    pub name: String,
    pub clients_connection_allowed: isize,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Prefile {
    pub cid: isize,
    pub name: String,
    pub callsign: String,
    pub flight_plan: Option<Flightplan>,
    pub last_updated: DateTime<Utc>,
}
