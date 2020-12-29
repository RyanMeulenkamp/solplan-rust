use druid::{Data, Lens};

use crate::model::layout::Layout;
use crate::model::roof::Roof;
use crate::model::boundary::Boundary;
use crate::model::panel::Panel;
use crate::model::clearance::Clearance;
use crate::algorithm::algorithm::Algorithm;
use std::cmp::Ordering;

#[derive(Clone, PartialEq, Data, Lens)]
pub struct Plan {
    roof: Roof,
    boundary: Boundary,
    panel: Panel,
    clearance: Clearance,
    layout: Layout,
    algorithm: Algorithm,
}

impl PartialOrd for Plan {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.get_total_panels().cmp(&self.get_total_panels()))
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
        roof: Roof, boundary: Boundary, panel: Panel, clearance: Clearance, layout: Layout,
        algorithm: Algorithm
    ) -> Plan {
        Plan { roof, boundary, panel, clearance, algorithm, layout }
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

    pub fn set_algorithm(&mut self, algorithm: Algorithm) {
        self.algorithm = algorithm;
    }

    pub fn set_layout(&mut self, layout: Layout) {
        self.layout = layout;
    }

    pub fn get_roof(&self) -> Roof {
        self.roof
    }

    pub fn get_boundary(&self) -> Boundary {
        self.boundary
    }

    pub fn get_panel(&self) -> Panel {
        self.panel
    }

    pub fn get_clearance(&self) -> Clearance {
        self.clearance
    }

    pub fn get_algorithm(&self) -> Algorithm {
        self.algorithm
    }

    pub fn get_layout(&self) -> Layout {
        self.layout.clone()
    }

    pub fn get_total_panels(&self) -> i32 {
        self.get_layout().get_rows().iter().map(|row| row.clone() as i32).sum()
    }

    pub fn get_total_area(&self) -> f64 {
        self.get_total_panels() as f64 * self.panel.get_height() * self.panel.get_width() / 1_000_000.0
    }

    pub fn get_total_dc_power(&self) -> f64 {
        self.get_total_panels() as f64 * self.panel.get_peak_power()
    }

    pub fn height(&self) -> f64 {
        let rows = self.get_layout().get_rows().len() as f64;
        rows * self.panel.get_height() + (rows - 1.0) * self.clearance.get_horizontal()
    }

    pub fn width_at(&self, row: i32) -> f64 {
        self.layout.clone().get_rows()
            .get(row as usize)
            .map_or(0.0, |width| {
                *width as f64 * self.panel.get_width() + (*width - 1) as f64 * self.clearance.get_vertical()
            })
    }
}