// All fields are in [0, 1].
#[derive(Clone, Debug, PartialEq)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}
impl Color {
    pub fn write(&self) {
        let ir = (255.999 * self.r) as u8;
        let ig = (255.999 * self.g) as u8;
        let ib = (255.999 * self.b) as u8;
        println!("{} {} {}", ir, ig, ib);
    }

    pub fn blend(&self, t: f64, other: &Self) -> Self {
        Self {
            r: (1. - t) * self.r + t * other.r,
            g: (1. - t) * self.g + t * other.g,
            b: (1. - t) * self.b + t * other.b,
        }
    }

    #[allow(dead_code)]
    pub fn scale(&self, t: f64) -> Self {
        Self {
            r: self.r * t,
            g: self.g * t,
            b: self.b * t,
        }
    }

    pub fn attenuate(&self, attenuation: &Attenuation) -> Self {
        Self {
            r: self.r * attenuation.r,
            g: self.g * attenuation.g,
            b: self.b * attenuation.b,
        }
    }

    pub fn average(colors: &Vec<Self>) -> Self {
        let mut r: f64 = 0.;
        let mut g: f64 = 0.;
        let mut b: f64 = 0.;
        for color in colors.iter() {
            r += color.r;
            g += color.g;
            b += color.b;
        }
        let num = colors.len() as f64;
        Self {
            r: r / num,
            g: g / num,
            b: b / num,
        }
    }
}

// All fields are in [0, 1].
#[derive(Clone, Debug, PartialEq)]
pub struct Attenuation {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}
