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
use crate::widget::panelwidget::{create_controls_widget, create_panels_widget};
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
                    create_controls_widget()
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

    AppLauncher::with_window(WindowDesc::new(build_app).title(LocalizedString::new("Solplan")))
        .use_simple_logger()
        .launch(State::new(
            Roof::new(9625.0, 13500.0, 7320.0),
            Boundary::new(30.0, 200.0, 30.0, 30.0),
            vector![
                Panel::new("JinkoSolar JKM320M-60HB",   1692.0, 1029.0, 315.0, 315.0 / 3.0),
                Panel::new("JinkoSolar JKM325M-60H(B)", 1684.0, 1002.0, 325.0, 325.0 / 3.0),
                Panel::new("JinkoSolar JKM330M-60H(B)", 1684.0, 1002.0, 330.0, 330.0 / 3.0),
                Panel::new("JinkoSolar JKM335M-60H(B)", 1684.0, 1002.0, 335.0, 335.0 / 3.0),
                Panel::new("JinkoSolar JKM340M-60H(B)", 1684.0, 1002.0, 340.0, 340.0 / 3.0),
                Panel::new("JinkoSolar JKM345M-60H",    1684.0, 1002.0, 345.0, 345.0 / 3.0),

                Panel::new("JinkoSolar JKM345M-66HB",   1841.0, 1002.0, 345.0, 345.0 / 3.0),
                Panel::new("JinkoSolar JKM350M-66HB",   1841.0, 1002.0, 350.0, 350.0 / 3.0),
                Panel::new("JinkoSolar JKM355M-66H(B)", 1841.0, 1002.0, 355.0, 355.0 / 3.0),
                Panel::new("JinkoSolar JKM360M-66H(B)", 1841.0, 1002.0, 360.0, 360.0 / 3.0),
                Panel::new("JinkoSolar JKM365M-66H(B)", 1841.0, 1002.0, 365.0, 365.0 / 3.0),
                Panel::new("JinkoSolar JKM370M-66H",    1841.0, 1002.0, 370.0, 370.0 / 3.0),
                Panel::new("JinkoSolar JKM375M-66H",    1841.0, 1002.0, 375.0, 375.0 / 3.0),

                Panel::new("JinkoSolar JKM390M-72H", 2008.0, 1002.0, 390.0, 390.0 / 3.0),
                Panel::new("JinkoSolar JKM395M-72H", 2008.0, 1002.0, 395.0, 395.0 / 3.0),
                Panel::new("JinkoSolar JKM400M-72H", 2008.0, 1002.0, 400.0, 400.0 / 3.0),
                Panel::new("JinkoSolar JKM405M-72H", 2008.0, 1002.0, 405.0, 405.0 / 3.0),
                Panel::new("JinkoSolar JKM410M-72H", 2008.0, 1002.0, 410.0, 410.0 / 3.0),

                Panel::new("JinkoSolar JKM425M-78H", 2166.0, 1002.0, 425.0, 425.0 / 3.0),
                Panel::new("JinkoSolar JKM430M-78H", 2166.0, 1002.0, 430.0, 430.0 / 3.0),
                Panel::new("JinkoSolar JKM435M-78H", 2166.0, 1002.0, 435.0, 435.0 / 3.0),
                Panel::new("JinkoSolar JKM440M-78H", 2166.0, 1002.0, 440.0, 440.0 / 3.0),
                Panel::new("JinkoSolar JKM445M-78H", 2166.0, 1002.0, 445.0, 445.0 / 3.0),

                Panel::new("JinkoSolar JKM375N-6RL3", 1855.0, 1029.0, 375.0, 375.0 / 3.0),
                Panel::new("JinkoSolar JKM370N-6RL3", 1855.0, 1029.0, 370.0, 370.0 / 3.0),
                Panel::new("JinkoSolar JKM385N-6RL3", 1855.0, 1029.0, 385.0, 385.0 / 3.0),
                Panel::new("JinkoSolar JKM380N-6RL3", 1855.0, 1029.0, 380.0, 380.0 / 3.0),
                Panel::new("JinkoSolar JKM395N-6RL3", 1855.0, 1029.0, 395.0, 395.0 / 3.0),

                Panel::new("JinkoSolar JKM450N-7RL3", 2182.0, 1029.0, 450.0, 450.0 / 3.0),
                Panel::new("JinkoSolar JKM455N-7RL3", 2182.0, 1029.0, 455.0, 455.0 / 3.0),
                Panel::new("JinkoSolar JKM460N-7RL3", 2182.0, 1029.0, 460.0, 460.0 / 3.0),
                Panel::new("JinkoSolar JKM465N-7RL3", 2182.0, 1029.0, 465.0, 465.0 / 3.0),
                Panel::new("JinkoSolar JKM470N-7RL3", 2182.0, 1029.0, 470.0, 470.0 / 3.0),

                Panel::new("Sunpower X22-345", 1559.0, 1046.0, 345.0, 345.0 / 3.0),
                Panel::new("Sunpower X22-360", 1559.0, 1046.0, 360.0, 360.0 / 3.0),
                Panel::new("Sunpower X22-370", 1559.0, 1046.0, 370.0, 370.0 / 3.0),

                Panel::new("Sunpower MAX2-340", 1690.0, 1046.0, 340.0, 340.0 / 3.0),
                Panel::new("Sunpower MAX2-350", 1690.0, 1046.0, 350.0, 350.0 / 3.0),
                Panel::new("Sunpower MAX2-360", 1690.0, 1046.0, 360.0, 360.0 / 3.0),

                Panel::new("Sunpower MAX3-355", 1690.0, 1046.0, 355.0, 390.0 / 3.0),
                Panel::new("Sunpower MAX3-370", 1690.0, 1046.0, 370.0, 395.0 / 3.0),
                Panel::new("Sunpower MAX3-375", 1690.0, 1046.0, 375.0, 400.0 / 3.0),

                Panel::new("Sunpower MAX3-390", 1690.0, 1046.0, 390.0, 390.0 / 3.0),
                Panel::new("Sunpower MAX3-395", 1690.0, 1046.0, 395.0, 395.0 / 3.0),
                Panel::new("Sunpower MAX3-400", 1690.0, 1046.0, 400.0, 400.0 / 3.0),

                Panel::new("Sunpower P3-315", 1690.0, 1046.0, 315.0, 315.0 / 3.0),
                Panel::new("Sunpower P3-320", 1690.0, 1046.0, 320.0, 320.0 / 3.0),
                Panel::new("Sunpower P3-325", 1690.0, 1046.0, 325.0, 325.0 / 3.0),
                Panel::new("Sunpower P3-330", 1690.0, 1046.0, 330.0, 330.0 / 3.0),
                Panel::new("Sunpower P3-335", 1690.0, 1046.0, 335.0, 335.0 / 3.0),
            ],
            Clearance::new(10.0, 10.0),
            Constraints::new(true, true, false, 30000.0),
        ))
        .expect("launch failed");
}
