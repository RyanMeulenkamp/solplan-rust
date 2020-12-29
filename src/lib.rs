use wasm_bindgen::prelude::*;
use druid::{AppLauncher, LocalizedString, UnitPoint, WindowDesc, WidgetExt, Widget, theme};
use druid::widget::{Align, Flex, CrossAxisAlignment, Label, Scroll};

use crate::model::boundary::Boundary;
use crate::model::roof::Roof;
use crate::widget::{boundarywidget::create_boundary_widget, roofwidget::create_roof_widget};
use crate::model::panel::Panel;
use crate::widget::panelwidget::create_panel_widget;
use crate::widget::planwidget::create_plan_widget;
use crate::model::state::State;
use crate::model::clearance::Clearance;
use crate::model::constraints::Constraints;
use crate::widget::clearancewidget::create_clearance_widget;
use crate::widget::constraintswidget::create_constraints_widget;
use crate::lens::planlens::PlanLens;

pub mod format;
pub mod lens;
pub mod model;
pub mod view;
pub mod widget;
pub mod algorithm;

fn build_app() -> impl Widget<State> {
    Flex::row()
        .with_default_spacer()
        .with_child(
            Align::new(
                UnitPoint::TOP_LEFT,
                Scroll::new(
                    Flex::column()
                        .cross_axis_alignment(CrossAxisAlignment::End)
                        .with_default_spacer()
                        .with_child(
                            Label::new("Roof")
                                .background(theme::PLACEHOLDER_COLOR)
                                .expand_width()
                        )
                        .with_child(
                            create_roof_widget()
                                .lens(State::roof)
                                .padding(10.0)
                                .border(theme::PLACEHOLDER_COLOR, 1.5)
                        )
                        .with_child(
                            Label::new("Boundary")
                                .background(theme::PLACEHOLDER_COLOR)
                                .expand_width()
                        )
                        .with_child(
                            create_boundary_widget()
                                .padding(10.0)
                                .border(theme::PLACEHOLDER_COLOR, 1.5)
                        )
                        .with_default_spacer()
                        .with_child(
                            Label::new("Panel")
                                .background(theme::PLACEHOLDER_COLOR)
                                .expand_width()
                        )
                        .with_child(
                            create_panel_widget()
                                .lens(State::panel)
                                .padding(10.0)
                                .border(theme::PLACEHOLDER_COLOR, 1.5)
                        )
                        .with_child(
                            Label::new("Clearance")
                                .background(theme::PLACEHOLDER_COLOR)
                                .expand_width()
                        )
                        .with_child(
                            create_clearance_widget()
                                .lens(State::clearance)
                                .padding(10.0)
                                .border(theme::PLACEHOLDER_COLOR, 1.5)
                        )
                        .with_default_spacer()
                        .with_child(
                            Label::new("Constraints")
                                .background(theme::PLACEHOLDER_COLOR)
                                .expand_width()
                        )
                        .with_child(
                            create_constraints_widget()
                                .lens(State::constraints)
                                .padding(10.0)
                                .border(theme::PLACEHOLDER_COLOR, 1.5)
                        )
                )
                    .vertical()
                    .fix_width(343.0)
            )
        )
        .with_flex_child(
            Align::new(
                UnitPoint::TOP_LEFT,
                create_plan_widget()
                    .lens(PlanLens{})
                    .padding(10.0),
            ),
            1.0
        )
}

#[wasm_bindgen]
pub fn app() {
    AppLauncher::with_window(WindowDesc::new(build_app).title(LocalizedString::new("Solplan")))
        .use_simple_logger()
        .launch(State::new(
            Roof::new(9625.0, 13500.0, 7320.0),
            Boundary::new(30.0, 200.0, 30.0, 30.0),
            Panel::new(1684.0, 1002.0, 335.0),
            Clearance::new(10.0, 10.0),
            Constraints::new(true, true, false),
        ))
        .expect("launch failed");
}
