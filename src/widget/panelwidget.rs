use druid::{Widget, WidgetExt, EventCtx, Event, Env, LifeCycleCtx, LifeCycle, UpdateCtx, BoxConstraints, LayoutCtx, Size, PaintCtx, Color, RenderContext, theme};
use druid::widget::{Flex, Label, TextBox, Scroll, List, Button, Checkbox, Either};
use crate::format::siformatter::SiFormatter;
use crate::model::panel::Panel;
use druid::widget::CrossAxisAlignment::Baseline;
use druid::im::Vector;
use crate::model::state::State;
use crate::format::currencyformatter::CurrencyFormatter;

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
            bc.constrain(Size::new(150.0, 110.0))
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

fn create_panel_forms_widget() -> impl Widget<Panel> {
    Flex::column()
        .cross_axis_alignment(druid::widget::CrossAxisAlignment::End)
        .with_child(
            Flex::row()
                .cross_axis_alignment(Baseline)
                .with_default_spacer()
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
                .cross_axis_alignment(Baseline)
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
                .cross_axis_alignment(Baseline)
                .with_child(Label::new("Power"))
                .with_default_spacer()
                .with_child(
                    TextBox::new()
                        .with_formatter(SiFormatter::WATT_PEAK)
                        .lens(Panel::peak_power)
                )
        )
        .with_default_spacer()
        .with_child(
            Flex::row()
                .cross_axis_alignment(Baseline)
                .with_child(Label::new("Price"))
                .with_default_spacer()
                .with_child(
                    TextBox::new()
                        .with_formatter(CurrencyFormatter::EUROS)
                        .lens(Panel::price)
                )
        )
}

fn create_single_panel_widget() -> impl Widget<Panel> {
    Flex::column()
        .with_default_spacer()
        .with_child(
            Flex::row()
                .with_default_spacer()
                .with_child(Label::new("Type"))
                .with_default_spacer()
                .with_child(
                    TextBox::new()
                        .fix_width(240.0)
                        .lens(Panel::name)
                )
                .with_default_spacer()
                .with_child(
                    Checkbox::new("")
                        .lens(Panel::selected)
                )
                .align_right()

        )
        .with_default_spacer()
        .with_child(
            Flex::row()
                .with_child(create_panel_forms_widget())
                .with_default_spacer()
                .with_flex_child(PanelGraphics {}, 1.0)
                .with_default_spacer()
        )
        .with_default_spacer()
        .border(theme::PLACEHOLDER_COLOR, 0.5)
        .fix_width(340.0)
}

pub fn create_controls_widget() -> impl Widget<State> {
    Flex::row()
        .with_child(
            Button::new("Add")
                .on_click(|_ctx, data: &mut State, _env| {
                    let mut panels = data.get_panels().clone();
                    panels.insert(0, Panel::new(
                        "type", 1684.0, 1002.0, 335.0, 100.0
                    ));
                    data.set_panels(panels);
                }),
        )
        .with_flex_spacer(1.0)
        .with_child(
            Either::new(
                |state: &State, _| state.get_panels().iter().any(|panel| panel.is_selected()),
                Button::new("Del")
                    .on_click(|_ctx, data: &mut State, _env| {
                        let mut new_data: Vector<Panel> = Vector::new();
                        for panel in data.get_panels().iter() {
                            if !panel.is_selected() {
                                new_data.push_back(panel.clone());
                            }
                        }
                        data.set_panels(new_data);
                    }),
                Label::new("Select panels to delete them")
            )
        )
        .padding(10.0)
        .fix_width(340.0)
}

pub fn create_panels_widget() -> impl Widget<Vector<Panel>> {
    Scroll::new(List::new(|| create_single_panel_widget()))
        .vertical()
        .fix_width(340.0)
        .expand_height()
}
