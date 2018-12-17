use crate::sensors::{RequestType, SensorMessage};
use crate::ResponseMap;
use crate::SensorList;
use crate::BUF_SIZE;
use std::io::prelude::*;
use std::net::SocketAddrV4;
use std::net::TcpListener;
use std::net::TcpStream;
use std::sync::mpsc::Sender;

pub fn node_listener(addr: SocketAddrV4, response_map: ResponseMap, sensor_list: SensorList) {
    let listener = TcpListener::bind(addr).unwrap();
    for stream in listener.incoming() {
        let data = read_string(stream.unwrap());
        println!("RECEIVED STRING: {:?}", data);
        if let Ok(message) = serde_json::from_str::<SensorMessage>(&data) {
            match message.request_type {
                RequestType::Discovery => {
                    let mut sensor_list = sensor_list.lock().unwrap();
                    sensor_list.insert(message.sensor());
                }
                _ => {
                    if let Some(tx) = response_map.lock().unwrap().remove(&message.id()) {
                        tx.send(data).unwrap()
                    }
                }
            }
        } else {
            println!("Parse error! continuing..");
        }
    }
}

pub fn send_to_node(addr: SocketAddrV4, message: String, tx: Sender<(String, SocketAddrV4)>) {
    tx.send((message, addr)).unwrap();
}

pub fn read_string(mut stream: TcpStream) -> String {
    let mut buf = [0; BUF_SIZE];
    let bytes_read = stream.read(&mut buf).unwrap();
    String::from_utf8_lossy(&buf[..bytes_read]).to_string()
}

pub fn send_string(data: String, mut stream: TcpStream) {
    stream.write_all(data.as_bytes()).unwrap();
    stream.flush().unwrap();
}
