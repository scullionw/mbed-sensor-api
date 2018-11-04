use rocket::{FromForm, FromFormValue};
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Hash, Eq, PartialEq, FromForm, Copy, Clone)]
pub struct Sensor {
    pub sensor_id: u32,
    pub sensor_type: SensorType,
}

impl Sensor {
    pub fn new(sensor_id: u32, sensor_type: SensorType) -> Sensor {
        Sensor {
            sensor_id,
            sensor_type,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Hash, Eq, PartialEq, Clone)]
pub struct SensorMessage {
    pub sensor: Sensor,
    pub request_type: RequestType,
    pub payload: String,
}

impl SensorMessage {
    pub fn get(sensor: Sensor) -> SensorMessage {
        SensorMessage {
            sensor,
            request_type: RequestType::Get,
            payload: String::new(),
        }
    }

    pub fn set(sensor: Sensor, set_val: String) -> SensorMessage {
        SensorMessage {
            sensor,
            request_type: RequestType::Set,
            payload: set_val,
        }
    }
    
    pub fn extract_payload(&self) -> String {
        self.payload.clone()
    }

    pub fn replace_payload(&mut self, new_payload: String) {
        self.payload = new_payload;
    }

    pub fn change_request_type(&mut self, t: RequestType) {
        self.request_type = t;
    }
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
