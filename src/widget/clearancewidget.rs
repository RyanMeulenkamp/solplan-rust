use druid::{
    Widget, WidgetExt, EventCtx, Event, Env, LifeCycleCtx, LifeCycle, UpdateCtx, BoxConstraints,
    LayoutCtx, Size, PaintCtx, Color, RenderContext
};
use druid::widget::{Flex, MainAxisAlignment};
use crate::format::siformatter::SiFormatter;
use crate::model::clearance::Clearance;
use crate::widget::common::create_form_element;

struct ClearanceGraphics;

impl Widget<Clearance> for ClearanceGraphics {
    fn event(&mut self, _: &mut EventCtx, _: &Event, _: &mut Clearance, _: &Env) {}
    fn lifecycle(&mut self, _: &mut LifeCycleCtx, _: &LifeCycle, _: &Clearance, _: &Env) {}
    fn update(&mut self, _: &mut UpdateCtx, _: &Clearance, _: &Clearance, _: &Env) {}

    fn layout(&mut self, _: &mut LayoutCtx, bc: &BoxConstraints, _: &Clearance, _: &Env) -> Size {
        if bc.is_width_bounded() | bc.is_height_bounded() {
            bc.constrain(Size::new(120.0, 80.0))
        } else {
            bc.max()
        }
    }

    fn paint(&mut self, ctx: &mut PaintCtx, clearance: &Clearance, _env: &Env) {
        let (one, two, three, four) = clearance.shapes(ctx.size());
        ctx.fill(one, &Color::rgb8(45, 58, 148));
        ctx.fill(two, &Color::rgb8(45, 58, 148));
        ctx.fill(three, &Color::rgb8(45, 58, 148));
        ctx.fill(four, &Color::rgb8(45, 58, 148));
    }
}

fn create_clearance_forms_widget() -> impl Widget<Clearance> {
    Flex::column()
        .cross_axis_alignment(druid::widget::CrossAxisAlignment::End)
        .with_child(create_form_element("Vertical", SiFormatter::MILLIMETERS, Clearance::vertical))
        .with_child(create_form_element("Horizontal", SiFormatter::MILLIMETERS, Clearance::horizontal))
}

pub fn create_clearance_widget() -> impl Widget<Clearance> {
    Flex::row()
        .with_child(create_clearance_forms_widget())
        .with_default_spacer()
        .with_child(ClearanceGraphics {}.padding((0.0, 0.0, 0.0, 10.0)).fix_size(120.0, 80.0))
        .main_axis_alignment(MainAxisAlignment::End)
        .fix_width(340.0)
}
