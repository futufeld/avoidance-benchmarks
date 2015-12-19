use super::linalg::vector2d::*;
use super::linalg::matrix2d::*;

// Defines a circle.
pub struct Circle {
    pub centre: Vec2D,
    pub radius: f64
}

impl Circle {
    // Creates a circle from the given centre and radius.
    pub fn new(centre: Vec2D, radius: f64) -> Circle {
        Circle { centre: centre, radius: radius }
    }
}

// Defines a feeler and the space in which it exists.
pub struct Feeler {
    pub length: f64,
    pub width: f64,
    pub to_world: Mat2D,
    pub to_local: Mat2D
}

// Result of interaction between feeler and circle.
pub enum FeelerResult {
    Case1,
    Case2(Vec2D),
    Case3(Vec2D)
}

impl Feeler {
    // Creates a feeler with the given length and width at the given position
    // and orientation.
    pub fn new(pos: Vec2D, ori: f64, length: f64, width: f64) -> Feeler {
        let to_local = Mat2D::translation(pos.neg()).turn(-ori);
        let to_world = Mat2D::rotation(ori).shift(pos);

        Feeler { length: length
               , width: width
               , to_local: to_local
               , to_world: to_world }
    }

    // Returns the interaction between this feeler and the given circle.
    pub fn obstacle_intersections(&self, circle: &Circle) -> FeelerResult {
        // Determine if an intersection is occurring.
        let local_centre = self.to_local.transform(circle.centre);
        let in_x = local_centre.x >= 0f64 && local_centre.x <= self.length;
        let in_y = local_centre.y.abs() - circle.radius <= self.width;
        if !in_x || !in_y { return FeelerResult::Case1 };

        // If the circle is already inside the feeler volume, return the
        // nearest point on that circle to the origin of the feeler.
        if local_centre.y.abs() < self.width {
            let mut x = local_centre.x - circle.radius;
            if x < 0f64 { x = 0f64 };
            return FeelerResult::Case3(Vec2D::new(x, local_centre.y));
        }

        // The centre of the circle lies outside the feeler volume but
        // intersects it. Return the nearest point of intersection on the
        // longitudinal edges of the feeler.
        let r2 = circle.radius * circle.radius;
        let point = if local_centre.y > 0f64 {
            let y = self.width + local_centre.y;
            let x = -self.length * (local_centre.x + (r2 - (y*y)).sqrt());
            Vec2D::new(x, self.width)
        } else {
            let y = -self.width - local_centre.y;
            let x = -self.length * (local_centre.x + (r2 - (y*y)).sqrt());
            Vec2D::new(x, -self.width)
        };
        FeelerResult::Case2(point)
    }
}
