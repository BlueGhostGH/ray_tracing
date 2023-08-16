#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub(crate) struct Camera {
    pub(crate) focal_length: f64,

    viewport: crate::Viewport,

    pub(crate) center: crate::Point3,
}

impl Camera {
    pub(crate) fn new(focal_length: f64, viewport: crate::Viewport, center: crate::Point3) -> Self {
        Camera {
            focal_length,
            viewport,
            center,
        }
    }
}
