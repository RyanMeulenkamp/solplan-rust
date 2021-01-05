use druid::{Widget, WidgetExt, EventCtx, Event, Env, LifeCycleCtx, LifeCycle, UpdateCtx, BoxConstraints, LayoutCtx, Size, PaintCtx, Color, RenderContext, theme, LensExt};
use druid::widget::{Flex, Label, TextBox, Scroll, List, Button, Checkbox, Either};
use crate::format::siformatter::SiFormatter;
use crate::model::panel::Panel;
use druid::im::Vector;
use crate::model::state::State;
use crate::format::currencyformatter::CurrencyFormatter;
use druid::lens::Identity;
use crate::widget::common::create_form_element;

struct PanelGraphics;

impl Widget<Panel> for PanelGraphics {
    fn event(&mut self, _: &mut EventCtx, _: &Event, _: &mut Panel, _: &Env) {}
    fn lifecycle(&mut self, _: &mut LifeCycleCtx, _: &LifeCycle, _: &Panel, _: &Env) {}
    fn update(&mut self, _: &mut UpdateCtx, _: &Panel, _: &Panel, _: &Env) {}

    fn layout(&mut self, _: &mut LayoutCtx, bc: &BoxConstraints, _: &Panel, _: &Env) -> Size {
        if bc.is_width_bounded() | bc.is_height_bounded() {
            bc.constrain(Size::new(150.0, 110.0))
        } else {
            bc.max()
        }
    }

    fn paint(&mut self, ctx: &mut PaintCtx, panel: &Panel, _: &Env) {
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
        .with_child(create_form_element("Width", SiFormatter::MILLIMETERS, Panel::width))
        .with_child(create_form_element("Height", SiFormatter::MILLIMETERS, Panel::height))
        .with_child(create_form_element("Power", SiFormatter::WATT_PEAK, Panel::peak_power))
        .with_child(create_form_element("Price", CurrencyFormatter::EUROS, Panel::price))
}

pub fn select_status(filter: &String, panels: Vector<Panel>) -> bool {
    let filtered = panels.iter()
        .filter(|panel| panel.get_name().contains(filter.as_str()))
        .collect::<Vector<&Panel>>();
    filtered.iter().any(|panel| panel.is_selected())
}

pub fn create_controls_widget() -> impl Widget<State> {
    Flex::row()
        .with_child(
            Button::new("Add")
                .on_click(|_ctx, data: &mut State, _env| {
                    let mut panels = data.get_panels().clone();
                    panels.insert(0, Panel::new("type", 1684.0, 1002.0, 335.0, 100.0));
                    data.set_panels(panels);
                }),
        )
        .with_default_spacer()
        .with_child(
            Button::new(
                |state: &State, _env: &Env| {
                    if select_status(&state.get_filter(), state.get_panels()) {
                        "Deselect all".to_string()
                    } else {
                        "Select all".to_string()
                    }
                })
                .on_click(|_ctx, state: &mut State, _env| {
                    let filter = state.get_filter();
                    let panels = state.get_panels();
                    let select = !select_status(&filter, panels.clone());

                    state.set_panels(
                        panels.into_iter()
                            .map(|panel| {
                                if panel.get_name().contains(filter.as_str()) {
                                    panel.with_selection(select)
                                } else {
                                    panel
                                }
                            })
                            .collect::<Vector<Panel>>()
                    );
                })
        )
        .with_default_spacer()
        .with_flex_child(
            TextBox::new()
                .expand_width()
                .lens(State::filter),
            1.0
        )
        .with_default_spacer()
        .with_child(
            Button::new("Del")
                .on_click(|_ctx, data: &mut State, _env| {
                    data.set_panels(
                        data.get_panels().into_iter()
                            .filter(|panel| !panel.is_selected())
                            .collect::<Vector<Panel>>()
                    );
                }),
        )
        .padding(10.0)
        .fix_width(340.0)
}

fn create_single_panel_widget() -> impl Widget<(String, Panel)> {
    Either::new(
        |(filter, panel): &(String, Panel), _| panel.get_name().contains(filter.as_str()),
        Flex::column()
            .with_default_spacer()
            .with_child(
                Flex::row()
                    .with_default_spacer()
                    .with_child(Label::new("Type"))
                    .with_default_spacer()
                    .with_child(TextBox::new().fix_width(240.0).lens(Panel::name))
                    .with_default_spacer()
                    .with_child(Checkbox::new("").lens(Panel::selected))
                    .align_right()

            )
            .with_default_spacer()
            .with_child(
                Flex::row()
                    .with_child(create_panel_forms_widget())
                    .with_default_spacer()
                    .with_flex_child(PanelGraphics {}.padding((0.0, 0.0, 0.0, 10.0)), 1.0)
                    .with_default_spacer()
            )
            .border(theme::PLACEHOLDER_COLOR, 0.5)
            .fix_width(340.0)
            .lens(Identity.map(
                |(_name, panel): &(String, Panel)| panel.clone(),
                |tuple: &mut (String, Panel), panel: Panel| tuple.1 = panel
            )),
        Flex::row()
    )
}

pub fn create_panels_widget() -> impl Widget<State> {
    Scroll::new(List::new(|| create_single_panel_widget()))
        .vertical()
        .fix_width(340.0)
        .expand_height()
        .lens(Identity.map(
            |state: &State| (state.get_filter(), state.get_panels()),
            |state: &mut State, (_, panels): (String, Vector<Panel>)| state.set_panels(panels)
        ))
}
