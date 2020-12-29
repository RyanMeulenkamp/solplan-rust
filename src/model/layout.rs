use druid::{Data, Lens};
use druid::im::Vector;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Data, Lens)]
pub struct Layout {
    rows: Vector<i32>,
}

impl Layout {
    pub fn get_rows(self) -> Vector<i32> {
        self.rows
    }
}

impl Layout {
    pub fn new(rows: Vector<i32>) -> Self {
        Layout {rows}
    }
}
