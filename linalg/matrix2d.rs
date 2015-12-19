use std::f64;

use super::vector2d::Vec2D;

pub struct Mat2D {
    m: [[f64; 3]; 3]
}

impl Mat2D {
    pub fn identity() -> Mat2D {
        Mat2D { m: [ [ 1f64, 0f64, 0f64]
                   , [ 0f64, 1f64, 0f64]
                   , [ 0f64, 0f64, 1f64] ] }
    }

    pub fn rotation(rad: f64) -> Mat2D {
        let c = f64::cos(rad);
        let s = f64::sin(rad);
        let mut result = Mat2D::identity();
        result.m[0][0] = c;
        result.m[1][0] = -s;
        result.m[0][1] = s;
        result.m[1][1] = c;
        result
    }

    pub fn scalation(v: Vec2D) -> Mat2D {
        let mut result = Mat2D::identity();
        result.m[0][0] = v.x;
        result.m[1][1] = v.y;
        result
    }

    pub fn translation(v: Vec2D) -> Mat2D {
        let mut result = Mat2D::identity();
        result.m[2][0] = v.x;
        result.m[2][1] = v.y;
        result
    }

    pub fn turn(&self, rad: f64) -> Mat2D {
        self.mul(Mat2D::rotation(rad))
    }

    pub fn scale(&self, v: Vec2D) -> Mat2D {
        self.mul(Mat2D::scalation(v))
    }

    pub fn shift(&self, v: Vec2D) -> Mat2D {
        self.mul(Mat2D::translation(v))
    }

    pub fn mul(&self, m: Mat2D) -> Mat2D {
        let mut result = Mat2D::identity();
        for i in 0..3 {
            for j in 0..3 {
                let mut sum = 0f64;
                for k in 0..3 {
                    sum += self.m[i][k] * m.m[k][j];
                }
                result.m[i][j] = sum;
            }
        }
        result
    }

    pub fn transform(&self, v: Vec2D) -> Vec2D {
        let x = self.m[0][0] * v.x + self.m[1][0] * v.y + self.m[2][0];
        let y = self.m[0][1] * v.x + self.m[1][1] * v.y + self.m[2][1];
        Vec2D { x: x, y: y }
    }
}
