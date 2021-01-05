use crate::algorithm::algorithm::Algorithm;
use crate::model::roof::Roof;
use crate::model::panel::Panel;
use crate::model::clearance::Clearance;
use druid::im::Vector;
use crate::algorithm::staggered::Staggered;

pub struct Rasterized{
    staggered: Staggered
}

impl Rasterized {
    pub fn new() -> Self {
        Rasterized {staggered: Staggered::new()}
    }
}

impl Algorithm for Rasterized {
    fn layout(&self, roof: Roof, panel: &Panel, clearance: Clearance) -> Vector<i32> {
        let mut layout: Vector<i32> = self.staggered.layout(roof.half(), panel, clearance)
            .into_iter()
            .map(|row| row * 2)
            .collect();
        layout.sort();
        layout
    }
}
