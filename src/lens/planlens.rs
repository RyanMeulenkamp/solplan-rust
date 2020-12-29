use crate::model::plan::Plan;
use crate::model::state::State;
use druid::Lens;
use druid::im::Vector;

pub struct PlanLens {

}

impl Lens<State, Vector<Plan>> for PlanLens {
    fn with<V, F: FnOnce(&Vector<Plan>) -> V>(&self, data: &State, f: F) -> V {
        let mut vec = data.get_plans();
        vec.sort();
        f(&vec)
    }

    fn with_mut<V, F: FnOnce(&mut Vector<Plan>) -> V>(&self, data: &mut State, f: F) -> V {
        let mut vec = data.get_plans();
        vec.sort();
        f(&mut vec)
    }
}
