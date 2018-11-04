use rocket::{FromForm, FromFormValue};
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Hash, Eq, PartialEq, FromForm, Copy, Clone)]
pub struct Sensor {
    pub sensor_id: u32,
    pub sensor_type: SensorType,
}

#[derive(Serialize, Deserialize, Debug, Hash, Eq, PartialEq, Clone)]
pub struct SensorMessage {
    pub sensor: Sensor,
    pub request_type: RequestType,
    pub payload: String,
}

#[derive(Serialize, Deserialize, Debug, Hash, Eq, PartialEq, Copy, Clone)]
pub enum RequestType {
    Get,
    GetResponse,
    Set,
}

#[derive(Serialize, Deserialize, Debug, Hash, Eq, PartialEq, FromFormValue, Copy, Clone)]
pub enum SensorType {
    Thermometer,
    Light,
    SmartSwitch,
    Thermostat,
    MusicPlayer,
    Store,
    Lock,
}
