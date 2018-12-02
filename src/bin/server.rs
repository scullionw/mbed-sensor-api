#![feature(proc_macro_hygiene, decl_macro)]

use lazy_static::lazy_static;
use rocket::request::Form;
use rocket::{get, post, routes, State};
use rocket_contrib::json::Json;
use sensor_api::comms;
use sensor_api::config::LinkConfig;
use sensor_api::sensors::{RequestType, Sensor, SensorMessage, SensorType};
use sensor_api::ResponseMap;
use sensor_api::SensorList;
use std::collections::{HashMap, HashSet};
use std::sync::mpsc::{channel, Receiver};
use std::sync::{Arc, Mutex};
use std::thread;

lazy_static! {
    static ref CONF: LinkConfig = LinkConfig::from_toml("Nodelink.toml");
}

fn validate_and_channel(
    message: &SensorMessage,
    map: &ResponseMap,
    sensor_list: &SensorList,
) -> Option<Receiver<String>> {
    match sensor_list.lock().unwrap().contains(&message.sensor()) {
        true => {
            let (tx, rx) = channel();
            let mut map = map.lock().unwrap();
            map.insert(message.id(), tx);
            Some(rx)
        }
        false => None,
    }
}

#[get("/sensor?<sensor..>", format = "json")]
fn read_sensor(
    sensor: Form<Sensor>,
    map: State<ResponseMap>,
    sensor_list: State<SensorList>,
) -> Option<Json<SensorMessage>> {
    let sensor = sensor.into_inner();
    let message = SensorMessage::get(sensor);
    match validate_and_channel(&message, &*map, &*sensor_list) {
        Some(rx) => {
            comms::send_to_node(
                CONF.node().addr(),
                serde_json::to_string(&message).unwrap(),
            );
            let response = rx.recv().unwrap();
            let message = serde_json::from_str(&response).unwrap();
            Some(Json(message))
        }
        None => None,
    }
}

#[post("/sensor", data = "<input>", format = "json")]
fn set_sensor(
    input: Json<SensorMessage>,
    map: State<ResponseMap>,
    sensor_list: State<SensorList>,
) -> Option<Json<SensorMessage>> {
    let message = input.into_inner();
    match validate_and_channel(&message, &*map, &*sensor_list) {
        Some(rx) => match message.request_type {
            RequestType::Set => {
                comms::send_to_node(
                    CONF.node().addr(),
                    serde_json::to_string(&message).unwrap(),
                );
                let response = rx.recv().unwrap();
                let message = serde_json::from_str(&response).unwrap();
                Some(Json(message))
            }
            _ => None,
        },
        None => None,
    }
}

#[get("/sensor?<set_val>&<sensor..>", format = "json", rank = 2)]
fn set_as_get_sensor(
    set_val: String,
    sensor: Form<Sensor>,
    map: State<ResponseMap>,
    sensor_list: State<SensorList>,
) -> Option<Json<SensorMessage>> {
    let message = SensorMessage::set(sensor.into_inner(), set_val);
    match validate_and_channel(&message, &*map, &*sensor_list) {
        Some(rx) => {
            comms::send_to_node(
                CONF.node().addr(),
                serde_json::to_string(&message).unwrap(),
            );
            let response = rx.recv().unwrap();
            let message = serde_json::from_str(&response).unwrap();
            Some(Json(message))
        }
        None => None,
    }
}

#[get("/sensors", format = "json")]
fn active_sensors(sensor_list: State<SensorList>) -> Json<SensorList> {
    Json((*sensor_list).clone())
}

#[get("/status")]
fn health() -> &'static str {
    "Sensor API is up!"
}

fn initialize_mock_sensors() -> SensorList {
    let sensors = vec![
        Sensor::new(1, SensorType::Light),
        Sensor::new(2, SensorType::Lock),
    ];
    let sensor_list = Arc::new(Mutex::new(HashSet::new()));
    {
        let mut sensor_list = sensor_list.lock().unwrap();
        for sensor in sensors {
            sensor_list.insert(sensor);
        }
        println!("{}", serde_json::to_string(&*sensor_list).unwrap());
    }
    sensor_list
}

fn main() {
    CONF.show();
    let sensor_list = initialize_mock_sensors();
    let response_map = Arc::new(Mutex::new(HashMap::new()));
    let (rocket_map, mbed_map) = (response_map.clone(), response_map);

    thread::spawn(move || comms::node_listener(CONF.listener().bind_addr(), mbed_map));

    rocket::ignite()
        .mount(
            "/",
            routes![
                health,
                active_sensors,
                read_sensor,
                set_as_get_sensor,
                set_sensor
            ],
        )
        .manage(rocket_map)
        .manage(sensor_list)
        .launch();
}
