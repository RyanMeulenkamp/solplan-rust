use druid::{Widget, WidgetExt, EventCtx, Event, Env, LifeCycleCtx, LifeCycle, UpdateCtx, BoxConstraints, LayoutCtx, Size, PaintCtx, Color, RenderContext, Affine, Point, LensExt};
use crate::model::roof::Roof;
use druid::widget::{Flex, MainAxisAlignment};
use crate::format::siformatter::SiFormatter;
use crate::model::boundary::Boundary;
use crate::model::state::State;
use druid::lens::Identity;
use crate::widget::common::create_form_element;

pub struct BoundaryGraphics;

impl Widget<(Roof, Boundary)> for BoundaryGraphics {
    fn event(&mut self, _: &mut EventCtx, _: &Event, _: &mut (Roof, Boundary), _: &Env) {}
    fn lifecycle(&mut self, _: &mut LifeCycleCtx, _: &LifeCycle, _: &(Roof, Boundary), _: &Env, ) {}
    fn update(&mut self, _: &mut UpdateCtx, _: &(Roof, Boundary), _: &(Roof, Boundary), _: &Env) {}

    fn layout(&mut self, _: &mut LayoutCtx, bc: &BoxConstraints, _: &(Roof, Boundary), _: &Env) -> Size {
        if bc.is_width_bounded() | bc.is_height_bounded() {
            let size = Size::new(120.0, 80.0);
            bc.constrain(size)
        } else {
            bc.max()
        }
    }

    fn paint(&mut self, ctx: &mut PaintCtx, tuple: &(Roof, Boundary), _: &Env) {
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
            roof.horizontal_boundary(tuple.1), tuple.1.get_top() - tuple.1.get_bottom()
        );
        effective_roof_path.apply_affine(Affine::translate(boundary_shift.to_vec2() * scale * 0.5));

        ctx.fill(roof_path, &Color::grey8(100));
        ctx.fill(effective_roof_path, &Color::rgb8(145, 105, 21));
    }
}

fn create_boundary_forms_widget() -> impl Widget<Boundary> {
    Flex::column()
        .cross_axis_alignment(druid::widget::CrossAxisAlignment::End)
        .with_child(create_form_element("Top", SiFormatter::MILLIMETERS, Boundary::top))
        .with_child(create_form_element("Left", SiFormatter::MILLIMETERS, Boundary::left))
        .with_child(create_form_element("Right", SiFormatter::MILLIMETERS, Boundary::right))
        .with_child(create_form_element("Bottom", SiFormatter::MILLIMETERS, Boundary::bottom))
}

pub fn create_boundary_widget() -> impl Widget<State> {
    Flex::row()
        .with_child(create_boundary_forms_widget().lens(State::boundary))
        .with_default_spacer()
        .with_child(
            BoundaryGraphics {}
                .padding((0.0, 0.0, 0.0, 10.0))
                .fix_size(120.0, 80.0)
                .lens(Identity.map(
                    |state: &State| (state.get_roof(), state.get_boundary()),
                    |_: &mut State, _: (Roof, Boundary)| ()
                ))
        )
        .main_axis_alignment(MainAxisAlignment::End)
        .fix_width(340.0)
}
