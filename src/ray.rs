use crate::pixel;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub(crate) struct Ray {
    origin: crate::Point3,
    direction: crate::Vec3,
}

impl Ray {
    pub(crate) fn new(origin: crate::Point3, direction: crate::Vec3) -> Self {
        Ray { origin, direction }
    }

    pub(crate) fn color(&self) -> pixel::Color {
        let unit_direction = self.direction.unit_vector();
        let t = 0.5f64 * (unit_direction.y() + 1f64);

        (1f64 - t) * crate::Vec3::from((1f64, 1f64, 1f64))
            + t * crate::Vec3::from((0.5f64, 0.7f64, 1f64))
    }
}
