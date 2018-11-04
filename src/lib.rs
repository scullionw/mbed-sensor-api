pub mod comms;
pub mod sensors;

use crate::sensors::Sensor;
use std::collections::{HashMap, HashSet};
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};

pub type ResponseMap = Arc<Mutex<HashMap<u32, Sender<String>>>>;
pub type SensorList = Arc<Mutex<HashSet<Sensor>>>;

pub const BUF_SIZE: usize = 1024;
pub const NODE_ADDR: &str = "127.0.0.1:8100";
pub const LISTENER_ADDR: &str = "127.0.0.1:8200";
