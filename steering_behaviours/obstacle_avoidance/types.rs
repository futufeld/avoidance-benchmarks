use super::linalg::vector2d::*;
use super::linalg::matrix2d::*;

use std::cmp::Ordering::Equal;

// Weighting factor for obstacle avoidance steering force.
const BRAKING_WEIGHT: f64 = 2f64;

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

// Result of interaction between feeler and circle.
pub enum FeelerResult {
    Case1,
    Case2(Interaction),
    Case3(Interaction)
}

// Pairs a circle with a point. Used to capture interactions between the
// feeler and circles.
pub struct Interaction {
    point: Vec2D,
    centre: Vec2D,
    radius: f64
}

impl Interaction {
    // Creates an interaction containing the provided values.
    fn new(point: Vec2D, centre: Vec2D, radius: f64) -> Interaction {
        Interaction { point: point, centre: centre, radius: radius }
    }
}

// Defines a feeler and the space in which it exists.
pub struct Feeler {
    pub position: Vec2D,
    pub orientation: f64,
    pub length: f64,
    pub width: f64,
    pub to_world: Mat2D,
    pub to_local: Mat2D
}

impl Feeler {
    // Creates a feeler with the given length and width at the given position
    // and orientation.
    pub fn new(pos: Vec2D, ori: f64, length: f64, width: f64) -> Feeler {
        let to_local = Mat2D::translation(pos.neg()).turn(-ori);
        let to_world = Mat2D::rotation(ori).shift(pos);
        Feeler { position: pos
               , orientation: ori
               , length: length
               , width: width
               , to_local: to_local
               , to_world: to_world }
    }

    // Updates the matrices that transform in and out of the feeler's space.
    pub fn update_matrices(&mut self) {
        let to_local = Mat2D::identity().shift(self.position.neg())
                                        .turn(-self.orientation);
        let to_world = Mat2D::identity().turn(self.orientation)
                                        .shift(self.position);
        self.to_local = to_local;
        self.to_world = to_world;
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
            let point = Vec2D::new(x, local_centre.y);
            let int = Interaction::new(point, local_centre, circle.radius);
            return FeelerResult::Case3(int);
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
        let int = Interaction::new(point, local_centre, circle.radius);
        FeelerResult::Case2(int)
    }

    //
    pub fn steering(&self, circles: &Vec<Circle>) -> Option<Vec2D> {
        // Collect interactions between feeler and circles.
        let mut interactions = vec!();
        for circle in circles.iter() {
            match self.obstacle_intersections(circle) {
                FeelerResult::Case1 => (),
                FeelerResult::Case2(x) => interactions.push(x),
                FeelerResult::Case3(x) => interactions.push(x)
            }
        }

        // Get the nearest interaction.
        if interactions.len() == 0 { return None };
        interactions.sort_by(|a, b| {
            let y1 = a.point.y;
            let y2 = b.point.y;
            y1.partial_cmp(&y2).unwrap_or(Equal)
        });
        let near = &interactions[0];

        // Determine steering force.
        let multiplier = 1f64 + (self.length - near.centre.x) / self.length;
        let force_x = (near.radius - near.centre.x) * BRAKING_WEIGHT;
        let force_y = (near.radius - near.centre.y) * multiplier;
        Some(self.to_world.transform(Vec2D::new(force_x, force_y)))
    }
}
