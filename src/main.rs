fn main() -> ::std::io::Result<()> {
    let mut image = ray_tracing::Image::new(512, 512);
    let (width, height) = (image.width(), image.height());

    image.generate(|i, j| {
        let r = ((i as f64 / (width - 1) as f64) * 255f64) as u8;
        let g = ((j as f64 / (height - 1) as f64) * 255f64) as u8;
        let b = (0f64 * 255f64) as u8;

        ray_tracing::Pixel { r, g, b }
    });

    image.output(
        &mut ::std::io::stdout().lock(),
        &mut ::std::io::stderr().lock(),
    )?;

    Ok(())
}
