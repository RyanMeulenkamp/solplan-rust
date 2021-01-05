use crate::algorithm::algorithm::Algorithm;
use crate::model::roof::Roof;
use crate::model::panel::Panel;
use crate::model::clearance::Clearance;
use druid::im::Vector;

pub struct Staggered;

impl Staggered {
    pub fn new() -> Staggered {
        Staggered {}
    }
}

impl Algorithm for Staggered {
    fn layout(&self, roof: Roof, panel: &Panel, clearance: Clearance) -> Vector<i32> {
        let rows = self.fit(roof.get_height(), panel.get_height(), clearance.get_horizontal());
        let mut layout: Vector<i32> = (1..=rows)
            .map(|row| row as f64)
            .map(|row| row * panel.get_height() + (row - 1.0) * clearance.get_horizontal())
            .map(|height| roof.width_at(height))
            .map(|width| self.fit(width, panel.get_width(), clearance.get_vertical()))
            .collect();
        layout.sort();
        layout
    }
}
