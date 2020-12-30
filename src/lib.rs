use druid::{AppLauncher, Data, LensExt, LocalizedString, theme, Widget, WidgetExt, WindowDesc};
use druid::im::{vector, Vector};
use druid::lens::Identity;
use druid::widget::{Flex, Label};
use wasm_bindgen::prelude::*;

use crate::model::boundary::Boundary;
use crate::model::clearance::Clearance;
use crate::model::constraints::Constraints;
use crate::model::panel::Panel;
use crate::model::plan::Plan;
use crate::model::roof::Roof;
use crate::model::state::State;
use crate::widget::{boundarywidget::create_boundary_widget, roofwidget::create_roof_widget};
use crate::widget::clearancewidget::create_clearance_widget;
use crate::widget::constraintswidget::create_constraints_widget;
use crate::widget::panelwidget::{create_add_remove_widget, create_panels_widget};
use crate::widget::planwidget::create_plan_widget;

pub mod format;
pub mod model;
pub mod view;
pub mod widget;
pub mod algorithm;

fn input_widget<T: Data>(title: &str, child: impl Widget<T> + 'static) -> impl Widget<T> {
    Flex::column()
        .with_child(
            Label::new(title)
                .background(theme::PLACEHOLDER_COLOR)
                .expand_width()
        )
        .with_child(
            child
                .padding(10.0)
                .border(theme::PLACEHOLDER_COLOR, 1.5)
        )
}

fn build_app() -> impl Widget<State> {
    Flex::row()
        .with_default_spacer()
        .with_child(
            Flex::column()
                .with_default_spacer()
                .with_child(input_widget(
                    "Roof",
                    create_roof_widget()
                        .lens(State::roof)
                ))
                .with_child(input_widget(
                    "Boundary",
                    create_boundary_widget()
                ))
                .with_default_spacer()
                .with_child(
                    Label::new("Panels")
                        .background(theme::PLACEHOLDER_COLOR)
                        .expand_width()
                )
                .with_child(
                    create_add_remove_widget()
                        .border(theme::PLACEHOLDER_COLOR, 1.5),
                )
                .with_flex_child(
                    create_panels_widget()
                        .lens(State::panels)
                        .border(theme::PLACEHOLDER_COLOR, 1.5),
                    1.0
                )
                .with_child(input_widget(
                    "Clearance",
                    create_clearance_widget()
                        .lens(State::clearance)
                ))
                .with_default_spacer()
                .with_child(input_widget(
                    "Constraints",
                    create_constraints_widget()
                        .lens(State::constraints)
                ))
                .with_default_spacer()
                .fix_width(343.0)
        )
        .with_flex_child(
            create_plan_widget()
                .lens(Identity.map(
                    |state: &State| state.get_plans(),
                    |_state: &mut State, _plans: Vector<Plan>| ()
                ))
                .padding(10.0),
            1.0
        )
}

#[wasm_bindgen]
pub fn app() {
    console_error_panic_hook::set_once();
    let window = WindowDesc::new(build_app).title(LocalizedString::new("Solplan"));

    let roof = Roof::new(9625.0, 13500.0, 7320.0);
    let boundary = Boundary::new(30.0, 200.0, 30.0, 30.0);
    let panels = vector![
        Panel::new("JinkoSolar JKM-335M-60H", 1684.0, 1002.0, 335.0),
        Panel::new("Sunpower Maxeon 3", 1690.0, 1046.0, 400.0),
    ];
    let clearance = Clearance::new(10.0, 10.0);
    let constraints = Constraints::new(true, true, false);
    let state = State::new(roof, boundary, panels, clearance, constraints);

    AppLauncher::with_window(window)
        .use_simple_logger()
        .launch(state)
        .expect("launch failed");
}
