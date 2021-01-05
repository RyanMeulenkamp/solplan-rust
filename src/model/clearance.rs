use druid::{Data, Lens};
use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, PartialEq, PartialOrd, Data, Lens, Serialize, Deserialize)]
pub struct Clearance {
    vertical: f64,
    horizontal: f64,
}

impl Eq for Clearance {

}

impl Clearance {
    pub const fn new(vertical: f64, horizontal: f64) -> Self {
        Clearance { vertical, horizontal }
    }

    pub fn get_vertical(&self) -> f64 {
        self.vertical
    }

    pub fn get_horizontal(&self) -> f64 {
        self.horizontal
    }
}
