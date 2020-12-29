use druid::{Data, Lens};

#[derive(Clone, Copy, PartialEq, PartialOrd, Data, Lens)]
pub struct Clearance {
    vertical: f64,
    horizontal: f64,
}

impl Eq for Clearance {

}

impl Clearance {
    pub fn new(vertical: f64, horizontal: f64) -> Self {
        Clearance { vertical, horizontal }
    }

    pub fn get_vertical(&self) -> f64 {
        self.vertical
    }

    pub fn get_horizontal(&self) -> f64 {
        self.horizontal
    }

    pub fn set_vertical(&mut self, vertical: f64) {
        self.vertical = vertical;
    }

    pub fn set_horizontal(&mut self, horizontal: f64) {
        self.horizontal = horizontal;
    }
}
