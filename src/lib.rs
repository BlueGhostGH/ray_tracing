#![feature(generic_const_exprs, get_many_mut)]
#![allow(incomplete_features)]

use ::std::io::Write;

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct Image {
    stride: usize,
    buffer: Box<[Pixel]>,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Image {
    pub fn new(width: usize, height: usize) -> Self {
        let mut buffer = Vec::with_capacity(width * height);
        buffer.resize_with(buffer.capacity(), Pixel::default);

        Image {
            stride: width,
            buffer: buffer.into_boxed_slice(),
        }
    }

    pub fn generate<G>(&mut self, generator: G)
    where
        G: Fn(usize, usize) -> Pixel,
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

            let Pixel { r, g, b } = pixel;
            out.write_fmt(format_args!("{r} {g} {b}\n")).unwrap();
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

    fn pixels(&self) -> &[Pixel] {
        &self.buffer
    }
}

mod progress {
    use crate::const_generics::{Assert, True};

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub(crate) struct Progress<const N: usize> {
        buffer: [u8; N],
        pub(crate) status: usize,
    }

    impl<const N: usize> Progress<N>
    where
        Assert<{ N > 0 }>: True,
        [(); N + 3]: Sized,
    {
        pub(crate) fn new() -> Self {
            let buffer = [b' '; N];

            Progress { buffer, status: 0 }
        }

        pub(crate) fn advance(&mut self) {
            if self.status == 0 {
                unsafe {
                    *self.buffer.get_unchecked_mut(self.status) = b'>';
                }
                self.status += 1;
            } else if self.status == N {
                unsafe {
                    *self.buffer.get_unchecked_mut(self.status - 1) = b'=';
                }
            } else {
                unsafe {
                    *self.buffer.get_unchecked_mut(self.status) = b'>';
                    *self.buffer.get_unchecked_mut(self.status - 1) = b'=';
                    self.status += 1;
                }
            }
        }

        pub(crate) fn bar_cr(&self) -> [u8; N + 3] {
            let bar = unsafe {
                let mut bar = [0; N + 3];

                let [cr, opening, closing] = bar.get_many_unchecked_mut([0, 1, N + 2]);
                *cr = b'\r';
                *opening = b'[';
                *closing = b']';

                (&mut bar[2..N + 2]).clone_from_slice(&self.buffer);

                bar
            };

            bar
        }
    }
}

mod const_generics {
    pub(crate) struct Assert<const CHECK: bool>;

    pub(crate) trait True {}

    impl True for Assert<true> {}
}
