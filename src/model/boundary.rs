use druid::{Data, Lens};

use crate::model::roof::Roof;

#[derive(Clone, Copy, PartialEq, PartialOrd, Data, Lens)]
pub struct Boundary {
    top: f64,
    bottom: f64,
    left: f64,
    right: f64,
}

impl Eq for Boundary {

}

impl Boundary {
    pub fn new(top: f64, bottom: f64, left: f64, right: f64) -> Self {
        Boundary { top, bottom, left, right }
    }

    pub fn effective_roof(&self, roof: Roof) -> Roof {
        roof.effective_roof(self.clone())
    }

    pub fn scaled(&self, scale: f64) -> Self {
        Self::new(self.top * scale, self.bottom * scale, self.left * scale, self.right * scale)
    }

    pub fn get_top(&self) -> f64 {
        self.top
    }

    pub fn get_bottom(&self) -> f64 {
        self.bottom
    }

    pub fn get_left(&self) -> f64 {
        self.left
    }

    pub fn get_right(&self) -> f64 {
        self.right
    }

    pub fn set_top(&mut self, top: f64) {
        self.top = top;
    }

    pub fn set_bottom(&mut self, bottom: f64) {
        self.bottom = bottom;
    }

    pub fn set_left(&mut self, left: f64) {
        self.left = left;
    }

    pub fn set_right(&mut self, right: f64) {
        self.right = right;
    }
}
