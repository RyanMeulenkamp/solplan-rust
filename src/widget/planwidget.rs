use druid::{BoxConstraints, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx, Size, UpdateCtx, Widget, RenderContext, Color, WidgetExt, theme, UnitPoint};

use crate::model::plan::Plan;
use crate::widget::boundarywidget::BoundaryGraphics;
use druid::widget::{Scroll, Flex, Label, List};
use druid::im::Vector;

pub struct PlanGraphics {
    boundary_graphics: BoundaryGraphics
}

impl PlanGraphics {
    pub fn new() -> Self {
        PlanGraphics { boundary_graphics: BoundaryGraphics{} }
    }
}

impl Widget<Plan> for PlanGraphics {
    fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, _data: &mut Plan, _env: &Env) {

    }

    fn lifecycle(
        &mut self,
        _ctx: &mut LifeCycleCtx,
        _event: &LifeCycle,
        _data: &Plan,
        _env: &Env,
    ) {

    }

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &Plan, _data: &Plan, _env: &Env) {

    }

    fn layout(
        &mut self,
        _layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        plan: &Plan,
        _env: &Env,
    ) -> Size {
        if bc.is_width_bounded() | bc.is_height_bounded() {
            bc.constrain_aspect_ratio(plan.get_roof().aspect_ratio(), bc.max().width)
        } else {
            bc.max()
        }
    }

    fn paint(&mut self, ctx: &mut PaintCtx, plan: &Plan, env: &Env) {
        let size = ctx.size();
        let scale = plan.get_roof().scale(size);

        self.boundary_graphics.paint(ctx, &(plan.get_roof(), plan.get_boundary()), env);

        let roof = plan.get_roof().scaled(scale);
        let boundary = plan.get_boundary().scaled(scale);
        let panel = plan.get_panels().scaled(scale);
        let clearance = plan.get_clearance().scaled(scale);

        let panel_shape = panel.shape();
        let start_y = (size.height - roof.get_height()) * 0.5 + roof.get_height() - boundary.get_bottom() - panel.get_height();

        for (row, cols) in plan.get_layout().iter().rev().map(|row| *row as f64).enumerate() {
            let width = cols * panel.get_width() + (cols - 1.0) * clearance.get_vertical();
            let start_x = (size.width - width + roof.horizontal_shift(boundary)) * 0.5;
            for col in 0..cols as i32 {
                let col = col as f64;
                ctx.fill(
                    panel_shape.with_origin((
                        start_x + col * (panel.get_width() + clearance.get_vertical()),
                        start_y - row as f64 * (panel.get_height() + clearance.get_horizontal())
                    )),
                    &Color::rgb8(45, 58, 148)
                );
            }
        }
    }
}

pub fn plan() -> impl Widget<Plan> {
    Flex::column()
        .with_child(
            Label::new(|plan: &Plan, _env: &_|
                format!(
                    "{} x {} (Area = {:.2}m²; DC power = {:.1}KWp)  Total est. price: {:.2}€",
                    plan.get_total_panels(), plan.get_panels().get_name(), plan.get_total_area(),
                    plan.get_total_dc_power() / 1000.0, plan.get_total_price()
                )
            )
                .background(theme::PLACEHOLDER_COLOR)
                .expand_width()
        )
        .with_child(
            PlanGraphics::new()
                .padding(10.0)
                .border(theme::PLACEHOLDER_COLOR, 1.5)
        )
        .with_default_spacer()
}

pub fn create_plan_widget() -> impl Widget<Vector<Plan>> {
    Scroll::new(List::new(|| plan())).vertical().align_vertical(UnitPoint::TOP_LEFT)
}

