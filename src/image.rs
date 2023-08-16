use crate::{
    const_generics::{Assert, True},
    pixel, progress,
};

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

    pub fn write_to_with_progress<W, L, const PROGRESS_LEN: usize>(
        &self,
        writer: &mut W,
        logger: &mut L,
    ) -> ::std::io::Result<()>
    where
        W: ::std::io::Write,
        L: ::std::io::Write,

        Assert<{ PROGRESS_LEN > 0 }>: True,
        [(); PROGRESS_LEN + 3]:,
    {
        let (width, height) = (self.width(), self.height());
        let total = width * height;

        writer.write_fmt(format_args!("P3\n{width} {height}\n255\n"))?;

        let mut progress = progress::Progress::<PROGRESS_LEN>::new();
        for (index, pixel) in self.pixels().iter().enumerate() {
            let completion = ((index as f64 / total as f64) * PROGRESS_LEN as f64).trunc() as usize;

            if completion > progress.status {
                progress.advance();

                logger.write(&progress.bar_cr())?;
                logger.flush()?;
            }

            pixel.write_to(writer)?;
        }

        logger.write_fmt(format_args!(
            "\r{:width$}",
            "Done",
            width = PROGRESS_LEN + 2
        ))?;

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
