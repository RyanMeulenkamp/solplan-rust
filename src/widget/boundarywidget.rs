use druid::{
    Widget, WidgetExt, EventCtx, Event, Env, LifeCycleCtx, LifeCycle, UpdateCtx, BoxConstraints,
    LayoutCtx, Size, PaintCtx, Color, RenderContext, Affine, Point
};
use crate::model::roof::Roof;
use druid::widget::{Flex, Label, TextBox, SizedBox, MainAxisAlignment};
use crate::format::distance::SiFormatter;
use crate::model::boundary::Boundary;
use crate::model::state::State;
use crate::lens::roofboundarytuplelens::RoofBoundaryTupleLens;

pub struct BoundaryGraphics;

impl Widget<(Roof, Boundary)> for BoundaryGraphics {
    fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, _data: &mut (Roof, Boundary), _env: &Env) {

    }

    fn lifecycle(
        &mut self,
        _ctx: &mut LifeCycleCtx,
        _event: &LifeCycle,
        _data: &(Roof, Boundary),
        _env: &Env,
    ) {

    }

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &(Roof, Boundary), _data: &(Roof, Boundary), _env: &Env) {

    }

    fn layout(
        &mut self,
        _layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &(Roof, Boundary),
        _env: &Env,
    ) -> Size {
        if bc.is_width_bounded() | bc.is_height_bounded() {
            let size = Size::new(120.0, 80.0);
            bc.constrain(size)
        } else {
            bc.max()
        }
    }

    fn paint(&mut self, ctx: &mut PaintCtx, tuple: &(Roof, Boundary), _env: &Env) {
        let size = ctx.size();
        let scale = tuple.0.scale(size);
        let roof = tuple.0.scaled(scale);
        let effective_roof = tuple.0.effective_roof(tuple.1).scaled(scale);

        let mut roof_path = roof.shape();
        let mut effective_roof_path = effective_roof.shape();

        let half_roof_spare = size - Size::new(roof.get_eaves(), roof.get_height());
        roof_path.apply_affine(Affine::translate(half_roof_spare.to_vec2() * 0.5));

        let half_effective_roof_spare = size - Size::new(effective_roof.get_eaves(), effective_roof.get_height());
        effective_roof_path.apply_affine(Affine::translate(half_effective_roof_spare.to_vec2() * 0.5));

        let boundary_shift = Point::new(
            roof.horizontal_shift(tuple.1), tuple.1.get_top() - tuple.1.get_bottom()
        );
        effective_roof_path.apply_affine(Affine::translate(boundary_shift.to_vec2() * scale * 0.5));

        ctx.fill(roof_path, &Color::grey8(100));
        ctx.fill(effective_roof_path, &Color::rgb8(145, 105, 21));
    }
}

pub fn create_boundary_forms_widget() -> impl Widget<Boundary> {
    Flex::column()
        .cross_axis_alignment(druid::widget::CrossAxisAlignment::End)
        .with_child(
            Flex::row()
                .cross_axis_alignment(druid::widget::CrossAxisAlignment::Baseline)
                .with_child(Label::new("Top"))
                .with_default_spacer()
                .with_child(
                    TextBox::new()
                        .with_formatter(SiFormatter::MILLIMETERS)
                        .lens(Boundary::top)))
        .with_default_spacer()
        .with_child(
            Flex::row()
                .cross_axis_alignment(druid::widget::CrossAxisAlignment::Baseline)
                .with_child(Label::new("Left"))
                .with_default_spacer()
                .with_child(
                    TextBox::new()
                        .with_formatter(SiFormatter::MILLIMETERS)
                        .lens(Boundary::left)))
        .with_default_spacer()
        .with_child(
            Flex::row()
                .cross_axis_alignment(druid::widget::CrossAxisAlignment::Baseline)
                .with_child(Label::new("Right"))
                .with_default_spacer()
                .with_child(
                    TextBox::new()
                        .with_formatter(SiFormatter::MILLIMETERS)
                        .lens(Boundary::right)))
        .with_default_spacer()
        .with_child(
            Flex::row()
                .cross_axis_alignment(druid::widget::CrossAxisAlignment::Baseline)
                .with_child(Label::new("Bottom"))
                .with_default_spacer()
                .with_child(
                    TextBox::new()
                        .with_formatter(SiFormatter::MILLIMETERS)
                        .lens(Boundary::bottom)))
}

pub fn create_boundary_widget() -> impl Widget<State> {
    Flex::row()
        .with_child(create_boundary_forms_widget().lens(State::boundary))
        .with_default_spacer()
        .with_child(
            SizedBox::new(BoundaryGraphics {})
                .width(120.0)
                .height(80.0)
                .lens(RoofBoundaryTupleLens{})
        )
        .main_axis_alignment(MainAxisAlignment::End)
        .fix_width(340.0)
}
