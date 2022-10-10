// All fields are in [0, 1).
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

    #[allow(dead_code)]
    pub fn blend(&self, t: f64, other: &Self) -> Self {
        Self {
            r: (1. - t) * self.r + t * other.r,
            g: (1. - t) * self.g + t * other.g,
            b: (1. - t) * self.b + t * other.b,
        }
    }
}
