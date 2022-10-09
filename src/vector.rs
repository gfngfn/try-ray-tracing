pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
impl Vec3 {
    #[allow(dead_code)]
    pub fn add(self, v: Self) -> Self {
        Vec3 {
            x: self.x + v.x,
            y: self.y + v.y,
            z: self.z + v.z,
        }
    }

    #[allow(dead_code)]
    pub fn length_squared(self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    #[allow(dead_code)]
    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }
}
