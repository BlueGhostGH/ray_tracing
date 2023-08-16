#[derive(Debug, Clone, Copy, Default, PartialEq, Hash)]
pub(crate) struct Vec3<C>
where
    C: Component,
{
    pub(crate) components: [C; 3],
}

pub(crate) trait Component {}

impl Component for f64 {}

impl<C> Vec3<C>
where
    C: Component,
{
    // fn new(x: F, y: F, z: F) -> Self {
    //     Vec3 { inner: [x, y, z] }
    // }
}
