use druid::{Data, Lens};
use druid::im::Vector;
use crate::model::orientation::Orientation;
use crate::algorithm::algorithm::Algorithm;
use crate::algorithm::algorithm::{Rasterized, Staggered};

#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Data, Lens)]
pub struct Constraints {
    portrait: bool,
    landscape: bool,
    transboundary: bool,
}

impl Constraints {
    pub fn new(portrait: bool, landscape: bool, transboundary: bool) -> Self {
        Constraints { portrait, landscape, transboundary }
    }

    pub fn set_portrait(&mut self, portrait: bool) {
        self.portrait = portrait;
    }

    pub fn set_landscape(&mut self, landscape: bool) {
        self.landscape = landscape;
    }

    pub fn set_transboundary(&mut self, transboundary: bool) {
        self.transboundary = transboundary;
    }

    pub fn get_portrait(&self) -> bool {
        self.portrait
    }

    pub fn get_landscape(&self) -> bool {
        self.landscape
    }

    pub fn get_transboundary(&self) -> bool {
        self.transboundary
    }

    pub fn get_orientations(&self) -> Vector<Orientation> {
        let mut orientations: Vector<Orientation> = Vector::new();
        if self.portrait {
            orientations.push_back(Orientation::PORTRAIT);
        }
        if self.landscape {
            orientations.push_back(Orientation::LANDSCAPE);
        }
        orientations
    }

    pub fn get_algorithms(self) -> Vec<Box<dyn Algorithm>> {
        let mut vector: Vec<Box<dyn Algorithm>> = Vec::new();
        vector.push(Box::new(Rasterized {}));
        if self.transboundary {
            vector.push(Box::new(Staggered {}));
        }
        vector
    }
}
