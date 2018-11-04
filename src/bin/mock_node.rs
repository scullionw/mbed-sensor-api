use sensor_api::comms;
use sensor_api::{LISTENER_ADDR, NODE_ADDR};
use std::net::TcpListener;
use std::net::TcpStream;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

fn main() {
    let (tx, rx) = channel();
    let handles = vec![
        thread::spawn(move || mock_node_receiver(NODE_ADDR, tx)),
        thread::spawn(move || mock_node_sender(LISTENER_ADDR, rx)),
    ];
    for handle in handles {
        handle.join().unwrap();
    }
}

fn mock_node_receiver(addr: &str, tx: Sender<String>) {
    let listener = TcpListener::bind(addr).unwrap();
    for stream in listener.incoming() {
        let data = comms::read_string(stream.unwrap());
        println!("RECEIVED: {:?}", &data);
        tx.send(data).unwrap();
    }
}

fn mock_node_sender(addr: &str, rx: Receiver<String>) {
    for data in rx {
        println!("SENDING: {:?}", &data);
        let stream = TcpStream::connect(addr).unwrap();
        comms::send_string(data, stream);
    }
}
