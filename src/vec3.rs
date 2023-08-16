#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub(crate) struct Vec3 {
    pub(crate) components: [f64; 3],
}

impl Vec3 {
    pub(crate) fn y(&self) -> f64 {
        self.components[1]
    }

    fn length_squared(&self) -> f64 {
        let [x, y, z] = self.components;

        x * x + y * y + z * z
    }

    fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub(crate) fn unit_vector(&self) -> Self {
        *self / self.length()
    }
}

pub(crate) type Point3 = Vec3;

impl From<(f64, f64, f64)> for Vec3 {
    fn from((x, y, z): (f64, f64, f64)) -> Self {
        Vec3 {
            components: [x, y, z],
        }
    }
}

impl ::core::ops::Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let [x1, y1, z1] = self.components;
        let [x2, y2, z2] = rhs.components;

        Vec3 {
            components: [x1 + x2, y1 + y2, z1 + z2],
        }
    }
}

impl ::core::ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let [x1, y1, z1] = self.components;
        let [x2, y2, z2] = rhs.components;

        Vec3 {
            components: [x1 - x2, y1 - y2, z1 - z2],
        }
    }
}

impl ::core::ops::Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self::Output {
        let [x, y, z] = self.components;

        Vec3 {
            components: [x * scalar, y * scalar, z * scalar],
        }
    }
}

impl ::core::ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, vec3: Vec3) -> Self::Output {
        let [x, y, z] = vec3.components;

        Vec3 {
            components: [x * self, y * self, z * self],
        }
    }
}

impl ::core::ops::Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, scalar: f64) -> Self::Output {
        let [x, y, z] = self.components;

        Vec3 {
            components: [x / scalar, y / scalar, z / scalar],
        }
    }
}
