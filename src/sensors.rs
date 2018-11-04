use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Sensor {
    sensor_id: u32,
    sensor_type: SensorType,
}

#[derive(Serialize, Deserialize, Debug)]
struct SensorMessage {
    sensor: Sensor,
    request_type: RequestType,
    payload: String,
}

#[derive(Serialize, Deserialize, Debug)]
enum RequestType {
    Get,
    GetResponse,
    Set,
}

#[derive(Serialize, Deserialize, Debug)]
enum SensorType {
    Thermometer,
    Light,
    SmartSwitch,
    Thermostat,
    MusicPlayer,
    Store,
    Lock,
}
