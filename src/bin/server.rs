#![feature(proc_macro_hygiene, decl_macro)]

use lazy_static::lazy_static;
use rocket::request::Form;
use rocket::{get, post, routes, State};
use rocket_contrib::json::Json;
use rocket_contrib::serve::StaticFiles;
use sensor_api::comms;
use sensor_api::config::LinkConfig;
use sensor_api::sensors::{RequestType, Sensor, SensorMessage};
use sensor_api::timeseries::Year;
use sensor_api::ResponseMap;
use sensor_api::SensorList;
use std::collections::{HashMap, HashSet};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::net::SocketAddrV4;
use std::net::TcpStream;

lazy_static! {
    static ref CONF: LinkConfig = LinkConfig::from_toml("Nodelink.toml");
}

pub fn rate_limited_sender(rx: Receiver<(String, SocketAddrV4)>) {
    for (message, addr) in rx {
        println!("SENDING STRING: {}", message);
        let stream = TcpStream::connect(addr).unwrap();
        comms::send_string(message, stream);
        println!("LIMITER WAITING {} ms.", CONF.time());
        std::thread::sleep(std::time::Duration::from_millis(CONF.time()));
        println!("LIMITER READY!");
    }
}

pub type Xmitter = Arc<Mutex<Sender<(String, SocketAddrV4)>>>;

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
    xmit: State<Xmitter>
) -> Option<Json<SensorMessage>> {
    let sensor = sensor.into_inner();
    let message = SensorMessage::get(sensor);
    match validate_and_channel(&message, &*map, &*sensor_list) {
        Some(rx) => {
            let xmit = (*xmit.lock().unwrap()).clone();
            comms::send_to_node(CONF.node().addr(), serde_json::to_string(&message).unwrap(), xmit);
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
    xmit: State<Xmitter>
) -> Option<Json<SensorMessage>> {
    let message = input.into_inner();
    match validate_and_channel(&message, &*map, &*sensor_list) {
        Some(rx) => match message.request_type {
            RequestType::Set => {
                let xmit = (*xmit.lock().unwrap()).clone();
                comms::send_to_node(CONF.node().addr(), serde_json::to_string(&message).unwrap(), xmit);
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
    xmit: State<Xmitter>
) -> Option<Json<SensorMessage>> {
    let message = SensorMessage::set(sensor.into_inner(), set_val);
    match validate_and_channel(&message, &*map, &*sensor_list) {
        Some(rx) => {
            let xmit = (*xmit.lock().unwrap()).clone();
            comms::send_to_node(CONF.node().addr(), serde_json::to_string(&message).unwrap(), xmit);
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
#[get("/polled_sensors", format = "json")]
fn polled_sensors(sensor_list: State<SensorList>) -> Json<HashSet<Sensor>> {
    let mut sensor_list = sensor_list.lock().unwrap().clone();
    sensor_list.retain(|&s| s.polled());
    Json(sensor_list)
}

#[get("/timeseries?<sensor..>", format = "json")]
fn timeseries(sensor: Form<Sensor>, _sensor_list: State<SensorList>) -> Json<Vec<Year>> {
    let sensor = sensor.into_inner();
    // if (&*sensor_list).lock().unwrap().contains(&sensor) {

    // }

    let data = vec![
        Year::new(2005, 771900 + sensor.sensor_id as u32),
        Year::new(2006, 771500),
        Year::new(2007, 770500),
        Year::new(2008, 770400),
        Year::new(2009, 771000),
        Year::new(2010, 772400),
        Year::new(2011, 774100),
        Year::new(2012, 776700),
        Year::new(2013, 777100),
        Year::new(2014, 779200),
        Year::new(2015, 782300),
    ];

    Json(data)
}

#[get("/status")]
fn health() -> &'static str {
    "Sensor API is up!"
}

fn main() {
    CONF.show();
    let sensor_list = Arc::new(Mutex::new(HashSet::new()));
    let response_map = Arc::new(Mutex::new(HashMap::new()));
    let (rocket_list, mbed_list) = (sensor_list.clone(), sensor_list);
    let (rocket_map, mbed_map) = (response_map.clone(), response_map);
    let (tx_lim, rx_lim) = channel();

    thread::spawn(move || comms::node_listener(CONF.listener().bind_addr(), mbed_map, mbed_list));
    thread::spawn(move || rate_limited_sender(rx_lim));

    rocket::ignite()
        .mount(
            "/",
            routes![
                health,
                active_sensors,
                read_sensor,
                set_as_get_sensor,
                set_sensor,
                polled_sensors,
                timeseries
            ],
        )
        .mount("/", StaticFiles::from("static"))
        .manage(rocket_map)
        .manage(rocket_list)
        .manage(Arc::new(Mutex::new(tx_lim)))
        .launch();
}
