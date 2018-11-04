#![feature(proc_macro_hygiene, decl_macro)]

use rocket::request::Form;
use rocket::{get, routes, State};
use rocket_contrib::json::Json;
use sensor_api::comms;
use sensor_api::sensors::{Sensor, SensorMessage, SensorType};
use sensor_api::ResponseMap;
use sensor_api::SensorList;
use sensor_api::{LISTENER_ADDR, NODE_ADDR};
use std::collections::{HashMap, HashSet};
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};
use std::thread;

#[get("/sensor?<sensor..>")]
fn read_sensor(
    sensor: Form<Sensor>,
    map: State<ResponseMap>,
    sensor_list: State<SensorList>,
) -> Option<Json<SensorMessage>> {
    let sensor = sensor.into_inner();

    if !sensor_list.lock().unwrap().contains(&sensor) {
        return None;
    }

    let (tx, rx) = channel();
    {
        let mut map = map.lock().unwrap();
        map.insert(sensor.sensor_id, tx);
    }

    let sensor_message = SensorMessage::get(sensor);
    comms::send_to_node(NODE_ADDR, serde_json::to_string(&sensor_message).unwrap());
    let response = rx.recv().unwrap();
    let sensor_message = serde_json::from_str(&response).unwrap();
    Some(Json(sensor_message))
}

#[get("/sensors", format = "json")]
fn active_sensors(sensor_list: State<SensorList>) -> Json<SensorList> {
    Json((*sensor_list).clone())
}

#[get("/hello")]
fn hello() -> &'static str {
    "Hello World!"
}

fn main() {
    let response_map = Arc::new(Mutex::new(HashMap::new()));
    let (rocket_map, mbed_map) = (response_map.clone(), response_map);

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

    thread::spawn(move || comms::node_listener(LISTENER_ADDR, mbed_map));

    rocket::ignite()
        .mount("/", routes![hello, active_sensors, read_sensor])
        .manage(rocket_map)
        .manage(sensor_list)
        .launch();
}
