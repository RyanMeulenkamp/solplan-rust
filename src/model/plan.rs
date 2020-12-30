use druid::{Data, Lens};

use crate::model::roof::Roof;
use crate::model::boundary::Boundary;
use crate::model::panel::Panel;
use crate::model::clearance::Clearance;
use std::cmp::Ordering;
use druid::im::Vector;

#[derive(Clone, PartialEq, Data, Lens)]
pub struct Plan {
    roof: Roof,
    boundary: Boundary,
    panel: Panel,
    clearance: Clearance,
    layout: Vector<i32>,
}

impl PartialOrd for Plan {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.get_total_dc_power().partial_cmp(&self.get_total_dc_power())
    }
}

impl Eq for Plan {

}

impl Ord for Plan {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).expect("Naar")
    }
}

impl Plan {
    pub fn new(
        roof: Roof, boundary: Boundary, panel: Panel, clearance: Clearance, layout: Vector<i32>,
    ) -> Plan {
        Plan { roof, boundary, panel, clearance, layout }
    }

    pub fn set_roof(&mut self, roof: Roof) {
        self.roof = roof;
    }

    pub fn set_boundary(&mut self, boundary: Boundary) {
        self.boundary = boundary;
    }

    pub fn set_panel(&mut self, panel: Panel) {
        self.panel = panel;
    }

    pub fn set_clearance(&mut self, clearance: Clearance) {
        self.clearance = clearance;
    }

    pub fn set_layout(&mut self, layout: Vector<i32>) {
        self.layout = layout;
    }

    pub fn get_roof(&self) -> Roof {
        self.roof
    }

    pub fn get_boundary(&self) -> Boundary {
        self.boundary
    }

    pub fn get_panels(&self) -> Panel {
        self.panel.clone()
    }

    pub fn get_clearance(&self) -> Clearance {
        self.clearance
    }

    pub fn get_layout(&self) -> Vector<i32> {
        self.layout.clone()
    }

    pub fn get_total_panels(&self) -> i32 {
        self.get_layout().iter().map(|row| row.clone() as i32).sum()
    }

    pub fn get_total_area(&self) -> f64 {
        self.get_total_panels() as f64 * self.panel.get_height() * self.panel.get_width() / 1_000_000.0
    }

    pub fn get_total_dc_power(&self) -> f64 {
        self.get_total_panels() as f64 * self.panel.get_peak_power()
    }

    fn distance(amount: i32, panel: f64, clearance: f64) -> f64 {
        let amount = amount as f64;
        amount * panel + (amount - 1.0) * clearance
    }

    pub fn height(&self) -> f64 {
        Plan::distance(self.get_layout().len() as i32, self.panel.get_height(), self.clearance.get_horizontal())
    }

    pub fn width_at(&self, row: usize) -> f64 {
        Plan::distance(self.layout[row], self.panel.get_width(), self.clearance.get_vertical())
    }
}