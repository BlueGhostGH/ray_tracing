#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub(crate) struct Viewport {
    height: f64,
    width: f64,
}

impl Viewport {
    pub(crate) fn new(height: f64, width: f64) -> Self {
        Viewport { height, width }
    }
}
