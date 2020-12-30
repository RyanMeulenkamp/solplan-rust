use druid::{Data, Lens};
use druid::im::Vector;

use crate::algorithm::algorithm::Algorithm;
use crate::model::boundary::Boundary;
use crate::model::clearance::Clearance;
use crate::model::constraints::Constraints;
use crate::model::orientation::Orientation;
use crate::model::panel::Panel;
use crate::model::plan::Plan;
use crate::model::roof::Roof;

#[derive(Clone, PartialEq, PartialOrd, Data, Lens)]
pub struct State {
    roof: Roof,
    boundary: Boundary,
    panels: Vector<Panel>,
    clearance: Clearance,
    constraints: Constraints,
}

impl State {
    pub fn new(
        roof: Roof, boundary: Boundary, panel: Vector<Panel>, clearance: Clearance,
        constraints: Constraints
    ) -> Self {
        State { roof, boundary, panels: panel, clearance, constraints }
    }

    pub fn set_roof(&mut self, roof: Roof) {
        self.roof = roof;
    }

    pub fn set_boundary(&mut self, boundary: Boundary) {
        self.boundary = boundary;
    }

    pub fn set_panels(&mut self, panel: Vector<Panel>) {
        self.panels = panel;
    }

    pub fn set_clearance(&mut self, clearance: Clearance) {
        self.clearance = clearance;
    }

    pub fn set_constraints(&mut self, constraints: Constraints) {
        self.constraints = constraints;
    }

    pub fn get_roof(&self) -> Roof {
        self.roof
    }

    pub fn get_boundary(&self) -> Boundary {
        self.boundary
    }

    pub fn get_panels(&self) -> Vector<Panel> {
        self.panels.clone()
    }

    pub fn get_clearance(&self) -> Clearance {
        self.clearance
    }

    pub fn get_constraints(&self) -> Constraints {
        self.constraints
    }

    pub fn get_algorithms(&self) -> Vec<Box<dyn Algorithm>> {
        self.get_constraints().get_algorithms()
    }

    pub fn get_orientations(&self) -> Vector<Orientation> {
        self.get_constraints().get_orientations()
    }

    pub fn get_panel_orientations(&self) -> Vector<Panel> {
        self.get_panels()
            .into_iter()
            .flat_map(|panel| self.get_orientations()
                .into_iter()
                .map(move |orientation| panel.transposed_to(orientation)))
            .collect::<Vector<Panel>>()
    }

    fn plan(&self, planner: &Box<dyn Algorithm>, panel: Panel) -> Plan {
        planner.plan(self.get_roof(), self.get_boundary(), &panel, self.get_clearance())
    }

    pub fn get_plans(&self) -> Vector<Plan> {
        let mut plans = self.get_algorithms()
            .into_iter()
            .flat_map(|algorithm| self.get_panel_orientations()
                .into_iter()
                .map(move |panel| self.plan(&algorithm, panel))
                .filter(|plan| plan.get_total_panels() > 0)
            )
            .collect::<Vec<Plan>>();
        plans.sort();
        plans.dedup();
        Vector::from(plans)
    }
}