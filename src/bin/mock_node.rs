use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

const BUF_SIZE: usize = 1024;

fn main() {
    let (tx, rx) = channel();
    let handles = vec![
        thread::spawn(move || node_receiver(tx)),
        thread::spawn(move || node_sender(rx)),
    ];
    for handle in handles {
        handle.join().unwrap();
    }
}

fn node_receiver(tx: Sender<String>) {
    let listener = TcpListener::bind("127.0.0.1:8100").unwrap();
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let mut buf = [0; BUF_SIZE];
        let bytes_read = stream.read(&mut buf).unwrap();
        let data = String::from_utf8_lossy(&buf[..bytes_read]).to_string();
        println!("RECEIVED: {:?}", &data);
        tx.send(data).unwrap();
    }
}

fn node_sender(rx: Receiver<String>) {
    for data in rx {
        let mut stream = TcpStream::connect("127.0.0.1:8200").unwrap();
        println!("SENDING: {:?}", &data);
        stream.write(data.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
