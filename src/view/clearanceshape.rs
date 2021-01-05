use crate::model::clearance::Clearance;
use druid::{Size, Rect};
use druid::kurbo::{BezPath, Shape};

impl Clearance {

    pub fn is_horizontal_leading(self, size: Size) -> bool {
        self.get_horizontal() / size.height > self.get_vertical() / size.width
    }

    pub fn scaled(self, scale: f64) -> Self {
        Clearance::new(self.get_vertical() * scale, self.get_horizontal() * scale)
    }

    pub fn fit_to_size(self, size: Size) -> Self {
        let horizontal: f64;
        let vertical: f64;

        if self.is_horizontal_leading(size) {
            horizontal = size.height / 3.0;
            vertical = (self.get_vertical() / self.get_horizontal()) * horizontal;
        } else {
            vertical = size.width / 3.0;
            horizontal = (self.get_horizontal() / self.get_vertical()) * vertical;
        }

        Self::new(vertical, horizontal)
    }

    pub fn shapes(self, size: Size) -> (BezPath, BezPath, BezPath, BezPath) {
        let scaled = self.fit_to_size(size);
        let rect = Rect::new(
            0.0, 0.0, (size.width - scaled.get_vertical()) * 0.5, (size.height - scaled.get_horizontal()) * 0.5
        );
        (
            rect.to_path(0.0),
            rect.with_origin((0.0, rect.height() + scaled.get_horizontal())).to_path(0.0),
            rect.with_origin((rect.width() + scaled.get_vertical(), 0.0)).to_path(0.0),
            rect.with_origin((rect.width() + scaled.get_vertical(), rect.height() + scaled.get_horizontal())).to_path(0.0),
        )
    }
}
