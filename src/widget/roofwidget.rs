use druid::{Widget, WidgetExt, EventCtx, Event, Env, LifeCycleCtx, LifeCycle, UpdateCtx, BoxConstraints, LayoutCtx, Size, PaintCtx, Color, RenderContext, Affine};
use crate::model::roof::Roof;
use druid::widget::{Flex, Label, TextBox, SizedBox, MainAxisAlignment};
use crate::format::distance::SiFormatter;

struct RoofGraphics;

impl Widget<Roof> for RoofGraphics {
    fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, _data: &mut Roof, _env: &Env) {

    }

    fn lifecycle(
        &mut self,
        _ctx: &mut LifeCycleCtx,
        _event: &LifeCycle,
        _data: &Roof,
        _env: &Env,
    ) {

    }

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &Roof, _data: &Roof, _env: &Env) {

    }

    fn layout(
        &mut self,
        _layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _roof: &Roof,
        _env: &Env,
    ) -> Size {
        if bc.is_width_bounded() | bc.is_height_bounded() {
            bc.constrain(Size::new(120.0, 80.0))
        } else {
            bc.max()
        }
    }

    fn paint(&mut self, ctx: &mut PaintCtx, roof: &Roof, _env: &Env) {
        let size = ctx.size();
        let scaled_roof = roof.scaled_to_size(size);
        let half_spare = (size - Size::new(scaled_roof.get_eaves(), scaled_roof.get_height())) / 2.0;

        let mut path = scaled_roof.shape();

        path.apply_affine(Affine::translate(half_spare.to_vec2()));

        ctx.fill(path, &Color::grey8(100));
    }
}

pub fn create_roof_forms_widget() -> impl Widget<Roof> {
    Flex::column()
        .cross_axis_alignment(druid::widget::CrossAxisAlignment::End)
        .with_child(
            Flex::row()
                .cross_axis_alignment(druid::widget::CrossAxisAlignment::Baseline)
                .with_child(Label::new("Ridge"))
                .with_default_spacer()
                .with_child(
                    TextBox::new()
                        .with_formatter(SiFormatter::MILLIMETERS)
                        .lens(Roof::ridge)
                )
        )
        .with_default_spacer()
        .with_child(
            Flex::row()
                .cross_axis_alignment(druid::widget::CrossAxisAlignment::Baseline)
                .with_child(Label::new("Eaves"))
                .with_default_spacer()
                .with_child(
                    TextBox::new()
                        .with_formatter(SiFormatter::MILLIMETERS)
                        .lens(Roof::eaves)
                )
        )
        .with_default_spacer()
        .with_child(
            Flex::row()
                .cross_axis_alignment(druid::widget::CrossAxisAlignment::Baseline)
                .with_child(Label::new("Height"))
                .with_default_spacer()
                .with_child(
                    TextBox::new()
                        .with_formatter(SiFormatter::MILLIMETERS)
                        .lens(Roof::height)
                )
        )
}

pub fn create_roof_widget() -> impl Widget<Roof> {
    Flex::row()
        .with_child(create_roof_forms_widget())
        .with_default_spacer()
        .with_child(SizedBox::new(RoofGraphics {}).width(120.0).height(80.0))
        .main_axis_alignment(MainAxisAlignment::End)
        .fix_width(330.0)
}
