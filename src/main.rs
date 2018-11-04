#![feature(proc_macro_hygiene, decl_macro)]

mod comms;
mod sensors;

use crate::sensors::{RequestType, Sensor, SensorMessage, SensorType};
use rocket::request::Form;
use rocket::{get, routes, State};
use rocket_contrib::json::Json;
use serde_derive::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::mpsc::{channel, Sender};
use std::sync::{Arc, Mutex};
use std::thread;

const NODE_ADDR: &str = "127.0.0.1:8100";
const LISTENER_ADDR: &str = "127.0.0.1:8200";

type ResponseMap = Arc<Mutex<HashMap<u32, Sender<String>>>>;
type SensorList = Arc<Mutex<HashSet<Sensor>>>;

#[get("/sensor?<sensor..>")]
fn read_sensor(sensor: Form<Sensor>, map: State<ResponseMap>) -> String {
    let sensor = sensor.into_inner();
    let (tx, rx) = channel();
    {
        let mut map = map.lock().unwrap();
        map.insert(sensor.sensor_id, tx);
    }
    let sensor_message = SensorMessage {
        sensor,
        request_type: RequestType::Get,
        payload: String::new(),
    };
    comms::send_to_node(NODE_ADDR, serde_json::to_string(&sensor_message).unwrap());
    let response = rx.recv().unwrap();
    format!("Message received from {}: {}!", sensor.sensor_id, response)
}

#[get("/sensors", format = "json")]
fn sensors(sensor_list: State<SensorList>) -> Json<SensorList> {
    Json((*sensor_list).clone())
}

#[get("/hello")]
fn hello() -> &'static str {
    "Hello World!"
}

fn main() {
    let response_map = Arc::new(Mutex::new(HashMap::new()));
    let (rocket_map, mbed_map) = (response_map.clone(), response_map);

    let sensor_1 = Sensor {
        sensor_id: 1,
        sensor_type: SensorType::Light,
    };
    let sensor_2 = Sensor {
        sensor_id: 2,
        sensor_type: SensorType::Lock,
    };

    let sensor_list = Arc::new(Mutex::new(HashSet::new()));
    {
        let mut sensor_list = sensor_list.lock().unwrap();
        sensor_list.insert(sensor_1);
        sensor_list.insert(sensor_2);
        // println!("{:#?}", sensor_list);
        let sensor_list_json = serde_json::to_string(&*sensor_list).unwrap();
        println!("{}", sensor_list_json);
    }

    thread::spawn(move || comms::node_listener(LISTENER_ADDR, mbed_map));

    rocket::ignite()
        .mount("/", routes![hello, sensors, read_sensor])
        .manage(rocket_map)
        .manage(sensor_list)
        .launch();
}
