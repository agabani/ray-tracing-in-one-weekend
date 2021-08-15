use crate::number::random_f64;

#[derive(Clone, Copy)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
    }

    pub fn length_squared(&self) -> f64 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn unit_vector(&self) -> Self {
        self / self.length()
    }

    pub fn dot(&self, v: &Vec3) -> f64 {
        self.x() * v.x() + self.y() * v.y() + self.z() * v.z()
    }

    pub fn random(min_max: Option<(f64, f64)>) -> Self {
        Self::new(
            random_f64(min_max),
            random_f64(min_max),
            random_f64(min_max),
        )
    }

    pub fn random_in_unit_sphere() -> Self {
        loop {
            let point = Self::random(Some((-1.0, 1.0)));
            if point.length_squared() < 1.0 {
                return point;
            }
        }
    }

    pub fn random_unit_vector() -> Self {
        Self::random_in_unit_sphere().unit_vector()
    }

    pub fn random_in_unit_hemisphere(normal: &Self) -> Self {
        let in_unit_sphere = Self::random_in_unit_sphere();
        if in_unit_sphere.dot(normal) > 0.0 {
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }

    pub fn near_zero(&self) -> bool {
        let epsilon = 1e-8;
        self.x.abs() < epsilon && self.y.abs() < epsilon && self.z.abs() < epsilon
    }

    pub fn reflect(&self, normal: &Self) -> Self {
        self - 2.0 * self.dot(normal) * normal
    }
}

impl std::ops::Add for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl std::ops::Add<Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        self + &rhs
    }
}

impl std::ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        &self + &rhs
    }
}

impl std::ops::Div<f64> for &Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        Self::Output {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl std::ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        &self / rhs
    }
}

impl std::ops::Mul<&Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: &Vec3) -> Self::Output {
        Self::Output {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl std::ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        self * &rhs
    }
}

impl std::ops::Neg for &Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Self::Output {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl std::ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        -&self
    }
}

impl std::ops::Sub for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl std::ops::Sub<Vec3> for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        self - &rhs
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        &self - &rhs
    }
}

impl std::ops::Sub<&Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: &Vec3) -> Self::Output {
        &self - rhs
    }
}
