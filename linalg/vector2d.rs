use std::fmt;

pub const EPSILON: f64 = 1e-8;

#[derive(Copy, Clone)]
pub struct Vec2D { pub x: f64
                 , pub y: f64 }

impl fmt::Display for Vec2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Vec2D {
    pub fn new(x: f64, y: f64) -> Vec2D {
        Vec2D { x: x, y: y }
    }

    pub fn zero() -> Vec2D { Vec2D::new(0f64, 0f64) }

    pub fn unitx() -> Vec2D { Vec2D::new(1f64, 0f64) }

    pub fn unity() -> Vec2D { Vec2D::new(0f64, 1f64) }

    pub fn polar(angle: f64, mag: f64) -> Vec2D {
        Vec2D { x: angle.cos(), y: angle.sin() }.mul(mag)
    }

    pub fn copy(&self) -> Vec2D {
        Vec2D { x: self.x, y: self.y }
    }

    pub fn neg(&self) -> Vec2D {
        Vec2D { x: -self.x, y: -self.y }
    }

    pub fn add(&self, v: Vec2D) -> Vec2D {
        Vec2D { x: self.x + v.x, y: self.y + v.y }
    }

    pub fn sub(&self, v: Vec2D) -> Vec2D {
        Vec2D { x: self.x - v.x, y: self.y - v.y }
    }

    pub fn mul(&self, s: f64) -> Vec2D {
        Vec2D { x: self.x * s, y: self.y * s }
    }

    pub fn mag_sq(&self) -> f64 {
        self.x * self.x + self.y * self.y
    }

    pub fn mag(&self) -> f64 {
        let mag = self.mag_sq();
        if mag < EPSILON { mag } else { mag.sqrt() }
    }

    pub fn dot(&self, v: Vec2D) -> f64 {
        self.x * v.x + self.y * v.y
    }

    pub fn perp(&self) -> Vec2D {
        Vec2D { x: -self.y, y: self.x }
    }

    pub fn angle(&self) -> f64 {
        self.y.atan2(self.x)
    }
}
