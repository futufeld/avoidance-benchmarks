use super::linalg::vector2d::*;
use super::common::types::Frame;

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

// Pairs a circle with a distance. Used to capture interactions between vehicle
// feeler and circles.
#[derive(Copy, Clone)]
pub struct Interaction {
    dist: f64,
    centre: Vec2D,
    radius: f64
}

impl Interaction {
    // Creates an interaction containing the provided values.
    fn new(distance: f64, centre: Vec2D, radius: f64) -> Interaction {
        Interaction { dist: distance, centre: centre, radius: radius }
    }
}

// Defines a vehicle with a single feeler volume.
pub struct Vehicle {
    pub frame: Frame,
    pub length: f64,
    pub width: f64,
}

impl Vehicle {
    // Creates a new vehicle using the given values.
    pub fn new(frame: Frame, length: f64, width: f64) -> Vehicle {
        Vehicle { frame: frame, length: length, width: width }
    }

    // Updates the matrices of the underlying frame.
    pub fn update(&mut self) {
        self.frame.update_matrices();
    }

    // Returns the interaction between the vehicle's feeler and the given
    // circle.
    pub fn interaction(&self, circle: &Circle) -> Option<Interaction> {
        let local_centre = self.frame.to_local.transform(circle.centre);
        if local_centre.y.abs() > circle.radius + self.width {
            return None;
        }

        let r2 = (circle.radius + self.width) * (circle.radius + self.width);
        let y2 = local_centre.y * local_centre.y;
        let mut x = -self.length * (local_centre.x + (r2 - y2).sqrt());
        if x < 0f64 { x = 0f64 };
        Some(Interaction::new(x, local_centre, circle.radius))
    }

    // Returns a force intended to prevent collision between the vehicle and a
    // collection of circles.
    pub fn obstacle_avoidance(&self, circles: &Vec<Circle>) -> Option<Vec2D> {

        // Collect interactions between vehicle's feeler and circles.
        let mut nearest: Option<Interaction> = None;
        for circle in circles.iter() {

            // Check if interaction is closer than known nearest.
            let interaction = self.interaction(circle);
            if let Some(int) = interaction {
                if let Some(near) = nearest {
                    if int.dist < near.dist { nearest = interaction }
                } else {
                    nearest = interaction
                }
            }
        }
        if nearest.is_none() { return None };
        let near: Interaction = nearest.unwrap();

        // Determine steering force.
        let multiplier = 1f64 + (self.length - near.centre.x) / self.length;
        let force_x = (near.radius - near.centre.x) * BRAKING_WEIGHT;
        let force_y = (near.radius - near.centre.y) * multiplier;
        Some(self.frame.to_world.transform(Vec2D::new(force_x, force_y)))
    }
}
