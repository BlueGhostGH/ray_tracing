const ASPECT_RATIO: f64 = 16f64 / 9f64;

const IMAGE_WIDTH: usize = 400;
const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;

const VIEWPORT_HEIGHT: f64 = 2f64;
const VIEWPORT_WIDTH: f64 = (IMAGE_WIDTH / IMAGE_HEIGHT) as f64 * VIEWPORT_HEIGHT;

fn main() -> ::std::io::Result<()> {
    ray_tracing::render(IMAGE_WIDTH, IMAGE_HEIGHT, VIEWPORT_WIDTH, VIEWPORT_HEIGHT)
}
