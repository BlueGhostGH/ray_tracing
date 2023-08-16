#![feature(generic_const_exprs, get_many_mut)]
#![allow(incomplete_features)]

mod image;
mod pixel;
pub use image::Image;
pub use pixel::Pixel;

mod vec3;

mod progress;

mod const_generics {
    pub(crate) struct Assert<const CHECK: bool>;

    pub(crate) trait True {}

    impl True for Assert<true> {}
}
