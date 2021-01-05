use crate::model::roof::Roof;
use druid::kurbo::BezPath;
use druid::Size;

impl Roof {
    pub fn scale(&self, size: Size) -> f64 {
        let width_ratio = size.width / self.get_eaves();
        let height_ratio = size.height / self.get_height();

        if width_ratio > height_ratio {
            height_ratio
        } else {
            width_ratio
        }
    }

    pub fn scaled(&self, scale: f64) -> Self {
        Self::new(
            self.get_ridge() * scale, self.get_eaves() * scale,
            self.get_height() * scale
        )
    }

    pub fn scaled_to_size(&self, size: Size) -> Self {
        self.scaled(self.scale(size))
    }

    pub fn shape(&self) -> BezPath {
        let mut path = BezPath::new();
        path.line_to((0.0, self.get_height()));
        let diff = (self.get_eaves() - self.get_ridge()) * 0.5;
        path.line_to((diff, 0.0));
        path.line_to((0.0 + self.get_eaves() - diff, 0.0));
        path.line_to((0.0 + self.get_eaves(), self.get_height()));
        path.line_to((0.0, self.get_height()));
        path
    }
}
