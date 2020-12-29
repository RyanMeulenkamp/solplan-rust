use druid::{Data, Lens};
use crate::model::roof::Roof;
use crate::model::boundary::Boundary;
use crate::model::panel::Panel;
use crate::model::clearance::Clearance;
use crate::model::constraints::Constraints;
use druid::im::Vector;
use crate::model::plan::Plan;
use crate::model::layout::Layout;
use crate::algorithm::algorithm::{Algorithm, plan_rasterized, plan_staggered};

#[derive(Clone, Copy, PartialEq, PartialOrd, Data, Lens)]
pub struct State {
    roof: Roof,
    boundary: Boundary,
    panel: Panel,
    clearance: Clearance,
    constraints: Constraints,
}

impl State {
    pub fn new(
        roof: Roof, boundary: Boundary, panel: Panel, clearance: Clearance, constraints: Constraints
    ) -> Self {
        State { roof, boundary, panel, clearance, constraints }
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

    pub fn set_constraints(&mut self, constraints: Constraints) {
        self.constraints = constraints;
    }

    pub fn get_roof(self) -> Roof {
        self.roof
    }

    pub fn get_boundary(self) -> Boundary {
        self.boundary
    }

    pub fn get_panel(self) -> Panel {
        self.panel
    }

    pub fn get_clearance(self) -> Clearance {
        self.clearance
    }

    pub fn get_constraints(self) -> Constraints {
        self.constraints
    }

    pub fn get_plans(self) -> Vector<Plan> {
        let mut plans = self.constraints.get_orientations().into_iter()
            .flat_map(move |orientation| {
                self.constraints.get_algorithms().into_iter().map(move |algorithm| {
                    let panel = self.panel.transposed_to(orientation.clone());
                    Plan::new(
                        self.roof, self.boundary, panel, self.clearance,
                        Layout::new(match algorithm {
                            Algorithm::Rasterized => plan_rasterized(
                                self.roof.effective_roof(self.boundary), panel, self.clearance
                            ),
                            Algorithm::Staggered => plan_staggered(
                                self.roof.effective_roof(self.boundary), panel, self.clearance
                            ),
                        }),
                        algorithm
                    )
                })
            })
            .collect::<Vector<Plan>>();
        plans.sort();
        plans
    }
}