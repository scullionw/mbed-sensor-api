pub mod comms;
pub mod config;
pub mod sensors;

use crate::sensors::Sensor;
use std::collections::{HashMap, HashSet};
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};

pub type ResponseMap = Arc<Mutex<HashMap<u32, Sender<String>>>>;
pub type SensorList = Arc<Mutex<HashSet<Sensor>>>;

pub const BUF_SIZE: usize = 1024;
