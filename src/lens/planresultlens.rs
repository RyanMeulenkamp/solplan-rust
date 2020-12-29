use druid::Lens;
use crate::model::plan::Plan;

pub struct PlanResultLens;

impl PlanResultLens {
    fn label_text(&self, plan: &Plan) -> String {
        format!(
            "{} panels (area: {:.2}m²; DC power: {:.0}Wp)",
            plan.get_total_panels(), plan.get_total_area(), plan.get_total_dc_power()
        )
    }
}

impl Lens<Plan, String> for PlanResultLens {
    fn with<V, F: FnOnce(&String) -> V>(&self, data: &Plan, f: F) -> V {
        f(&self.label_text(&data))
    }

    fn with_mut<V, F: FnOnce(&mut String) -> V>(&self, data: &mut Plan, f: F) -> V {
        f(&mut self.label_text(data))
    }
}