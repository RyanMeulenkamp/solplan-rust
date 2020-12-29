use druid::{Widget, WidgetExt};
use crate::model::constraints::Constraints;
use druid::widget::{Flex, Checkbox, CrossAxisAlignment};

pub fn create_constraints_widget() -> impl Widget<Constraints> {
    Flex::column()
        .with_child(Checkbox::new("Portrait").lens(Constraints::portrait))
        .with_default_spacer()
        .with_child(Checkbox::new("Landscape").lens(Constraints::landscape))
        .with_default_spacer()
        .with_child(Checkbox::new("Transboundary").lens(Constraints::transboundary))
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .fix_width(340.0)
}