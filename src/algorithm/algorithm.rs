use crate::model::panel::Panel;
use crate::model::roof::Roof;
use crate::model::clearance::Clearance;
use druid::im::Vector;
use crate::model::plan::Plan;
use crate::model::boundary::Boundary;

pub trait Algorithm {
    fn layout(&self, roof: Roof, panel: &Panel, clearance: Clearance) -> Vector<i32>;

    fn plan(&self, roof: Roof, boundary: Boundary, panel: &Panel, clearence: Clearance) -> Plan {
        Plan::new(
            roof, boundary, panel.clone(), clearence,
            self.layout(roof.effective_roof(boundary), panel, clearence)
        )
    }
}

fn fit(roof: f64, panel: f64, clearance: f64) -> i32 {
    let mut row = 1.0;
    while row * panel + (row - 1.0) * clearance <= roof {
        row += 1.0;
    }
    (row - 1.0) as i32
}

pub struct Staggered;

impl Algorithm for Staggered {
    fn layout(&self, roof: Roof, panel: &Panel, clearance: Clearance) -> Vector<i32> {
        let mut layout: Vec<i32> = (1..=fit(roof.get_height(), panel.get_height(), clearance.get_horizontal()))
            .map(|row| row as f64 * panel.get_height() + (row - 1) as f64 * clearance.get_horizontal())
            .map(|row| fit(roof.width_at(row), panel.get_width(), clearance.get_vertical()))
            .collect();
        layout.sort();
        Vector::from(layout)
    }
}

pub struct Rasterized;

impl Algorithm for Rasterized {
    fn layout(&self, roof: Roof, panel: &Panel, clearance: Clearance) -> Vector<i32> {
        let mut layout: Vec<i32> = Staggered {}.layout(roof.half(), panel, clearance)
            .iter()
            .map(|row| row * 2)
            .collect();
        layout.sort();
        Vector::from(layout)
    }
}

