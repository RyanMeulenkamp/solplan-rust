use crate::model::panel::Panel;
use crate::model::roof::Roof;
use crate::model::clearance::Clearance;
use druid::im::Vector;
use crate::model::plan::Plan;
use crate::model::boundary::Boundary;

pub trait Algorithm {
    fn fit(&self, roof: f64, panel: f64, clearance: f64) -> i32 {
        let mut row = 1.0;
        while row * panel + (row - 1.0) * clearance <= roof {
            row += 1.0;
        }
        (row - 1.0) as i32
    }

    fn layout(&self, roof: Roof, panel: &Panel, clearance: Clearance) -> Vector<i32>;

    fn plan(&self, roof: Roof, boundary: Boundary, panel: &Panel, clearence: Clearance) -> Plan {
        Plan::new(
            roof, boundary, panel.clone(), clearence,
            self.layout(roof.effective_roof(boundary), panel, clearence)
        )
    }
}
