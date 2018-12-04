use rocket::{FromForm, FromFormValue};
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Hash, Eq, PartialEq, Clone)]
pub struct Year {
    year: String,
    value: u32
}

impl Year {
    pub fn new(year: u16, value: u32) -> Year {
        Year { year: year.to_string(), value }
    }
}