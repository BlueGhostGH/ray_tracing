use crate::vec3;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Pixel {
    pub(crate) inner: vec3::Vec3<f64>,
}

impl Pixel {
    pub(crate) fn write_to<W>(self, writer: &mut W) -> ::std::io::Result<()>
    where
        W: ::std::io::Write,
    {
        let vec3::Vec3 {
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
            inner: vec3::Vec3 {
                components: [r, g, b],
            },
        }
    }
}
