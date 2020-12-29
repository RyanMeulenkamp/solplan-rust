use druid::{
    Widget, WidgetExt, EventCtx, Event, Env, LifeCycleCtx, LifeCycle, UpdateCtx, BoxConstraints,
    LayoutCtx, Size, PaintCtx, Color, RenderContext
};
use druid::widget::{Flex, Label, TextBox, SizedBox, MainAxisAlignment};
use crate::format::distance::SiFormatter;
use crate::model::panel::Panel;

struct PanelGraphics;

impl Widget<Panel> for PanelGraphics {
    fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, _data: &mut Panel, _env: &Env) {

    }

    fn lifecycle(
        &mut self,
        _ctx: &mut LifeCycleCtx,
        _event: &LifeCycle,
        _data: &Panel,
        _env: &Env,
    ) {

    }

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &Panel, _data: &Panel, _env: &Env) {

    }

    fn layout(
        &mut self,
        _layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _panel: &Panel,
        _env: &Env,
    ) -> Size {
        if bc.is_width_bounded() | bc.is_height_bounded() {
            bc.constrain(Size::new(120.0, 80.0))
        } else {
            bc.max()
        }
    }

    fn paint(&mut self, ctx: &mut PaintCtx, panel: &Panel, _env: &Env) {
        let size = ctx.size();
        let scaled_panel = panel.scaled_to_size(size);
        let half_spare = (size - Size::new(scaled_panel.get_width(), scaled_panel.get_height())) / 2.0;

        ctx.fill(
            scaled_panel.shape().with_origin((half_spare.width, half_spare.height)),
            &Color::rgb8(45, 58, 148)
        );
    }
}

pub fn create_panel_forms_widget() -> impl Widget<Panel> {
    Flex::column()
        .cross_axis_alignment(druid::widget::CrossAxisAlignment::End)
        .with_child(
            Flex::row()
                .cross_axis_alignment(druid::widget::CrossAxisAlignment::Baseline)
                .with_child(Label::new("Width"))
                .with_default_spacer()
                .with_child(
                    TextBox::new()
                        .with_formatter(SiFormatter::MILLIMETERS)
                        .lens(Panel::width)
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
                        .lens(Panel::height)
                )
        )
        .with_default_spacer()
        .with_child(
            Flex::row()
                .cross_axis_alignment(druid::widget::CrossAxisAlignment::Baseline)
                .with_child(Label::new("Power"))
                .with_default_spacer()
                .with_child(
                    TextBox::new()
                        .with_formatter(SiFormatter::WATT_PEAK)
                        .lens(Panel::peak_power)
                )
        )
}

pub fn create_panel_widget() -> impl Widget<Panel> {
    Flex::row()
        .with_child(create_panel_forms_widget())
        .with_default_spacer()
        .with_child(SizedBox::new(PanelGraphics {}).width(120.0).height(80.0))
        .main_axis_alignment(MainAxisAlignment::End)
        .fix_width(340.0)
}
