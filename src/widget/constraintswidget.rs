use druid::{Widget, WidgetExt};
use crate::model::constraints::Constraints;
use druid::widget::{Flex, Checkbox, CrossAxisAlignment, TextBox, Label};
use crate::format::currencyformatter::CurrencyFormatter;

pub fn create_constraints_widget() -> impl Widget<Constraints> {
    Flex::row()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(
            Flex::column()
                .cross_axis_alignment(CrossAxisAlignment::Start)
                .with_child(Checkbox::new("Portrait").lens(Constraints::portrait))
                .with_default_spacer()
                .with_child(Checkbox::new("Landscape").lens(Constraints::landscape))
                .with_default_spacer()
                .with_child(Checkbox::new("Transboundary").lens(Constraints::transboundary))
        )
        .with_child(Label::new("Budget"))
        .with_default_spacer()
        .with_child(
            TextBox::new()
                .with_formatter(CurrencyFormatter::EUROS)
                .lens(Constraints::budget)
                .fix_width(110.0)
        )
        .fix_width(340.0)
}