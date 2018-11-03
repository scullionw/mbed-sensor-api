#![feature(proc_macro_hygiene, decl_macro)]

use rocket::request::Form;
use rocket::{get, routes, FromForm, State};
use std::sync::{Mutex, Arc};
use std::collections::HashMap;
use std::sync::mpsc::{channel, Sender, Receiver};
use std::thread;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;

const BUF_SIZE: usize = 1024;

struct ResponseMapper {
    map: Mutex<HashMap<String, Sender<String>>>,
}

fn send(message: String) {
    let mut stream = TcpStream::connect("127.0.0.1:8100").unwrap();
    stream.write(message.as_bytes()).unwrap();
    stream.flush().unwrap();
}

#[get("/<sensorid>/<message>")]
fn read_sensor(sensorid: String, message: String, state: State<ResponseMapper>) -> String {
    let (tx, rx) = channel();
    {
        let mut map = state.map.lock().unwrap();
        map.insert(sensorid.clone(), tx);
    }
    send(message);
    let response = rx.recv().unwrap();
    format!("Message received from {}: {}!", sensorid, response)
}

fn main() {
    let response_mapper_mbed = Arc::new(ResponseMapper { map: Mutex::new(HashMap::new()) });
    let response_mapper_rocket = response_mapper_mbed.clone();
    thread::spawn(move || {
        let listener = TcpListener::bind("127.0.0.1:8100").unwrap();
        for stream in listener.incoming() {
            let mut stream = stream.unwrap();
            let mut buf = [0; BUF_SIZE];
            stream.read(&mut buf).unwrap();
            let data = String::from_utf8_lossy(&buf).to_string();
            println!("{}", data);
            response_mapper_mbed.map
                .lock()
                .unwrap()
                .remove(&data)
                .map(|tx| tx.send(data).unwrap());
        }
    });

    rocket::ignite()
        .mount("/", routes![read_sensor])
        .manage(response_mapper_rocket)
        .launch();
}
