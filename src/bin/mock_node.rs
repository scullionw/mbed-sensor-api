use lazy_static::lazy_static;
use sensor_api::comms;
use sensor_api::config::LinkConfig;
use sensor_api::sensors::{Sensor, RequestType, SensorMessage, SensorType};
use std::net::SocketAddrV4;
use std::net::TcpListener;
use std::net::TcpStream;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use sensor_api::SensorList;
use std::sync::{Arc, Mutex};
use std::collections::{HashSet, HashMap};

lazy_static! {
    static ref CONF: LinkConfig = LinkConfig::from_toml("Nodelink.toml");
}

struct SensorMemory {
    pub target_value: String,
    pub sensor_value: String,
}

impl SensorMemory {
    fn new(target_value: String, sensor_value: String) -> SensorMemory {
        SensorMemory { target_value, sensor_value }
    }
}

fn main() {
    CONF.show();
    let sensor_list = initialize_mock_sensors();
    discovery(CONF.listener().addr(), &sensor_list);
    let (tx, rx) = channel();
    let handles = vec![
        thread::spawn(move || mock_fixed_node_receiver(CONF.node().bind_addr(), tx, &sensor_list)),
        thread::spawn(move || mock_fixed_node_sender(CONF.listener().addr(), rx)),
    ];
    for handle in handles {
        handle.join().unwrap();
    }
}

fn discovery(addr: SocketAddrV4, sensor_list: &SensorList) {
    for s in &*sensor_list.lock().unwrap() {
        let stream = TcpStream::connect(addr).unwrap();
        let discovery_message = s.discovery();
        let discovery_message = serde_json::to_string(&discovery_message).unwrap();
        comms::send_string(discovery_message, stream);
    }
}

fn mock_fixed_node_receiver(addr: SocketAddrV4, tx: Sender<String>, sensor_list: &SensorList) {
    let mut memory_map = HashMap::new();
    for s in &*sensor_list.lock().unwrap() {
        memory_map.insert(s.clone(), SensorMemory::new(String::new(), "DEFAULT_VALUE".to_owned()));
    }
    
    let listener = TcpListener::bind(addr).unwrap();
    for stream in listener.incoming() {
        let data = comms::read_string(stream.unwrap());
        println!("RECEIVED: {:?}", &data);
        let sensor_reponse = mock_mobile_node(data, &mut memory_map);
        tx.send(sensor_reponse).unwrap();
    }
}

fn mock_fixed_node_sender(addr: SocketAddrV4, rx: Receiver<String>) {
    for data in rx {
        println!("SENDING: {:?}", &data);
        let stream = TcpStream::connect(addr).unwrap();
        comms::send_string(data, stream);
    }
}

fn mock_mobile_node(data: String, mbed_memory: &mut HashMap<Sensor, SensorMemory>)  -> String {
    let mut message: SensorMessage = serde_json::from_str(&data).unwrap();
    match message.request_type {
        RequestType::Get => {
            let sensor_memory = mbed_memory.get(&message.sensor()).expect("Sensor does not exist!");
            message.replace_payload(sensor_memory.sensor_value.clone());
        }
        RequestType::Set => { 
            let sensor_memory = mbed_memory.get_mut(&message.sensor()).expect("Sensor does not exist!");
            sensor_memory.sensor_value = message.extract_payload();
            println!("New value: {} has been set!", sensor_memory.sensor_value)
        },
        _ => unreachable!(),
    }
    message.change_request_type(RequestType::GetResponse);
    serde_json::to_string(&message).unwrap()
}


fn initialize_mock_sensors() -> SensorList {
    let sensors = vec![
        Sensor::new(1, SensorType::Light),
        Sensor::new(2, SensorType::Lock),
        Sensor::new(3, SensorType::Thermometer),
        Sensor::new(4, SensorType::Thermometer),
        Sensor::new(5, SensorType::SmartSwitch),
        Sensor::new(6, SensorType::Thermostat),
        Sensor::new(7, SensorType::MusicPlayer),
        Sensor::new(8, SensorType::Store),
        Sensor::new(9, SensorType::Thermometer),
    ];
    let sensor_list = Arc::new(Mutex::new(HashSet::new()));
    {
        let mut sensor_list = sensor_list.lock().unwrap();
        for sensor in sensors {
            sensor_list.insert(sensor);
        }
        //println!("{}", serde_json::to_string(&*sensor_list).unwrap());
    }
    sensor_list
}