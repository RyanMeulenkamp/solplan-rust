use druid::{Data, Lens};
use crate::model::boundary::Boundary;
use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, PartialEq, PartialOrd, Data, Lens, Serialize, Deserialize)]
pub struct Roof {
    ridge: f64,
    eaves: f64,
    height: f64,
}

impl Eq for Roof {

}

impl Roof {
    pub const fn new(ridge_width: f64, eaves_width: f64, height: f64) -> Self {
        Roof { ridge: ridge_width, eaves: eaves_width, height }
    }

    pub fn slope(&self) -> f64 {
        (self.eaves - self.ridge) / self.height
    }

    pub fn width_at(&self, height: f64) -> f64 {
        self.eaves - height * self.slope()
    }

    pub fn perpendicular_to_horizontal(&self, perpendicular_boundary: f64) -> f64 {
        let y = (self.eaves - self.ridge) * 0.5;
        perpendicular_boundary / y.atan2(self.height).cos()
    }

    pub fn horizontal_shift(&self, boundary: Boundary) -> f64 {
        self.perpendicular_to_horizontal(boundary.get_left() - boundary.get_right())
    }

    pub fn effective_roof(&self, boundary: Boundary) -> Roof {
        let clearance = self.perpendicular_to_horizontal(boundary.get_left() + boundary.get_right());
        Roof::new(
            self.width_at(self.height - boundary.get_top()) - clearance,
            self.width_at(boundary.get_bottom()) - clearance,
            self.height - boundary.get_top() - boundary.get_bottom()
        )
    }

    pub fn half(&self) -> Roof {
        Roof::new(self.ridge * 0.5, self.eaves * 0.5, self.height)
    }

    pub fn get_ridge(&self) -> f64 {
        self.ridge
    }

    pub fn get_eaves(&self) -> f64 {
        self.eaves
    }

    pub fn get_height(&self) -> f64 {
        self.height
    }

    pub fn set_ridge(&mut self, ridge_width: f64) {
        self.ridge = ridge_width;
    }

    pub fn set_eaves(&mut self, eaves_width: f64) {
        self.eaves = eaves_width;
    }

    pub fn set_height(&mut self, height: f64) {
        self.height = height;
    }

    pub fn aspect_ratio(&self) -> f64 {
        self.height / self.eaves
    }
}
