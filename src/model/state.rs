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
use serde::{Serialize, Deserialize};
use druid::im::vector;

#[derive(Clone, PartialEq, PartialOrd, Data, Lens, Serialize, Deserialize)]
pub struct State {
    roof: Roof,
    boundary: Boundary,
    panels: Vector<Panel>,
    clearance: Clearance,
    constraints: Constraints,
    filter: String,
}

impl State {
    pub fn fallback() -> Self {
        State::new(
            Roof::new(9625.0, 13500.0, 7320.0),
            Boundary::new(30.0, 200.0, 30.0, 30.0),
            vector![
                Panel::new("JinkoSolar JKM345M-60H",    1684.0, 1002.0, 345.0, 345.0 / 3.0),
                Panel::new("JinkoSolar JKM375M-66H",    1841.0, 1002.0, 375.0, 375.0 / 3.0),
                Panel::new("JinkoSolar JKM410M-72H",    2008.0, 1002.0, 410.0, 410.0 / 3.0),
                Panel::new("JinkoSolar JKM445M-78H",    2166.0, 1002.0, 445.0, 445.0 / 3.0),
                Panel::new("JinkoSolar JKM395N-6RL3",   1855.0, 1029.0, 395.0, 395.0 / 3.0),
                Panel::new("JinkoSolar JKM470N-7RL3",   2182.0, 1029.0, 470.0, 470.0 / 3.0),
                Panel::new("Sunpower MAX3-400",         1690.0, 1046.0, 400.0, 400.0 / 3.0),
            ],
            Clearance::new(10.0, 10.0),
            Constraints::new(true, true, false, 30000.0),
            "",
        )
    }

    pub fn new(
        roof: Roof, boundary: Boundary, panel: Vector<Panel>, clearance: Clearance,
        constraints: Constraints, filter: &str
    ) -> Self {
        State { roof, boundary, panels: panel, clearance, constraints, filter: filter.to_string() }
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

    pub fn set_filter(&mut self, filter: String) {
        self.filter = filter;
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

    pub fn get_filter(&self) -> String {
        self.filter.clone()
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
                .filter(|plan| plan.get_total_price() <= self.constraints.get_budget())
            )
            .collect::<Vec<Plan>>();
        plans.sort();
        plans.dedup();
        Vector::from(plans.into_iter().take(10).collect::<Vec<Plan>>())
    }
}