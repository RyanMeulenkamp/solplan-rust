use druid::{Data, Widget, Lens, WidgetExt};
use druid::widget::{Flex, Label, TextBox, CrossAxisAlignment};
use druid::text::format::Formatter;


pub fn create_form_element<I: Data, O: Data, L: Lens<I, O>>(
    text: impl Into<String>, formatter: impl Formatter<O> + 'static, lens: L
) -> impl Widget<I> {
    Flex::row()
        .cross_axis_alignment(CrossAxisAlignment::Baseline)
        .with_default_spacer()
        .with_child(Label::new(text.into()))
        .with_default_spacer()
        .with_child(TextBox::new().with_formatter(formatter))
        .padding((0.0, 0.0, 0.0, 10.0))
        .lens(lens)
}
