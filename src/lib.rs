use druid::{AppLauncher, Data, LensExt, LocalizedString, theme, Widget, WidgetExt, WindowDesc, AppDelegate, WindowId, Env, DelegateCtx};
use druid::im::Vector;
use druid::lens::Identity;
use druid::widget::{Flex, Label};
use wasm_bindgen::prelude::*;

use crate::model::plan::Plan;
use crate::model::state::State;
use crate::widget::{boundarywidget::create_boundary_widget, roofwidget::create_roof_widget};
use crate::widget::clearancewidget::create_clearance_widget;
use crate::widget::constraintswidget::create_constraints_widget;
use crate::widget::panelwidget::{create_controls_widget, create_panels_widget};
use crate::widget::planwidget::create_plan_widget;
use crate::util::configuration::{write_config, read_config};

pub mod format;
pub mod model;
pub mod view;
pub mod widget;
pub mod algorithm;
pub mod util;

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

struct Delegate;

impl AppDelegate<State> for Delegate {
    fn window_removed(&mut self, _id: WindowId, data: &mut State, _env: &Env, _ctx: &mut DelegateCtx) {
        write_config(data)
    }
}


#[wasm_bindgen]
pub fn app() {
    console_error_panic_hook::set_once();

    AppLauncher::with_window(WindowDesc::new(build_app).title(LocalizedString::new("Solplan")))
        .delegate(Delegate{})
        .use_simple_logger()
        .launch(read_config())
        .expect("launch failed");
}
