#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub(crate) struct Pixel {
    pub(crate) inner: Color,
}

pub(crate) type Color = crate::Vec3;

impl Pixel {
    pub(crate) fn write_to<W>(self, writer: &mut W) -> ::std::io::Result<()>
    where
        W: ::std::io::Write,
    {
        let Color {
            components: [r, g, b],
        } = self.inner;

        let r = (r * 255f64) as u8;
        let g = (g * 255f64) as u8;
        let b = (b * 255f64) as u8;

        writer.write_fmt(format_args!("{r} {g} {b}\n"))?;

        Ok(())
    }
}

impl From<(f64, f64, f64)> for Pixel {
    fn from((r, g, b): (f64, f64, f64)) -> Self {
        Pixel {
            inner: Color {
                components: [r, g, b],
            },
        }
    }
}

impl From<Color> for Pixel {
    fn from(color: Color) -> Self {
        Pixel { inner: color }
    }
}
