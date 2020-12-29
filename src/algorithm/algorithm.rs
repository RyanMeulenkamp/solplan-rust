use crate::model::panel::Panel;
use crate::model::roof::Roof;
use crate::model::clearance::Clearance;
use druid::im::Vector;
use druid::Data;

#[derive(Clone, PartialEq, Data, Copy)]
pub enum Algorithm {
    Rasterized, Staggered
}

fn fit(roof: f64, panel: f64, clearance: f64) -> i32 {
    let mut row = 1.0;
    while row * panel + (row - 1.0) * clearance <= roof {
        row += 1.0;
    }
    (row - 1.0) as i32
}

pub fn plan_staggered(roof: Roof, panel: Panel, clearance: Clearance) -> Vector<i32> {
    let mut layout: Vec<i32> = (1..=fit(roof.get_height(), panel.get_height(), clearance.get_horizontal()))
        .map(|row| row as f64 * panel.get_height() + (row - 1) as f64 * clearance.get_horizontal())
        .map(|row| fit(roof.width_at(row), panel.get_width(), clearance.get_vertical()))
        .collect();
    layout.sort();
    Vector::from(layout)
}

pub fn plan_rasterized(roof: Roof, panel: Panel, clearance: Clearance) -> Vector<i32> {
    let mut layout: Vec<i32> = plan_staggered(roof.half(), panel, clearance)
        .iter()
        .map(|row| row * 2)
        .collect();
    layout.sort();
    Vector::from(layout)
}
