use druid::{Data, Lens};
use crate::model::orientation::Orientation;
use serde::{Serialize, Deserialize};

#[derive(Clone, PartialEq, PartialOrd, Data, Lens, Serialize, Deserialize)]
pub struct Panel {
    name: String,
    width: f64,
    height: f64,
    peak_power: f64,
    price: f64,
    selected: bool,
}

impl Panel {
    pub fn new(name: &str, width: f64, height: f64, peak_power: f64, price: f64) -> Self {
        Panel { name: name.to_string(), width, height, peak_power, price, selected: false }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_width(&self) -> f64 {
        self.width
    }

    pub fn get_height(&self) -> f64 {
        self.height
    }

    pub fn get_peak_power(&self) -> f64 {
        self.peak_power
    }

    pub fn get_price(&self) -> f64 {
        self.price
    }

    pub fn is_selected(&self) -> bool {
        self.selected
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    pub fn set_width(&mut self, width: f64) {
        self.width = width;
    }

    pub fn set_height(&mut self, height: f64) {
        self.height = height;
    }

    pub fn set_peak_power(&mut self, peak_power: f64) {
        self.peak_power = peak_power;
    }

    pub fn set_price(&mut self, price: f64) {
        self.price = price;
    }

    pub fn set_selected(&mut self, selected: bool) {
        self.selected = selected;
    }

    pub fn is_rectangle(&self) -> bool {
        self.height == self.width
    }

    pub fn is_portrait(&self) -> bool {
        self.height > self.width
    }

    pub fn is_landscape(&self) -> bool {
        self.height < self.width
    }

    pub fn get_orientation(&self) -> Orientation {
        if self.is_portrait() {
            Orientation::PORTRAIT
        } else if self.is_landscape() {
            Orientation::LANDSCAPE
        } else {
            Orientation::RECTANGLE
        }
    }

    pub fn transpose(&mut self) -> &Self {
        let new_width = self.height;
        self.set_height(self.width);
        self.set_width(new_width);
        self
    }

    pub fn transposed(&self) -> Self {
        Self::new(self.name.as_str(), self.height, self.width, self.peak_power, self.price)
    }

    pub fn as_portrait(&self) -> Self {
        if self.is_landscape() {
            self.transposed()
        } else {
            self.clone()
        }
    }

    pub fn as_landscape(&self) -> Self {
        if self.is_portrait() {
            self.transposed()
        } else {
            self.clone()
        }
    }

    pub fn to_portrait(&mut self) {
        if self.is_landscape() {
            self.transpose();
        }
    }

    pub fn to_landscape(&mut self) {
        if self.is_portrait() {
            self.transpose();
        }
    }

    pub fn transpose_to(&mut self, orientation: Orientation) {
        if orientation == Orientation::RECTANGLE {
        } else if orientation == Orientation::PORTRAIT {
            self.to_portrait()
        } else {
            self.to_landscape()
        }
    }

    pub fn transposed_to(&self, orientation: Orientation) -> Self {
        if orientation == Orientation::PORTRAIT {
            self.as_portrait()
        } else if orientation == Orientation::LANDSCAPE {
            self.as_landscape()
        } else {
            self.clone()
        }
    }
}
