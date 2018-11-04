use crate::sensors::SensorMessage;
use crate::ResponseMap;
use crate::BUF_SIZE;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

pub fn node_listener(addr: &str, response_map: ResponseMap) {
    let listener = TcpListener::bind(addr).unwrap();
    for stream in listener.incoming() {
        let data = read_string(stream.unwrap());
        println!("RECEIVED STRING: {:?}", data);
        let message: SensorMessage = serde_json::from_str(&data).unwrap();
        if let Some(tx) = response_map
            .lock()
            .unwrap()
            .remove(&message.sensor.sensor_id)
        {
            tx.send(data).unwrap()
        }
    }
}

pub fn send_to_node(addr: &str, message: String) {
    println!("SENDING STRING: {}", message);
    let stream = TcpStream::connect(addr).unwrap();
    send_string(message, stream);
}

pub fn read_string(mut stream: TcpStream) -> String {
    let mut buf = [0; BUF_SIZE];
    let bytes_read = stream.read(&mut buf).unwrap();
    String::from_utf8_lossy(&buf[..bytes_read]).to_string()
}

pub fn send_string(data: String, mut stream: TcpStream) {
    stream.write(data.as_bytes()).unwrap();
    stream.flush().unwrap();
}
