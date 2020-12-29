use druid::{
    Widget, WidgetExt, EventCtx, Event, Env, LifeCycleCtx, LifeCycle, UpdateCtx, BoxConstraints,
    LayoutCtx, Size, PaintCtx, Color, RenderContext
};
use druid::widget::{Flex, Label, TextBox, SizedBox, MainAxisAlignment};
use crate::format::distance::SiFormatter;
use crate::model::clearance::Clearance;

struct ClearanceGraphics;

impl Widget<Clearance> for ClearanceGraphics {
    fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, _data: &mut Clearance, _env: &Env) {

    }

    fn lifecycle(
        &mut self,
        _ctx: &mut LifeCycleCtx,
        _event: &LifeCycle,
        _data: &Clearance,
        _env: &Env,
    ) {

    }

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &Clearance, _data: &Clearance, _env: &Env) {

    }

    fn layout(
        &mut self,
        _layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _clearance: &Clearance,
        _env: &Env,
    ) -> Size {
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

pub fn create_clearance_forms_widget() -> impl Widget<Clearance> {
    Flex::column()
        .cross_axis_alignment(druid::widget::CrossAxisAlignment::End)
        .with_child(
            Flex::row()
                .cross_axis_alignment(druid::widget::CrossAxisAlignment::Baseline)
                .with_child(Label::new("Vertical"))
                .with_default_spacer()
                .with_child(
                    TextBox::new()
                        .with_formatter(SiFormatter::MILLIMETERS)
                        .lens(Clearance::vertical)
                )
        )
        .with_default_spacer()
        .with_child(
            Flex::row()
                .cross_axis_alignment(druid::widget::CrossAxisAlignment::Baseline)
                .with_child(Label::new("Horizontal"))
                .with_default_spacer()
                .with_child(
                    TextBox::new()
                        .with_formatter(SiFormatter::MILLIMETERS)
                        .lens(Clearance::horizontal)
                )
        )
}

pub fn create_clearance_widget() -> impl Widget<Clearance> {
    Flex::row()
        .with_child(create_clearance_forms_widget())
        .with_default_spacer()
        .with_child(SizedBox::new(ClearanceGraphics {}).width(120.0).height(80.0))
        .main_axis_alignment(MainAxisAlignment::End)
        .fix_width(340.0)
}
