fn main() -> ::std::io::Result<()> {
    let mut image = ray_tracing::Image::new(512, 512);
    let (width, height) = (image.width(), image.height());

    image.generate(|i, j| {
        let r = i as f64 / (width - 1) as f64;
        let g = j as f64 / (height - 1) as f64;
        let b = 0f64;

        ray_tracing::Pixel::from((r, g, b))
    });

    image.output(
        &mut ::std::io::stdout().lock(),
        &mut ::std::io::stderr().lock(),
    )?;

    Ok(())
}
