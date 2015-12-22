use super::linalg::vector2d::*;
use super::common::vehicle::Vehicle;

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
pub struct FeelerVehicle {
    pub vehicle: Vehicle,
    pub length: f64,
    pub width: f64,
}

impl FeelerVehicle {
    //
    pub fn new(vehicle: Vehicle, length: f64, width: f64) -> FeelerVehicle {
        FeelerVehicle { vehicle: vehicle, length: length, width: width }
    }

    // Updates the matrices of the underlying vehicle.
    pub fn update(&mut self) {
        self.vehicle.update_matrices();
    }

    // Returns the interaction between this feeler and the given circle.
    pub fn intersection(&self, circle: &Circle) -> Option<Interaction> {
        let local_centre = self.vehicle.to_local.transform(circle.centre);
        if local_centre.y.abs() > circle.radius + self.width {
            return None;
        }

        let r2 = (circle.radius + self.width) * (circle.radius + self.width);
        let y2 = local_centre.y * local_centre.y;
        let x = -self.length * (local_centre.x + (r2 - y2).sqrt());
        let local_point = if x < 0f64 {
            Vec2D::zero()
        } else {
            Vec2D::new(x, 0f64)
        };
        Some(Interaction::new(local_point, local_centre, circle.radius))
    }

    // Returns a force intended to prevent collision between the vehicle and a
    // collection of circles.
    pub fn obstacle_avoidance(&self, circles: &Vec<Circle>) -> Option<Vec2D> {
        // Collect interactions between feeler and circles.
        let mut interactions = vec!();
        for circle in circles.iter() {
            match self.intersection(circle) {
                Some(x) => interactions.push(x),
                None => ()
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
        Some(self.vehicle.to_world.transform(Vec2D::new(force_x, force_y)))
    }
}
