#![feature(proc_macro_hygiene, decl_macro)]

mod sensors;
mod comms;

use rocket::{get, routes, State};
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::mpsc::{channel, Sender};
use std::sync::{Arc, Mutex};
use std::thread;

const NODE_ADDR: &str = "127.0.0.1:8100";
const LISTENER_ADDR: &str = "127.0.0.1:8200";

type ResponseMap = Arc<Mutex<HashMap<String, Sender<String>>>>;

#[get("/<sensorid>/<message>")]
fn sensor_test(sensorid: String, message: String, map: State<ResponseMap>) -> String {
    let (tx, rx) = channel();
    {
        let mut map = map.lock().unwrap();
        map.insert(message.clone(), tx);
    }
    comms::send_to_node(NODE_ADDR, message);
    let response = rx.recv().unwrap();
    format!("Message received from {}: {}!", sensorid, response)
}

#[get("/hello")]
fn hello() -> &'static str {
    "Hello World!"
}

fn main() {
    let response_map = Arc::new(Mutex::new(HashMap::new()));
    let (rocket_map, mbed_map) = (response_map.clone(), response_map);

    thread::spawn(move || comms::node_listener(LISTENER_ADDR, mbed_map));

    rocket::ignite()
        .mount("/", routes![hello, sensor_test])
        .manage(rocket_map)
        .launch();
}
