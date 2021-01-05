use druid::{Widget, WidgetExt, EventCtx, Event, Env, LifeCycleCtx, LifeCycle, UpdateCtx, BoxConstraints, LayoutCtx, Size, PaintCtx, Color, RenderContext, Affine};
use crate::model::roof::Roof;
use druid::widget::{Flex, MainAxisAlignment};
use crate::format::siformatter::SiFormatter;
use crate::widget::common::create_form_element;

struct RoofGraphics;

impl Widget<Roof> for RoofGraphics {
    fn event(&mut self, _: &mut EventCtx, _: &Event, _: &mut Roof, _: &Env) {}
    fn lifecycle(&mut self, _: &mut LifeCycleCtx, _: &LifeCycle, _: &Roof, _: &Env) {}
    fn update(&mut self, _: &mut UpdateCtx, _: &Roof, _: &Roof, _: &Env) {}
    fn layout(&mut self, _: &mut LayoutCtx, bc: &BoxConstraints, _: &Roof, _: &Env) -> Size {
        if bc.is_width_bounded() | bc.is_height_bounded() {
            bc.constrain(Size::new(120.0, 80.0))
        } else {
            bc.max()
        }
    }

    fn paint(&mut self, ctx: &mut PaintCtx, roof: &Roof, _: &Env) {
        let size = ctx.size();
        let scaled_roof = roof.scaled_to_size(size);
        let half_spare = (size - Size::new(scaled_roof.get_eaves(), scaled_roof.get_height())) / 2.0;

        let mut path = scaled_roof.shape();

        path.apply_affine(Affine::translate(half_spare.to_vec2()));

        ctx.fill(path, &Color::grey8(100));
    }
}

fn create_roof_forms_widget() -> impl Widget<Roof> {
    Flex::column()
        .cross_axis_alignment(druid::widget::CrossAxisAlignment::End)
        .with_child(create_form_element("Ridge", SiFormatter::MILLIMETERS, Roof::ridge))
        .with_child(create_form_element("Eaves", SiFormatter::MILLIMETERS, Roof::eaves))
        .with_child(create_form_element("Height", SiFormatter::MILLIMETERS, Roof::height))
}

pub fn create_roof_widget() -> impl Widget<Roof> {
    Flex::row()
        .with_child(create_roof_forms_widget())
        .with_default_spacer()
        .with_child(RoofGraphics {}.fix_size(120.0, 80.0))
        .main_axis_alignment(MainAxisAlignment::End)
        .fix_width(340.0)
}
