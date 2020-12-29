use druid::Lens;
use crate::model::roof::Roof;
use crate::model::boundary::Boundary;
use crate::model::state::State;

pub struct RoofBoundaryTupleLens {

}

impl Lens<State, (Roof, Boundary)> for RoofBoundaryTupleLens {
    fn with<V, F: FnOnce(&(Roof, Boundary)) -> V>(&self, data: &State, f: F) -> V {
        f(&(data.get_roof().clone(), data.get_boundary()))
    }

    fn with_mut<V, F: FnOnce(&mut (Roof, Boundary)) -> V>(&self, data: &mut State, f: F) -> V {
        f(&mut (data.get_roof().clone(), data.get_boundary()))
    }
}
