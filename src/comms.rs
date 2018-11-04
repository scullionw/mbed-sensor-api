use crate::ResponseMap;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

const BUF_SIZE: usize = 1024;

pub fn node_listener(addr: &str, response_map: ResponseMap) {
    let listener = TcpListener::bind(addr).unwrap();
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let mut buf = [0; BUF_SIZE];
        let bytes_read = stream.read(&mut buf).unwrap();
        println!("RECEIVED BUF: {:?}", &buf[..bytes_read]);
        let data = String::from_utf8_lossy(&buf[..bytes_read]).to_string();
        println!("RECEIVED STRING: {:?}", data);
        response_map
            .lock()
            .unwrap()
            .remove(&data)
            .map(|tx| tx.send(data).unwrap());
    }
}

pub fn send_to_node(addr: &str, message: String) {
    let mut stream = TcpStream::connect(addr).unwrap();
    println!("SENDING STRING: {:?}", message);
    let message = message.as_bytes();
    println!("SENDING BYTES: {:?}", message);
    stream.write(message).unwrap();
    stream.flush().unwrap();
}
