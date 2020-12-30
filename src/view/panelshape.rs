use crate::model::panel::Panel;
use druid::{Size, Rect};

impl Panel {
    pub fn scale(&self, size: Size) -> f64 {
        let width_ratio = size.width / self.get_width();
        let height_ratio = size.height / self.get_height();
        if width_ratio > height_ratio {
            height_ratio
        } else {
            width_ratio
        }
    }

    pub fn scaled(&self, scale: f64) -> Self {
        Self::new(
            self.get_name(), self.get_width() * scale, self.get_height() * scale,
            self.get_peak_power(), self.get_price()
        )
    }

    pub fn scaled_to_size(&self, size: Size) -> Self {
        self.scaled(self.scale(size))
    }

    pub fn shape(&self) -> Rect {
        Rect::new(0.0, 0.0, self.get_width(), self.get_height())
    }
}
