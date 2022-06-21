const EPS: f64 = 1e-8;

#[derive(Clone, Copy)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    pub const fn empty() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub const fn get_x(&self) -> f64 {
        self.x
    }

    pub const fn get_y(&self) -> f64 {
        self.y
    }

    pub const fn get_z(&self) -> f64 {
        self.z
    }

    pub fn negative(&self) -> Self {
        Self::new(-self.x, -self.y, -self.z)
    }

    pub fn add(&self, other: &Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }

    pub fn subtract(&self, other: &Self) -> Self {
        self.add(&other.negative())
    }

    pub fn multiply(&self, other: &Self) -> Self {
        Self::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }

    pub fn add_constant(&self, value: f64) -> Self {
        Self::new(self.x + value, self.y + value, self.z + value)
    }

    pub fn multiply_constant(&self, value: f64) -> Self {
        Self::new(self.x * value, self.y * value, self.z * value)
    }

    pub fn divide_constant(&self, value: f64) -> Self {
        self.multiply_constant(1.0 / value)
    }

    pub fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn square(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f64 {
        self.square().sqrt()
    }

    pub fn unit_vector(&self) -> Self {
        self.divide_constant(self.length())
    }

    pub fn near_zero(&self) -> bool {
        self.x.abs() < EPS && self.y.abs() < EPS && self.z.abs() < EPS
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }
}

pub type Point3 = Vec3;
