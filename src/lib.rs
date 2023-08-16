#![feature(generic_const_exprs, get_many_mut)]
#![allow(incomplete_features)]

mod image;
mod pixel;
use image::Image;

mod camera;
mod ray;
use camera::Camera;
use ray::Ray;

mod vec3;
mod viewport;
use vec3::{Point3, Vec3};
use viewport::Viewport;

mod progress;

pub fn render(
    image_width: usize,
    image_height: usize,
    viewport_width: f64,
    viewport_height: f64,
) -> Result<(), ::std::io::Error> {
    let camera = Camera::new(
        1f64,
        Viewport::new(viewport_height, viewport_width),
        Vec3::from((0f64, 0f64, 0f64)),
    );

    let viewport_u = Vec3::from((viewport_width, 0f64, 0f64));
    let viewport_v = Vec3::from((0f64, -viewport_height, 0f64));

    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    let viewport_upper_left = camera.center
        - viewport_u / 2f64
        - viewport_v / 2f64
        - Vec3::from((0f64, 0f64, camera.focal_length));
    let pixel_00_location = viewport_upper_left + 0.5f64 * (pixel_delta_u + pixel_delta_v);

    let mut image = Image::new(image_width, image_height);

    image.generate(|i, j| {
        let pixel_center = pixel_00_location + i as f64 * pixel_delta_u + j as f64 * pixel_delta_v;
        let ray_direction = pixel_center - camera.center;
        let ray = Ray::new(camera.center, ray_direction);

        let pixel_color = ray.color();

        pixel_color
    });

    image.write_to_with_progress::<::std::io::StdoutLock<'_>, ::std::io::StderrLock<'_>, 20>(
        &mut ::std::io::stdout().lock(),
        &mut ::std::io::stderr().lock(),
    )?;

    Ok(())
}

mod const_generics {
    pub struct Assert<const CHECK: bool>;

    pub trait True {}

    impl True for Assert<true> {}
}
