use rand::Rng;
use rocket::{FromForm, FromFormValue};
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Hash, Eq, PartialEq, FromForm, Copy, Clone)]
pub struct Sensor {
    pub sensor_id: u16,
    pub sensor_type: SensorType,
}

impl Sensor {
    pub fn new(sensor_id: u16, sensor_type: SensorType) -> Sensor {
        Sensor {
            sensor_id,
            sensor_type,
        }
    }

    pub fn polled(&self) -> bool {
        match self.sensor_type {
            SensorType::Thermometer => true,
            _ => false
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Hash, Eq, PartialEq, Clone)]
pub struct SensorMessage {
    pub sensor_id: u16,
    pub sensor_type: SensorType,
    pub message_id: u16,
    pub request_type: RequestType,
    pub payload: String,
}

impl SensorMessage {
    pub fn get(sensor: Sensor) -> SensorMessage {
        SensorMessage {
            sensor_id: sensor.sensor_id,
            message_id: rand::thread_rng().gen(),
            sensor_type: sensor.sensor_type,
            request_type: RequestType::Get,
            payload: String::new(),
        }
    }

    pub fn set(sensor: Sensor, set_val: String) -> SensorMessage {
        SensorMessage {
            sensor_id: sensor.sensor_id,
            message_id: rand::thread_rng().gen(),
            sensor_type: sensor.sensor_type,
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

    pub fn sensor_id(&self) -> u16 {
        self.sensor_id
    }

    pub fn message_id(&self) -> u16 {
        self.message_id
    }

    pub fn id(&self) -> (u16, u16) {
        (self.sensor_id, self.message_id)
    }

    pub fn sensor(&self) -> Sensor {
        Sensor {
            sensor_id: self.sensor_id,
            sensor_type: self.sensor_type,
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Hash, Eq, PartialEq, Copy, Clone)]
pub enum RequestType {
    Get,
    GetResponse,
    Set,
    Discovery,
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