use ::std::io::Write;

use crate::{pixel, progress};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Image {
    stride: usize,
    buffer: Box<[pixel::Pixel]>,
}

impl Image {
    pub fn new(width: usize, height: usize) -> Self {
        let mut buffer = Vec::with_capacity(width * height);
        buffer.resize_with(buffer.capacity(), pixel::Pixel::default);

        Image {
            stride: width,
            buffer: buffer.into_boxed_slice(),
        }
    }

    pub fn generate<G>(&mut self, generator: G)
    where
        G: Fn(usize, usize) -> pixel::Pixel,
    {
        self.buffer
            .iter_mut()
            .enumerate()
            .for_each(|(index, pixel)| *pixel = generator(index % self.stride, index / self.stride))
    }

    pub fn output<O, L>(&self, out: &mut O, log: &mut L) -> ::std::io::Result<()>
    where
        O: Write,
        L: Write,
    {
        let (width, height) = (self.width(), self.height());
        let total = width * height;

        out.write_fmt(format_args!("P3\n{width} {height}\n255\n"))
            .unwrap();

        let mut progress = progress::Progress::<20>::new();
        for (index, pixel) in self.pixels().iter().enumerate() {
            let completion = ((index as f64 / total as f64) * 20f64).trunc() as usize;

            if completion > progress.status {
                progress.advance();

                log.write(&progress.bar_cr())?;
                log.flush()?;
            }

            pixel.write_to(out)?;
        }

        log.write_fmt(format_args!("\r{:width$}", "Done", width = 20 + 2))?;

        Ok(())
    }

    pub fn width(&self) -> usize {
        self.stride
    }

    pub fn height(&self) -> usize {
        self.buffer.len() / self.stride
    }

    fn pixels(&self) -> &[pixel::Pixel] {
        &self.buffer
    }
}
