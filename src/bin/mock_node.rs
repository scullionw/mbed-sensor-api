use sensor_api::comms;
use sensor_api::sensors::{RequestType, SensorMessage};
use sensor_api::{LISTENER_ADDR, NODE_ADDR};
use std::net::TcpListener;
use std::net::TcpStream;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

fn main() {
    let (tx, rx) = channel();
    let handles = vec![
        thread::spawn(move || mock_fixed_node_receiver(NODE_ADDR, tx)),
        thread::spawn(move || mock_fixed_node_sender(LISTENER_ADDR, rx)),
    ];
    for handle in handles {
        handle.join().unwrap();
    }
}

fn mock_fixed_node_receiver(addr: &str, tx: Sender<String>) {
    let listener = TcpListener::bind(addr).unwrap();
    for stream in listener.incoming() {
        let data = comms::read_string(stream.unwrap());
        println!("RECEIVED: {:?}", &data);
        let sensor_reponse = mock_mobile_node(data);
        tx.send(sensor_reponse).unwrap();
    }
}

fn mock_fixed_node_sender(addr: &str, rx: Receiver<String>) {
    for data in rx {
        println!("SENDING: {:?}", &data);
        let stream = TcpStream::connect(addr).unwrap();
        comms::send_string(data, stream);
    }
}

fn mock_mobile_node(data: String) -> String {
    let mut message: SensorMessage = serde_json::from_str(&data).unwrap();
    match message.request_type {
        RequestType::Get => {
            let mut new_payload = message.extract_payload();
            new_payload.push_str("MOCK_SENSOR_VALUE");
            message.replace_payload(new_payload);
        }
        RequestType::Set => println!("New value: {} has been set!", message.extract_payload()),
        RequestType::GetResponse => unreachable!(),
    }
    message.change_request_type(RequestType::GetResponse);
    serde_json::to_string(&message).unwrap()
}
