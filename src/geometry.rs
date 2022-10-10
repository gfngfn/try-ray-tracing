#[derive(Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
impl Vec3 {
    #[allow(dead_code)]
    pub fn add(&self, v: &Self) -> Self {
        Vec3 {
            x: self.x + v.x,
            y: self.y + v.y,
            z: self.z + v.z,
        }
    }

    pub fn scale(&self, ratio: f64) -> Self {
        Vec3 {
            x: self.x * ratio,
            y: self.y * ratio,
            z: self.z * ratio,
        }
    }

    pub fn divide(&self, d: f64) -> Self {
        Vec3 {
            x: self.x / d,
            y: self.y / d,
            z: self.z / d,
        }
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn unit_vector(&self) -> UnitVec3 {
        UnitVec3::new(self)
    }
}

/// The type for representing 3D unit vectors (i.e. 3D vectors with their length 1)
#[derive(Clone)]
pub struct UnitVec3 {
    x: f64,
    y: f64,
    z: f64,
}
impl UnitVec3 {
    pub fn new(v: &Vec3) -> Self {
        let w = v.divide(v.length());
        Self {
            x: w.x,
            y: w.y,
            z: w.z,
        }
    }

    pub fn inject(&self) -> Vec3 {
        Vec3 {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}

#[derive(Clone)]
pub struct Point3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
impl Point3 {
    pub fn add(&self, v: &Vec3) -> Self {
        Point3 {
            x: self.x + v.x,
            y: self.y + v.y,
            z: self.z + v.z,
        }
    }

    pub fn subtract(&self, pt: &Point3) -> Vec3 {
        Vec3 {
            x: self.x - pt.x,
            y: self.y - pt.y,
            z: self.z - pt.z,
        }
    }
}

#[derive(Clone)]
pub struct Ray {
    pub origin: Point3,
    pub direction: UnitVec3,
}
impl Ray {
    #[allow(dead_code)]
    pub fn at(&self, t: f64) -> Point3 {
        self.origin.add(&self.direction.inject().scale(t))
    }
}
