#![feature(proc_macro_hygiene, decl_macro)]

use rocket::request::Form;
use rocket::{get, routes, FromForm, State};
use std::collections::HashMap;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;

const BUF_SIZE: usize = 1024;

struct ResponseMapper {
    map: Mutex<HashMap<String, Sender<String>>>,
}

fn send_to_node(message: String) {
    let mut stream = TcpStream::connect("127.0.0.1:8100").unwrap();
    println!("SENDING STRING: {:?}", message);
    let message = message.as_bytes();
    println!("SENDING BYTES: {:?}", message);
    stream.write(message).unwrap();
    stream.flush().unwrap();
}

#[get("/<sensorid>/<message>")]
fn read_sensor(sensorid: String, message: String, state: State<Arc<ResponseMapper>>) -> String {
    let (tx, rx) = channel();
    {
        let mut map = state.map.lock().unwrap();
        map.insert(message.clone(), tx);
    }
    send_to_node(message);
    let response = rx.recv().unwrap();
    format!("Message received from {}: {}!", sensorid, response)
}

#[get("/hello")]
fn hello() -> &'static str {
    "Hello world!"
}

fn main() {
    let response_mapper_mbed = Arc::new(ResponseMapper {
        map: Mutex::new(HashMap::new()),
    });
    let response_mapper_rocket = response_mapper_mbed.clone();
    thread::spawn(move || {
        let listener = TcpListener::bind("127.0.0.1:8200").unwrap();
        for stream in listener.incoming() {
            let mut stream = stream.unwrap();
            let mut buf = [0; BUF_SIZE];
            let bytes_read = stream.read(&mut buf).unwrap();
            println!("RECEIVED BUF: {:?}", &buf[..bytes_read]);
            let data = String::from_utf8_lossy(&buf[..bytes_read]).to_string();
            println!("RECEIVED STRING: {:?}", data);
            response_mapper_mbed
                .map
                .lock()
                .unwrap()
                .remove(&data)
                .map(|tx| tx.send(data).unwrap());
        }
    });

    rocket::ignite()
        .mount("/", routes![read_sensor, hello])
        .manage(response_mapper_rocket)
        .launch();
}
