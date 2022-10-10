#[derive(Clone, Debug, PartialEq)]
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

    pub fn inner_product(&self, v: &Self) -> f64 {
        self.x * v.x + self.y * v.y + self.z * v.z
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

#[derive(Clone, Debug, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec3_tests() {
        let v1 = Vec3 {
            x: 1.,
            y: 2.,
            z: 3.,
        };
        let v2 = Vec3 {
            x: 2.,
            y: 3.,
            z: 1.,
        };
        assert_eq!(
            Vec3 {
                x: 3.,
                y: 5.,
                z: 4.,
            },
            v1.add(&v2)
        );
        assert_eq!(11., v1.inner_product(&v2));

        let v3 = Vec3 {
            x: 3.,
            y: 4.,
            z: 0.,
        };
        assert_eq!(
            Vec3 {
                x: 4.5,
                y: 6.,
                z: 0.
            },
            v3.scale(1.5)
        );
        assert_eq!(
            Vec3 {
                x: 1.5,
                y: 2.,
                z: 0.,
            },
            v3.divide(2.)
        );
        assert_eq!(25., v3.length_squared());
        assert_eq!(5., v3.length());
        assert_eq!(
            Vec3 {
                x: 0.6,
                y: 0.8,
                z: 0.,
            },
            v3.unit_vector().inject()
        )
    }

    #[test]
    fn point3_tests() {
        let p1 = Point3 {
            x: 1.,
            y: 2.,
            z: 3.,
        };
        let p2 = Point3 {
            x: 2.,
            y: 3.,
            z: 1.,
        };
        assert_eq!(
            Vec3 {
                x: -1.,
                y: -1.,
                z: 2.,
            },
            p1.subtract(&p2)
        );

        let v = Vec3 {
            x: 13.,
            y: 24.,
            z: 30.,
        };
        assert_eq!(
            Point3 {
                x: 14.,
                y: 26.,
                z: 33.,
            },
            p1.add(&v)
        );
    }
}
